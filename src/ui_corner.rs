use std::any::Any;
use kayak_ui::prelude::Corner;

use crate::{json_deserializer::UiParseNode, ui_parser::Conv};

pub struct UiCorner {
    node: UiParseNode,
}
impl UiCorner {
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

    fn top_left(&self) -> f32 {
        self.to_f32(&self.node.top_left.clone(), "top")
    }

    fn top_right(&self) -> f32 {
        self.to_f32(&self.node.top_right.clone(), "left")
    }

    fn bottom_left(&self) -> f32 {
        self.to_f32(&self.node.bottom_left.clone(), "right")
    }

    fn bottom_right(&self) -> f32 {
        self.to_f32(&self.node.bottom_right.clone(), "bottom")
    }

    pub fn parse(&self) -> Result<Corner<f32>, &'static str> {        
        let top_left = self.top_left();
        let top_right = self.top_right();
        let bottom_left = self.bottom_left();
        let bottom_right = self.bottom_right();
        let edge = Corner {
            top_left,
            top_right,
            bottom_left,
            bottom_right,
            ..Default::default()
        };
        Ok(edge)            
    }
    // top: node.top
}