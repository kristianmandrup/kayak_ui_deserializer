use morphorm::Units;
use regex::Regex;

use crate::serialized::OptStr;

// pub enum Units {
//     /// A number of pixels
//     Pixels(f32),
//     /// A percentage of the parent dimension
//     Percentage(f32),
//     /// A factor of the remaining free space
//     Stretch(f32),
//     /// Automatically determine the value
//     Auto,
// }
pub fn to_units(val: String) -> Option<Units> {
    let px_re = Regex::new(r"(\d+)\s*px").unwrap();
    let pct_re = Regex::new(r"(\d+)\s*%").unwrap();
    let em_re = Regex::new(r"(\d+)\s*em").unwrap();
    if let Some(num) = extract_f32(px_re, val.clone()) {
        return Some(Units::Pixels(num))
    } else if let Some(num) = extract_f32(pct_re, val.clone()) {
        return Some(Units::Percentage(num))
    } else if let Some(num) = extract_f32(em_re, val.clone()) {
        return Some(Units::Stretch(num))
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
            to_units(str)
        } else {
            None
        }
    }
}