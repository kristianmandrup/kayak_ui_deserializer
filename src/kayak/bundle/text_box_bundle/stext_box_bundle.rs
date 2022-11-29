use nanoserde::{DeJson, SerJson, DeRon, SerRon};
use crate::{serialized::OptStr, kayak::{kstyle::skstyle::SKStyle, text_box_props::stext_box_props::STextBoxProps}};

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct STextBoxBundle {
    pub text_box: Option<STextBoxProps>,
    pub styles: Option<SKStyle>,
    // pub on_event: OnEvent,
    // pub on_layout: OnLayout,
    // pub on_change: OnChange,
    pub focusable: OptStr,
    pub name: String,
}
