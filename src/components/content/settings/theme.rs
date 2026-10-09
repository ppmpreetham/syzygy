use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};
use super::method::method;

// radio
#[derive(Deserialize, Serialize, EnumIter, Display)]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}
impl ThemeMode {
    pub fn method() -> method {
        method::Radio(Self::iter().map(|v| v.to_string()).collect())
    }
}

// dropdown
#[derive(Deserialize, Serialize)]
pub enum Theme {
    Monokai,
    Catppuccin,
    Dracula,
    Nord,
}

// list
