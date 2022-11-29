use nanoserde::{DeJson, SerJson, DeRon, SerRon};
use crate::serialized::OptStr;

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct SCorner {
    pub top_left: OptStr,
    pub top_right: OptStr,
    pub bottom_left: OptStr,
    pub bottom_right: OptStr,
}
