use kayak_ui::prelude::Corner;

use crate::{ui_parser::Conv};

use super::scorner::SCorner;


fn part_to_string(part: &str) -> Option<String> {
    if part.is_empty() { None } else { Some(part.to_string()) }
}

fn all_corner(str: String) -> SCorner {
    let top_left = part_to_string(str.as_str());
    let top_right = part_to_string(str.as_str());
    let bottom_left = part_to_string(str.as_str());
    let bottom_right = part_to_string(str.as_str());
    SCorner {
        top_left,
        top_right,
        bottom_left,
        bottom_right
    }
}

pub fn corner_to_str(c: Corner<f32>) -> String {
    format!("{} {} {} {}", c.top_left, c.top_right, c.bottom_left, c.bottom_right)
}

pub fn corner_from_str(str: String) -> SCorner {
    let parts = str.split(' ').collect::<Vec<&str>>();
    if parts.len() <= 1 {
        all_corner(parts[0].to_string().clone())
    } else {
        let top_left = part_to_string(parts[0]);
        let top_right = part_to_string(parts[1]);
        let bottom_left = part_to_string(parts[2]);
        let bottom_right = part_to_string(parts[3]);
        SCorner {
            top_left,
            top_right,
            bottom_left,
            bottom_right
        }    
    }
}

pub fn deserialize_corner(corner: SCorner) -> Result<Corner<f32>, &'static str>  {
    CornerDeser::new(corner).deserialize()
}


pub struct CornerDeser {
    node: SCorner,
}
impl CornerDeser {
    pub fn new(node: SCorner) -> Self {
        Self {
            node
        }
    }

    pub fn from_str(str: String) -> Self {
        Self {
            node: corner_from_str(str),
        }
    }

    fn to_f32(&self, prop: &Option<String>) -> Option<f32> {
        if let Some(str) = Conv::get_prop(prop) {
            Conv(str).to_f32()
        } else {
            None
        }                    
    }

    fn top_left(&self) -> Option<f32> {
        self.to_f32(&self.node.top_left.clone())
    }

    fn top_right(&self) -> Option<f32> {
        self.to_f32(&self.node.top_right.clone())
    }

    fn bottom_left(&self) -> Option<f32> {
        self.to_f32(&self.node.bottom_left.clone())
    }

    fn bottom_right(&self) -> Option<f32> {
        self.to_f32(&self.node.bottom_right.clone())
    }

    pub fn deserialize(&self) -> Result<Corner<f32>, &'static str> {        
        let top_left = self.top_left();
        let top_right = self.top_right();
        let bottom_left = self.bottom_left();
        let bottom_right = self.bottom_right();
        let mut corner = Corner::default();
        if let Some(val) = top_left {
            corner.top_left = val;    
        }
        if let Some(val) = top_right {
            corner.top_right = val;    
        }
        if let Some(val) = bottom_left {
            corner.bottom_left = val;    
        }
        if let Some(val) = bottom_right {
            corner.bottom_right = val;    
        }
        Ok(corner)            
    }
    // top: node.top
}