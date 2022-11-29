use nanoserde::{DeJson, SerJson, DeRon, SerRon};

use crate::serialized::OptStr;

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct SUiRect {
    pub left: OptStr,
    /// The value corresponding to the right side of the UI rect.
    pub right: OptStr,
    /// The value corresponding to the top side of the UI rect.
    pub top: OptStr,
    /// The value corresponding to the bottom side of the UI rect.
    pub bottom: OptStr,
}
