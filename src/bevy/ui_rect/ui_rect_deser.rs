use bevy::ui::{Val, UiRect};
use regex::Regex;

use super::sui_rect::SUiRect;

pub fn to_val(val: String) -> Val {
    let px_re = Regex::new(r"(\d+)\s*px").unwrap();
    let pct_re = Regex::new(r"(\d+)\s*%").unwrap();
    if let Some(num) = extract_f32(px_re, val.clone()) {
        return Val::Px(num)
    } else if let Some(num) = extract_f32(pct_re, val.clone()) {
        return Val::Percent(num)
    } else {
        match val.as_str() {
            "auto" => Val::Auto,
            _ => Val::Undefined
        }    
    }
}    

fn part_to_string(part: &str) -> Option<String> {
    if part.is_empty() { None } else { Some(part.to_string()) }
}

pub fn rect_from_str(str: String) -> Option<SUiRect> {
    let parts = str.split(' ').collect::<Vec<&str>>();
    if parts.len() == 1 {    
        let top = part_to_string(parts[0]);
        let right = part_to_string(parts[0]);
        let bottom = part_to_string(parts[0]);
        let left = part_to_string(parts[0]);
        let rect = SUiRect {
            top,
            left,
            bottom,
            right
        };
        Some(rect)
    } else if parts.len() == 4 {
        let top = part_to_string(parts[0]);
        let right = part_to_string(parts[1]);
        let bottom = part_to_string(parts[2]);
        let left = part_to_string(parts[3]);
        let rect = SUiRect {
            top,
            left,
            bottom,
            right
        };
        Some(rect)
    } else {
        None
    }
}

pub fn extract_f32(re: Regex, val: String) -> Option<f32> {
    if let Some(caps) = re.captures(val.as_str()) {
        let text1 = caps.get(1).map_or("", |m| m.as_str());
        text1.trim().parse::<f32>().ok()
    } else {
        None
    }
}

pub fn deserialize_ui_rect(rect: SUiRect) -> Result<UiRect, &'static str>  {
    UiRectDeser::new(rect).deserialize()
}


pub struct UiRectDeser {
    node: SUiRect,
}
impl UiRectDeser {
    pub fn new(node: SUiRect) -> Self {
        Self {
            node
        }
    }

    pub fn left(&self) -> Option<Val> {
        let prop = &self.node.left.clone();
        if let Some(val) = prop.clone() {
            Some(to_val(val))
        } else {
            None
        }       
        
    }

    pub fn right(&self) -> Option<Val> {
        let prop = &self.node.right.clone();
        if let Some(val) = prop.clone() {
            Some(to_val(val))
        } else {
            None
        }       
        
    }
    
    pub fn top(&self) -> Option<Val> {
        let prop = &self.node.top.clone();
        if let Some(val) = prop.clone() {
            Some(to_val(val))
        } else {
            None
        }       
        
    }

    pub fn bottom(&self) -> Option<Val> {
        let prop = &self.node.bottom.clone();
        if let Some(val) = prop.clone() {
            Some(to_val(val))
        } else {
            None
        }       
        
    }

    pub fn deserialize(&self) -> Result<UiRect, &'static str> {        
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

