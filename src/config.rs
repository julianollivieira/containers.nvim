use std::str::FromStr;

use nvim_oxi::{
    Object,
    conversion::{Error as ConversionError, FromObject},
    lua,
    serde::Deserializer,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub windows: Option<WindowConfig>,
}

impl FromObject for Config {
    fn from_object(obj: Object) -> Result<Self, ConversionError> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}

impl lua::Poppable for Config {
    unsafe fn pop(lstate: *mut lua::ffi::lua_State) -> Result<Self, lua::Error> {
        let obj = Object::pop(lstate)?;
        Self::from_object(obj).map_err(lua::Error::pop_error_from_err::<Self, _>)
    }
}

#[derive(Debug, Deserialize)]
pub struct WindowConfig {
    #[serde(default)]
    pub list: Option<WindowPosition>,
    #[serde(default)]
    pub logs: Option<WindowPosition>,
    #[serde(default)]
    pub shell: Option<WindowPosition>,
}

#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WindowPosition {
    SplitRight,
    SplitLeft,
    SplitTop,
    SplitBottom,
    Floating,
    Tab,
    CurrentBuffer,
}

impl FromStr for WindowPosition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "split-right" => Ok(WindowPosition::SplitRight),
            "split-left" => Ok(WindowPosition::SplitLeft),
            "split-top" => Ok(WindowPosition::SplitTop),
            "split-bottom" => Ok(WindowPosition::SplitBottom),
            "floating" => Ok(WindowPosition::Floating),
            "tab" => Ok(WindowPosition::Tab),
            "current-buffer" => Ok(WindowPosition::CurrentBuffer),
            _ => Err(()),
        }
    }
}
