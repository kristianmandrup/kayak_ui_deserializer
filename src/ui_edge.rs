use std::{marker::PhantomData, str::FromStr, fmt::Debug};

use kayak_ui::prelude::Edge;
use morphorm::Units;
use crate::{json_deserializer::{UiParseNode, OptStr}, ui_parser::Conv, ui_unit::UiUnit};


pub fn to_edge_units(prop: OptStr) -> Edge<Units> {
    let unit= UiUnit::new(prop).parse().unwrap();
    Edge {
        top: unit,
        left: unit,
        right: unit,
        bottom: unit 
    }    
}


pub struct UiEdge<T> {
    node: UiParseNode,
    phantom: PhantomData<T>,
}
impl<T> UiEdge<T> where T: Copy + Default + PartialEq + FromStr + Debug {
    pub fn new(node: UiParseNode) -> Self {
        Self {
            node,
            phantom: PhantomData

        }
    }

    fn to_type(&self, prop: &Option<String>, label: &str) -> T {
        if let str = Conv::get_prop(prop) {
            Conv(str).to_type::<T>()
        } else {
            panic!("missing {}", label)
        }                    
    }

    fn top(&self) -> T {
        self.to_type(&self.node.top.clone(), "top")
    }

    fn left(&self) -> T {
        self.to_type(&self.node.left.clone(), "left")
    }

    fn right(&self) -> T {
        self.to_type(&self.node.right.clone(), "right")
    }

    fn bottom(&self) -> T {
        self.to_type(&self.node.bottom.clone(), "bottom")
    }

    pub fn parse(&self) -> Result<Edge<T>, &'static str> {        
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