use bevy::prelude::Vec2;
use kayak_ui::{widgets::{TextureAtlasProps}};

use crate::{ui_parser::Conv};

use super::stexture_atlas_props::STextureAtlasProps;

pub fn deserialize_texture_atlas_props(tab: STextureAtlasProps) -> Result<TextureAtlasProps, &'static str>  {
    TextureAtlasPropsDeser::new(tab).deserialize()
}

pub struct TextureAtlasPropsDeser {
    node: STextureAtlasProps,
}
impl TextureAtlasPropsDeser {
    pub fn new(node: STextureAtlasProps) -> Self {
        Self {
            node
        }
    }

    fn to_f32(&self, prop: &Option<String>) -> Option<f32> {
        if let Some(str) = Conv::get_prop(prop) {
            Conv(str).to_f32()
        } else {
            None
        }                    
    }

    fn to_vec2(&self, prop: &Option<Vec<Option<String>>>) -> Option<Vec2> {
        if let Some(vec) = prop.to_owned() {
            let optx = self.to_f32(&vec[0]);
            let opty = self.to_f32(&vec[1]);
            let xy = (optx, opty);
            if let (Some(x), Some(y)) = xy {
                Some(Vec2::new(x, y))
            } else {
                None
            }            
        } else {
            None
        }
    }

    fn position(&self) -> Option<Vec2> {
        let prop = &self.node.position.clone();
        self.to_vec2(prop)
    }

    fn tile_size(&self) -> Option<Vec2> {
        let prop = &self.node.tile_size.clone();
        self.to_vec2(prop)
    }


    pub fn deserialize(&self) -> Result<TextureAtlasProps, &'static str> {        
        let position = self.position();
        let tile_size = self.tile_size();
        let mut tap = TextureAtlasProps::default();
        if let Some(val) = position {
            tap.position = val;    
        }
        if let Some(val) = tile_size {
            tap.tile_size = val;    
        }
        Ok(tap)
    }    
}



