use nanoserde::{DeJson, SerJson, DeRon, SerRon};

use crate::serialized::OptStr;

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct SEdge {
    pub top: OptStr,
    pub left: OptStr,
    pub right: OptStr,
    pub bottom: OptStr,
    pub all: OptStr,
}
