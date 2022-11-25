use bevy::prelude::Vec2;
use kayak_ui::{widgets::TextureAtlasProps, prelude::KStyle};

use crate::{json_deserializer::{STextureAtlasBundle, STextureAtlasProps}, ui_style::StyleBuilder, ui_parser::Conv};

// pub struct TextureAtlasProps {
//     /// The handle to image
//     pub handle: Handle<Image>,
//     /// The position of the tile (in pixels)
//     pub position: Vec2,
//     /// The size of the tile (in pixels)
//     pub tile_size: Vec2,
// }
pub struct TextureAtlasPropsBuilder {
    node: STextureAtlasProps,
}
impl TextureAtlasPropsBuilder {
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


    pub fn parse(&self) -> Result<TextureAtlasProps, &'static str> {        
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

// impl Widget for TextureAtlasProps {}

// /// A widget that renders a bevy texture atlas
// #[derive(Bundle)]
// pub struct TextureAtlasBundle {
//     pub atlas: TextureAtlasProps,
//     pub styles: KStyle,
//     pub widget_name: WidgetName,
// }
pub struct TextureAtlasBundleBuilder {
    node: STextureAtlasBundle,
}
impl TextureAtlasBundleBuilder {
    pub fn new(node: STextureAtlasBundle) -> Self {
        Self {
            node
        }
    }

    fn atlas(&self) -> Option<TextureAtlasProps> {
        let prop = &self.node.atlas.clone();
        TextureAtlasPropsBuilder::new(prop.to_owned()).parse().ok()
    }

    fn styles(&self) -> Option<KStyle> {
        let prop = &self.node.styles.clone();
        StyleBuilder::new(prop.to_owned()).parse().ok()
    }

    fn widget_name(&self) -> Option<String> {
        let prop = &self.node.name.clone();
        prop.to_owned()
    }

//     pub atlas: TextureAtlasProps,
//     pub styles: KStyle,
//     pub widget_name: WidgetName,

}