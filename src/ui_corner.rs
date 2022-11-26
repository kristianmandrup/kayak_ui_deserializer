use kayak_ui::prelude::Corner;

use crate::{ui_parser::Conv, serialized::OptStr};

pub struct UiCorner {
    top_left: OptStr,
    top_right: OptStr,
    bottom_left: OptStr,
    bottom_right: OptStr,
}

fn part_to_string(part: &str) -> Option<String> {
    if part.is_empty() { None } else { Some(part.to_string()) }
}

fn corner_from_str(str: String) -> UiCorner {
    let parts = str.split(' ').collect::<Vec<&str>>();
    let top_left = part_to_string(parts[0]);
    let top_right = part_to_string(parts[1]);
    let bottom_left = part_to_string(parts[2]);
    let bottom_right = part_to_string(parts[3]);
    UiCorner {
        top_left,
        top_right,
        bottom_left,
        bottom_right
    }
}

pub struct CornerBuilder {
    node: UiCorner,
}
impl CornerBuilder {
    pub fn new(node: UiCorner) -> Self {
        Self {
            node
        }
    }

    pub fn create_from_str(str: String) -> Self {
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

    pub fn parse(&self) -> Result<Corner<f32>, &'static str> {        
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