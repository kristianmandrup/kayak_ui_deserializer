use nanoserde::{DeJson, SerJson, DeRon, SerRon};

use crate::serialized::OptStr;

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct SSize {
    /// The width of the 2-dimensional area.
    pub width: OptStr,
    /// The height of the 2-dimensional area.
    pub height: OptStr,
}
