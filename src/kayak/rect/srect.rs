use nanoserde::{DeJson, SerJson, DeRon, SerRon};
use crate::{serialized::OptStr};

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct SRect {
    pub posy: OptStr,
    pub posx: OptStr,
    pub width: OptStr,
    pub height: OptStr,
    pub z_index: OptStr,
}
