use nanoserde::{DeJson, SerJson, DeRon, SerRon};

use crate::{serialized::OptStr, kayak::kstyle::skstyle::SKStyle};

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct SBackgroundBundle {
    pub background: OptStr,
    pub styles: Option<SKStyle>,
    pub name: String,
}
