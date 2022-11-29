use nanoserde::{DeJson, SerJson, DeRon, SerRon};

use crate::kayak::{kstyle::skstyle::SKStyle, texture_atlas_props::stexture_atlas_props::STextureAtlasProps};

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct STextureAtlasBundle {
    pub atlas: Option<STextureAtlasProps>,
    pub styles: Option<SKStyle>,
    pub name: String,
}
