use nanoserde::{DeJson, SerJson, DeRon, SerRon};

use crate::kayak::{window::swindow::SWindow, kstyle::skstyle::SKStyle};

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct SWindowBundle {
    pub window: Option<SWindow>,
    pub styles: Option<SKStyle>,
    // pub children: SChildren,
    pub name: String
}
