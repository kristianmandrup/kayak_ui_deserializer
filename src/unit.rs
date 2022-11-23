use regex::Regex;

use crate::{json_deserializer::OptStr};

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

pub enum UiNodeUnit {
    Pixels(f32),    
}


pub fn parse_unit(opt: OptStr) -> Result<UiNodeUnit, &'static str>  {
    if let Some(str) = opt {
        let re = Regex::new(r"px\s*$").unwrap();
        let str = re.replace(&str, "");
        let number = &str[..str.len() - 2];
        let pixels = number.parse::<f32>().unwrap();
        Ok(UiNodeUnit::Pixels(pixels))
    } else {
        Err("bad unit")        
    }
}