use nanoserde::{DeJson, SerJson, DeRon, SerRon};

use crate::kayak::kstyle::skstyle::SKStyle;

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct SButton {
    pub name: String,
    pub styles: Option<SKStyle>,
}
