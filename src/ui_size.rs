use bevy::ui::{Size, Val};

use crate::{json_deserializer::SSize, ui_rect::to_val};

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
