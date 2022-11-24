// pub struct Rect {
//     pub posx: f32,
//     pub posy: f32,
//     pub width: f32,
//     pub height: f32,
//     pub z_index: f32,
// }
use std::any::Any;

use kayak_ui::prelude::{Edge, Rect};

use crate::{ui_parser::Conv, json_deserializer::OptStr};

pub struct UiRect {
    pub posy: OptStr,
    pub posx: OptStr,
    pub width: OptStr,
    pub height: OptStr,
    pub z_index: OptStr,
}

pub struct RectBuilder {
    node: UiRect,
}
impl RectBuilder {
    pub fn new(node: UiRect) -> Self {
        Self {
            node
        }
    }

    fn to_f32(&self, prop: &Option<String>, label: &str) -> Option<f32> {
        if let str = Conv::get_prop(prop) {
            if let Some(val) = str {
                Conv(val).to_f32()
            } else {
                None
            }
        } else {
            None
        }                    
    }

    fn posx(&self) -> Option<f32> {
        self.to_f32(&self.node.posx.clone(), "posx")
    }

    fn posy(&self) -> Option<f32> {
        self.to_f32(&self.node.posy.clone(), "posy")
    }

    fn width(&self) -> Option<f32> {
        self.to_f32(&self.node.width.clone(), "width")
    }

    fn height(&self) -> Option<f32> {
        self.to_f32(&self.node.height.clone(), "height")
    }

    fn z_index(&self) -> Option<f32> {
        self.to_f32(&self.node.z_index.clone(), "height")
    }

    pub fn parse(&self) -> Result<Rect, &'static str> {        
        let posx = self.posx();
        let posy = self.posy();
        let width = self.width();
        let height = self.height();
        let z_index = self.z_index();
        let rect = Rect {
            // posy,
            // posx,
            // width,
            // height,
            // z_index,
            ..Default::default()
        };
        // if let Some(val) = posx {
        //     rect.posx = val;    
        // }
        // if let Some(val) = posy {
        //     rect.posy = val;    
        // }
        // if let Some(val) = width {
        //     rect.width = val;    
        // }
        // if let Some(val) = height {
        //     rect.height = val;    
        // }
        // if let Some(val) = z_index {
        //     rect.z_index = val;    
        // }
        Ok(rect)            
    }
    // top: node.top
}