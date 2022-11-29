use kayak_ui::prelude::Units;
use regex::Regex;

use crate::serialized::OptStr;

pub fn units_to_str(units: Units) -> String {
    match units {
        Units::Pixels(num) => format!("{} px", num),        
        Units::Percentage(num) => format!("{} %", num),
        Units::Stretch(num) => format!("{} em", num),   
        _ => "auto".to_string()
    }
}

pub fn to_units(val: String) -> Units {
    let px_re = Regex::new(r"(\d+)\s*px").unwrap();
    let pct_re = Regex::new(r"(\d+)\s*%").unwrap();
    let em_re = Regex::new(r"(\d+)\s*em").unwrap();
    if let Some(num) = extract_f32(px_re, val.clone()) {
        return Units::Pixels(num)
    } else if let Some(num) = extract_f32(pct_re, val.clone()) {
        return Units::Percentage(num)
    } else if let Some(num) = extract_f32(em_re, val.clone()) {
        return Units::Stretch(num)
    } else {
        Units::Auto
    }
}

pub fn from_units(units: Units) -> String {
    match units {
        Units::Pixels(num) => format!("{}px", num.to_string()),
        Units::Percentage(num) => format!("{}%", num.to_string()),
        Units::Stretch(num) => format!("{}em", num.to_string()),
        _ => panic!("invalid units")
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


pub struct UiUnit {
    str: OptStr
}

impl UiUnit {
    pub fn new(str: OptStr) -> Self {
        Self {
            str
        }
    }

    pub fn parse(&self) -> Option<Units>  {
        if let Some(str) = self.str.clone() {
            Some(to_units(str))
        } else {
            None
        }
    }
}