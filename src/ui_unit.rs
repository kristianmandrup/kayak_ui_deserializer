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



pub struct UiUnit {
    str: OptStr
}

impl UiUnit {
    pub fn new(str: OptStr) -> Self {
        Self {
            str
        }
    }

    fn number(re: Regex, str: String) -> f32 {
        let str = re.replace(&str, "");
        let num_str = &str[..str.len() - 2];
        num_str.parse::<f32>().unwrap()
    }

    pub fn parse(&self) -> Result<Units, &'static str>  {
        if let Some(str) = self.str.clone() {
            if let Ok(re) = Regex::new(r"px\s*$") {
                let number = UiUnit::number(re, str);
                Ok(Units::Pixels(number))    
            } else if let Ok(re) = Regex::new(r"%\s*$") {
                let number = UiUnit::number(re, str);
                Ok(Units::Percentage(number))    
            } else if let Ok(re) = Regex::new(r"em\s*$") {
                let number = UiUnit::number(re, str);
                Ok(Units::Stretch(number))
            } else {
                Err("Bad unit")
            }
        } else {
            Err("Missing unit")
        }
    }
}