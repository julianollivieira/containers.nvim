mod providers;

use nvim_oxi::{
    Dictionary, Function, Object,
    api::{
        Window,
        types::{SplitDirection, WindowConfig},
    },
};
use providers::Provider;

#[nvim_oxi::plugin]
fn nvim_docker() -> anyhow::Result<Dictionary> {
    let open = Function::from_fn(|_: usize| {
        let mut window_config = WindowConfig::default();
        window_config.split = Some(SplitDirection::Right);
        window_config.win = Some(Window::current());
        window_config.width = Some(80);

        let containers = providers::docker::Docker::get_containers().unwrap();

        let lines: Vec<String> = containers.iter().fold(Vec::new(), |mut acc, c| {
            acc.extend_from_slice(&[format!("- {}", c.id)]);

            acc
        });

        let mut new_buf = nvim_oxi::api::create_buf(false, true).unwrap();
        let window = nvim_oxi::api::open_win(&new_buf, false, &window_config).unwrap();
        window
            .get_buf()
            .unwrap()
            .set_text(0..0, 0, 0, lines)
            .unwrap();

        new_buf.set_option("modifiable", false).unwrap();
    });

    Ok(Dictionary::from_iter([("open", Object::from(open))]))
}
