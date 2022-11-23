// pub struct Rect {
//     pub posx: f32,
//     pub posy: f32,
//     pub width: f32,
//     pub height: f32,
//     pub z_index: f32,
// }
use std::any::Any;

use kayak_ui::prelude::Edge;

use crate::{json_deserializer::UiParseNode, ui_parser::Conv};

pub struct UiEdge {
    node: UiParseNode,
}
impl UiEdge {
    pub fn new(node: UiParseNode) -> Self {
        Self {
            node
        }
    }

    fn to_f32(&self, prop: &Option<String>, label: &str) -> f32 {
        if let str = Conv::get_prop(prop) {
            Conv(str).to_f32()
        } else {
            panic!("missing {}", label)
        }                    
    }

    fn posx(&self) -> f32 {
        self.to_f32(&self.node.posx.clone(), "posx")
    }

    fn posy(&self) -> f32 {
        self.to_f32(&self.node.posy.clone(), "posy")
    }

    fn width(&self) -> f32 {
        self.to_f32(&self.node.width.clone(), "width")
    }

    fn height(&self) -> f32 {
        self.to_f32(&self.node.height.clone(), "height")
    }

    fn z_index(&self) -> f32 {
        self.to_f32(&self.node.z_index.clone(), "height")
    }

    pub fn parse(&self) -> Result<Edge<f32>, &'static str> {        
        let posx = self.posx();
        let posy = self.posy();
        let width = self.width();
        let height = self.height();
        let z_index = self.z_index();
        let edge = Rect {
            posy,
            posx,
            width,
            height,
            z_index,
            ..Default::default()
        };
        Ok(edge)            
    }
    // top: node.top
}