use nanoserde::{DeJson, SerJson, DeRon, SerRon};

use crate::{bevy::image::simage::SImage, serialized::OptStr, kayak::edge::sedge::SEdge};
#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct SNinePatch {
    pub image: Option<SImage>,
    pub border: OptStr,
    pub border_obj: Option<SEdge>,
}
