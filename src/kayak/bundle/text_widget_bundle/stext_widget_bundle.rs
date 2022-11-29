use nanoserde::{DeJson, SerJson, DeRon, SerRon};

use crate::kayak::{text_props::stext_props::STextProps, kstyle::skstyle::SKStyle};

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct STextWidgetBundle {
    pub name: String,
    pub text: Option<STextProps>,
    pub styles: Option<SKStyle>,
}
