use bevy::ui::{Size, Val};

use crate::{ui_rect::to_val, serialized::SSize};

fn part_to_string(part: &str) -> Option<String> {
    if part.is_empty() { None } else { Some(part.to_string()) }
}

fn all_size(str: String) -> SSize {
    let width = part_to_string(str.as_str());
    let height = part_to_string(str.as_str());
    SSize {
        width,
        height,
    }
}

pub fn size_from_str(str: String) -> SSize {
    let parts = str.split(' ').collect::<Vec<&str>>();
    if parts.len() <= 1 {
        all_size(parts[0].to_string().clone())
    } else {
        let width = part_to_string(parts[0]);
        let height = part_to_string(parts[1]);
        SSize {
            width,
            height,
        }    
    }
}



pub struct SizeBuilder {
    node: SSize,
}
impl SizeBuilder {
    pub fn new(node: SSize) -> Self {
        Self {
            node
        }
    }

    pub fn width(&self) -> Option<Val> {
        let prop = &self.node.width.clone();        
        if let Some(val) = prop.clone() {
            Some(to_val(val))
        } else {
            None
        }
    }

    pub fn height(&self) -> Option<Val> {
        let prop = &self.node.height.clone();        
        if let Some(val) = prop.clone() {
            Some(to_val(val))
        } else {
            None
        }
        
    }

    pub fn parse(&self) -> Result<Size, &'static str> {        
        let width = self.width();
        let height = self.height();
        let mut size = Size::default();
        if let Some(val) = width {
            size.width = val;    
        }        
        if let Some(val) = height {
            size.height = val;    
        }        
        Ok(size)
    }
}
