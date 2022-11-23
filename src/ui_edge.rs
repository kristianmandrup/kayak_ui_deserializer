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

    fn top(&self) -> f32 {
        self.to_f32(&self.node.top.clone(), "top")
    }

    fn left(&self) -> f32 {
        self.to_f32(&self.node.left.clone(), "left")
    }

    fn right(&self) -> f32 {
        self.to_f32(&self.node.right.clone(), "right")
    }

    fn bottom(&self) -> f32 {
        self.to_f32(&self.node.bottom.clone(), "bottom")
    }

    pub fn parse(&self) -> Result<Edge<f32>, &'static str> {        
        let top = self.top();
        let left = self.left();
        let right = self.right();
        let bottom = self.bottom();
        let edge = Edge {
            top,
            left,
            right,
            bottom,
            ..Default::default()
        };
        Ok(edge)            
    }
    // top: node.top
}