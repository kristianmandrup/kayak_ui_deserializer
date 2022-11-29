use bevy::ui::{Val, CalculatedSize};

use crate::{serialized::SSize};

use super::ui_rect::ui_rect_deser::to_val;

pub fn deserialize_calc_size(ss: SSize) -> Result<CalculatedSize, &'static str>  {
    CalcSizeDeser::new(ss).deserialize()
}

pub struct CalcSizeDeser {
    node: SSize,
}
impl CalcSizeDeser {
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

    pub fn deserialize(&self) -> Result<CalculatedSize, &'static str> {        
        let width = self.width();
        let height = self.height();
        let calc_size = CalculatedSize::default();
        let mut size = calc_size.size;
        if let Some(val) = width {
            size.width = val;    
        }        
        if let Some(val) = height {
            size.height = val;    
        }        
        Ok(calc_size)
    }
}
