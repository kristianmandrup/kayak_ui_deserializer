use std::{marker::PhantomData, str::FromStr, fmt::Debug};

use kayak_ui::prelude::{Edge, Units};
use crate::{ui_parser::Conv, serialized::OptStr, kayak::units::{to_units, units_to_str}};

use super::sedge::SEdge;


pub fn to_edge_units(prop: OptStr) -> Option<Edge<Units>> {
    if let Some(str) = prop {
        let unit= to_units(str);
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

pub fn edge_units_to_str(e: Edge<Units>) -> String {
    format!("{} {} {} {}", units_to_str(e.top), units_to_str(e.right), units_to_str(e.bottom), units_to_str(e.left))
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

pub fn deserialize_edge(edge: SEdge) -> Result<Edge<Units>, &'static str> {
    EdgeDeserUnits::new(edge).deserialize()
}

// pub fn deserialize_edge_f32(edge: SEdge) -> Result<Edge<f32>, &'static str> {
//     EdgeDeser::new<f32>(edge).deserialize()
// }

pub struct EdgeDeserUnits {
    node: SEdge
}
impl EdgeDeserUnits {
    pub fn new(node: SEdge) -> Self {
        Self {
            node,
        }
    }

    pub fn create_from_str(str: String) -> Self {
        Self {
            node: edge_from_str(str),
        }
    }

    fn top(&self) -> Option<Units> {
        if let Some(str) = &self.node.top.clone() {
            let units = to_units(str.to_owned());
            Some(units)    
        } else {
            None
        }
    }

    fn left(&self) -> Option<Units> {
        if let Some(str) = &self.node.left.clone() {
            let units = to_units(str.to_owned());
            Some(units)    
        } else {
            None
        }
    }

    fn right(&self) -> Option<Units> {
        if let Some(str) = &self.node.right.clone() {
            let units = to_units(str.to_owned());
            Some(units)    
        } else {
            None
        }

    }

    fn bottom(&self) -> Option<Units> {
        if let Some(str) = &self.node.bottom.clone() {
            let units = to_units(str.to_owned());
            Some(units)    
        } else {
            None
        }
    }

    pub fn deserialize(&self) -> Result<Edge<Units>, &'static str> {        
        let top = self.top();
        let left = self.left();
        let right = self.right();
        let bottom = self.bottom();
        let mut edge = Edge::<Units>::default();
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