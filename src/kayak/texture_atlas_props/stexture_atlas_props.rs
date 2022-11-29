use nanoserde::{DeJson, SerJson, DeRon, SerRon};

use crate::{serialized::OptStr, bevy::image::simage::SImage};

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct STextureAtlasProps {
    /// The handle to image
    pub handle: Option<SImage>,
    /// The position of the tile (in pixels)
    pub position: Option<Vec<OptStr>>,
    /// The size of the tile (in pixels)
    pub tile_size: Option<Vec<OptStr>>,
}
