use nanoserde::{DeJson, SerJson, DeRon, SerRon};

use crate::{serialized::OptStr, kayak::kstyle::skstyle::SKStyle};

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct SClipBundle {
    pub clip: OptStr,
    pub styles: Option<SKStyle>,
    pub name: String,
}
