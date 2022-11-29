use nanoserde::{DeJson, SerJson, DeRon, SerRon};

use crate::{serialized::OptStr, kayak::kstyle::skstyle::SKStyle};

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct SElementBundle {
    pub element: OptStr,
    pub styles: Option<SKStyle>,
    // pub children: SChildren,
    pub name: String
}
