
use bevy::ui::{Val, UiRect};
use kayak_ui::prelude::{Rect};
use regex::Regex;
use crate::{ui_parser::Conv, json_deserializer::{SRect, SUiRect}};

pub struct UiRectBuilder {
    node: SUiRect,
}
impl UiRectBuilder {
    pub fn new(node: SUiRect) -> Self {
        Self {
            node
        }
    }

    fn extract_f32(&self, re: Regex, val: String) -> Option<f32> {
        if let Some(caps) = re.captures(val.as_str()) {
            let text1 = caps.get(1).map_or("", |m| m.as_str());
            text1.trim().parse::<f32>().ok()
        } else {
            None
        }
    }

    fn to_val(&self, val: String) -> Val {
        let px_re = Regex::new(r"\d+\s*px").unwrap();
        let pct_re = Regex::new(r"\d+\s*px").unwrap();
        if let Some(num) = self.extract_f32(px_re, val.clone()) {
            return Val::Px(num)
        } else if let Some(num) = self.extract_f32(pct_re, val.clone()) {
            return Val::Percent(num)
        } else {
            match val.as_str() {
                "auto" => Val::Auto,
                _ => Val::Undefined
            }    
        }
    }    

    pub fn left(&self) -> Option<Val> {
        let prop = &self.node.left.clone();
        if let Some(val) = prop.clone() {
            Some(self.to_val(val))
        } else {
            None
        }       
        
    }

    pub fn right(&self) -> Option<Val> {
        let prop = &self.node.right.clone();
        if let Some(val) = prop.clone() {
            Some(self.to_val(val))
        } else {
            None
        }       
        
    }
    
    pub fn top(&self) -> Option<Val> {
        let prop = &self.node.top.clone();
        if let Some(val) = prop.clone() {
            Some(self.to_val(val))
        } else {
            None
        }       
        
    }

    pub fn bottom(&self) -> Option<Val> {
        let prop = &self.node.bottom.clone();
        if let Some(val) = prop.clone() {
            Some(self.to_val(val))
        } else {
            None
        }       
        
    }

    pub fn parse(&self) -> Result<UiRect, &'static str> {        
        let left = self.left();
        let right = self.right();
        let top = self.top();
        let bottom = self.bottom();
        let mut rect = UiRect::default();
        if let Some(val) = left {
            rect.left = val;    
        }        
        if let Some(val) = right {
            rect.right = val;    
        }        
        if let Some(val) = top {
            rect.top = val;    
        }        
        if let Some(val) = bottom {
            rect.bottom = val;    
        }        
        Ok(rect)
    }
}

pub struct RectBuilder {
    node: SRect,
}
impl RectBuilder {
    pub fn new(node: SRect) -> Self {
        Self {
            node
        }
    }

    fn to_f32(&self, prop: &Option<String>) -> Option<f32> {
        if let Some(str) = Conv::get_prop(prop) {
            Conv(str).to_f32()
        } else {
            None
        }                    
    }

    fn posx(&self) -> Option<f32> {
        self.to_f32(&self.node.posx.clone())
    }

    fn posy(&self) -> Option<f32> {
        self.to_f32(&self.node.posy.clone())
    }

    fn width(&self) -> Option<f32> {
        self.to_f32(&self.node.width.clone())
    }

    fn height(&self) -> Option<f32> {
        self.to_f32(&self.node.height.clone())
    }

    fn z_index(&self) -> Option<f32> {
        self.to_f32(&self.node.z_index.clone())
    }

    pub fn parse(&self) -> Result<Rect, &'static str> {        
        let posx = self.posx();
        let posy = self.posy();
        let width = self.width();
        let height = self.height();
        let z_index = self.z_index();
        let mut rect = Rect::default();
        if let Some(val) = posx {
            rect.posx = val;    
        }
        if let Some(val) = posy {
            rect.posy = val;    
        }
        if let Some(val) = width {
            rect.width = val;    
        }
        if let Some(val) = height {
            rect.height = val;    
        }
        if let Some(val) = z_index {
            rect.z_index = val;    
        }
        Ok(rect)            
    }
    // top: node.top
}