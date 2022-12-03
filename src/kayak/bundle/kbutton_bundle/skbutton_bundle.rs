use nanoserde::{DeJson, SerJson, DeRon, SerRon};

use crate::kayak::{kstyle::skstyle::SKStyle, kbutton::skbutton::SKButton};

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct SKButtonBundle {
    pub button: Option<SKButton>,
    pub styles: Option<SKStyle>,
    pub computed_styles: Option<SKStyle>,
    // pub on_event: OnEvent,
    pub name: String,
}
