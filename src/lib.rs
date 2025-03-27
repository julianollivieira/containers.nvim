mod config;
mod providers;
mod ui;

use std::sync::{OnceLock, RwLock};

use config::Config;
use nvim_oxi::{
    Dictionary, Function,
    api::{
        Window,
        types::{SplitDirection, WindowConfig},
    },
};
use providers::Provider;

pub static CONFIG: OnceLock<Config> = OnceLock::new();
static ACTIVE_WINDOW_ID: OnceLock<RwLock<Option<Window>>> = OnceLock::new();

#[nvim_oxi::plugin]
fn containers() -> anyhow::Result<Dictionary> {
    let mut dict = Dictionary::new();

    dict.insert("setup", Function::from_fn(setup));
    dict.insert("open", Function::from_fn(open));
    dict.insert("close", Function::from_fn(close));
    dict.insert("toggle", Function::from_fn(toggle));

    Ok(dict)
}

/// Opens the container list window if it is not open yet.
fn open(_: ()) {
    let active_window = ACTIVE_WINDOW_ID.get_or_init(|| RwLock::new(None));
    if active_window.read().unwrap().is_some() {
        return;
    }

    let mut window_config = WindowConfig::default();
    window_config.split = Some(SplitDirection::Right);
    window_config.win = Some(Window::current());
    window_config.width = Some(80);

    let containers = providers::docker::Docker::get_containers().unwrap();

    let mut new_buf = nvim_oxi::api::create_buf(false, true).unwrap();
    let window = nvim_oxi::api::open_win(&new_buf, false, &window_config).unwrap();

    *active_window.write().unwrap() = Some(window.clone());

    window
        .get_buf()
        .unwrap()
        .set_text(0..0, 0, 0, ui::draw_container_list(containers))
        .unwrap();

    new_buf.set_option("modifiable", false).unwrap();
}

/// Closes the container list window if it is open.
fn close(_: ()) {
    let window = ACTIVE_WINDOW_ID.get().unwrap().read().unwrap().clone();
    if let Some(window) = window {
        window.close(false).unwrap();
        *ACTIVE_WINDOW_ID.get().unwrap().write().unwrap() = None;
    }
}

/// Toggles the container list window.
fn toggle(_: ()) {
    if ACTIVE_WINDOW_ID
        .get_or_init(|| RwLock::new(None))
        .read()
        .unwrap()
        .is_none()
    {
        open(());
    } else {
        close(());
    }
}

fn setup(config: Config) {
    CONFIG.set(config).unwrap();
}
