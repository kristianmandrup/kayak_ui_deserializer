use std::{marker::PhantomData, str::FromStr, fmt::Debug};

use kayak_ui::prelude::Edge;
use morphorm::Units;
use crate::{ui_parser::Conv, serialized::{OptStr, SEdge}, morphorm::units::UiUnit};


pub fn to_edge_units(prop: OptStr) -> Edge<Units> {
    let unit= UiUnit::new(prop).parse().unwrap();
    Edge {
        top: unit,
        left: unit,
        right: unit,
        bottom: unit 
    }    
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

pub fn deserialize_edge<T>(edge: SEdge) -> Result<Edge<T>, &'static str> where T: Copy + Default + PartialEq + FromStr + Debug  {
    EdgeDeser::new(edge).deserialize()
}

pub struct EdgeDeser<T> {
    node: SEdge,
    phantom: PhantomData<T>,
}
impl<T> EdgeDeser<T> where T: Copy + Default + PartialEq + FromStr + Debug {
    pub fn new(node: SEdge) -> Self {
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

    fn to_type(&self, prop: &Option<String>) -> Option<T> {
        if let Some(str) = Conv::get_prop(prop) {
            Conv(str).to_type::<T>()
        } else {
            None
        }                    
    }

    fn top(&self) -> Option<T> {
        self.to_type(&self.node.top.clone())
    }

    fn left(&self) -> Option<T> {
        self.to_type(&self.node.left.clone())
    }

    fn right(&self) -> Option<T> {
        self.to_type(&self.node.right.clone())
    }

    fn bottom(&self) -> Option<T> {
        self.to_type(&self.node.bottom.clone())
    }

    pub fn deserialize(&self) -> Result<Edge<T>, &'static str> {        
        let top = self.top();
        let left = self.left();
        let right = self.right();
        let bottom = self.bottom();
        let mut edge = Edge::default();
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
    // top: node.top
}