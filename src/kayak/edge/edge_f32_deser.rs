use std::{marker::PhantomData, str::FromStr, fmt::Debug};

use kayak_ui::prelude::{Edge, Units};
use crate::{ui_parser::Conv, serialized::OptStr, kayak::units::to_units, bevy::style::style_deser::to_f32};

use super::sedge::SEdge;


pub fn to_edge_f32(prop: OptStr) -> Option<Edge<f32>> {
    if let Some(str) = prop {
        let unit= to_f32(str);
        let edge = Edge {
            top: unit,
            left: unit,
            right: unit,
            bottom: unit 
        };
        Some(edge)
    } else {
        None
    }
    // let unit= UiUnit::new(prop).parse().unwrap();
}

pub fn edge_to_str(e: Edge<f32>) -> String {
    format!("{} {} {} {}", e.top, e.right, e.bottom, e.left)
}

fn part_to_string(part: &str) -> Option<String> {
    if part.is_empty() { None } else { Some(part.to_string()) }
}

fn edge_from_str(str: String) -> SEdge {
    let parts = str.split(' ').collect::<Vec<&str>>();
    if parts.len() <= 1 {
        return SEdge {
            top: None,
            right: None,
            bottom: None,
            left: None,
            all: Some(str)
        }    
    }
    if parts.len() < 4 {
        panic!("bad edge")
    }

    let top = part_to_string(parts[0]);
    let right = part_to_string(parts[1]);
    let bottom = part_to_string(parts[2]);
    let left = part_to_string(parts[3]);
    SEdge {
        top,
        right,
        bottom,
        left,
        all: None
    }
}

pub fn deserialize_edge_f32(edge: SEdge) -> Result<Edge<f32>, &'static str> {
    EdgeDeserF32::new(edge).deserialize()
}

// pub fn deserialize_edge_f32(edge: SEdge) -> Result<Edge<f32>, &'static str> {
//     EdgeDeser::new<f32>(edge).deserialize()
// }

pub struct EdgeDeserF32 {
    node: SEdge,
}
impl EdgeDeserF32 {
    pub fn new(node: SEdge) -> Self {
        Self {
            node,
        }
    }

    pub fn create_from_str(str: String) -> Self {
        Self {
            node: edge_from_str(str)
        }
    }

    fn to_type(&self, prop: &Option<String>) -> Option<f32> {
        if let Some(str) = Conv::get_prop(prop) {
            Conv(str).to_type::<f32>()
        } else {
            None
        }                    
    }

    fn top(&self) -> Option<f32> {
        self.to_type(&self.node.top.clone())
    }

    fn left(&self) -> Option<f32> {
        self.to_type(&self.node.left.clone())
    }

    fn right(&self) -> Option<f32> {
        self.to_type(&self.node.right.clone())
    }

    fn bottom(&self) -> Option<f32> {
        self.to_type(&self.node.bottom.clone())
    }

    pub fn deserialize(&self) -> Result<Edge<f32>, &'static str> {        
        let top = self.top();
        let left = self.left();
        let right = self.right();
        let bottom = self.bottom();
        let mut edge = Edge::<f32>::default();
        if let Some(val) = top {
            edge.top = val;    
        }
        if let Some(val) = left {
            edge.left = val;    
        }
        if let Some(val) = bottom {
            edge.bottom = val;    
        }
        if let Some(val) = right {
            edge.right = val;    
        }

        Ok(edge)            
    }
}