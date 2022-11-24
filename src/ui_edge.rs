use std::{marker::PhantomData, str::FromStr, fmt::Debug};

use kayak_ui::prelude::Edge;
use morphorm::Units;
use crate::{json_deserializer::{OptStr}, ui_parser::Conv, ui_unit::UiUnit};


pub fn to_edge_units(prop: OptStr) -> Edge<Units> {
    let unit= UiUnit::new(prop).parse().unwrap();
    Edge {
        top: unit,
        left: unit,
        right: unit,
        bottom: unit 
    }    
}

pub struct UiEdge {
    top: OptStr,
    left: OptStr,
    right: OptStr,
    bottom: OptStr,
}

fn part_to_string(part: &str) -> Option<String> {
    if part.is_empty() { None } else { Some(part.to_string()) }
}


fn edge_from_str(str: String) -> UiEdge {
    let parts = str.split(' ').collect::<Vec<&str>>();
    let top = part_to_string(parts[0]);
    let right = part_to_string(parts[1]);
    let bottom = part_to_string(parts[2]);
    let left = part_to_string(parts[3]);
    UiEdge {
        top,
        right,
        bottom,
        left
    }
}

pub struct EdgeBuilder<T> {
    node: UiEdge,
    phantom: PhantomData<T>,
}
impl<T> EdgeBuilder<T> where T: Copy + Default + PartialEq + FromStr + Debug {
    pub fn new(node: UiEdge) -> Self {
        Self {
            node,
            phantom: PhantomData

        }
    }

    pub fn create_from_str(str: String) -> Self {
        Self {
            node: edge_from_str(str),
            phantom: PhantomData
        }
    }

    fn to_type(&self, prop: &Option<String>, label: &str) -> Option<T> {
        if let str = Conv::get_prop(prop) {
            if let Some(val) = str {
                Conv(val).to_type::<T>()
            } else {
                None
            }
        } else {
            None
        }                    
    }

    fn top(&self) -> Option<T> {
        self.to_type(&self.node.top.clone(), "top")
    }

    fn left(&self) -> Option<T> {
        self.to_type(&self.node.left.clone(), "left")
    }

    fn right(&self) -> Option<T> {
        self.to_type(&self.node.right.clone(), "right")
    }

    fn bottom(&self) -> Option<T> {
        self.to_type(&self.node.bottom.clone(), "bottom")
    }

    pub fn parse(&self) -> Result<Edge<T>, &'static str> {        
        let top = self.top();
        let left = self.left();
        let right = self.right();
        let bottom = self.bottom();
        let edge = Edge {
            ..Default::default()
        };
        // if let Some(val) = top {
        //     edge.top = val;    
        // }
        // if let Some(val) = left {
        //     edge.left = val;    
        // }
        // if let Some(val) = bottom {
        //     edge.bottom = val;    
        // }
        // if let Some(val) = right {
        //     edge.right = val;    
        // }

        Ok(edge)            
    }
    // top: node.top
}