use nanoserde::{DeJson, SerJson, DeRon, SerRon};

use crate::kayak::{kstyle::skstyle::SKStyle, kbutton::skbutton::SButton};

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct SButtonBundle {
    pub button: Option<SButton>,
    pub styles: Option<SKStyle>,
    // pub on_event: OnEvent,
    pub name: String,
}
