use std::fmt::Debug;
use std::str::FromStr;
use std::{any::Any};


pub trait UiParser {
    fn parse(&self) -> Result<Box<dyn Any>, &'static str>;
}

pub struct Conv(pub String);

impl Conv {
    pub fn option_str(&self) -> Option<String> {
        let str = self.0.clone();
        if str.is_empty() { None } else { Some(str) }
    }

    pub fn option_f32(&self) -> Option<f32> {
        let str = self.0.clone();
        if str.is_empty() { None } else { self.to_f32() }
    }

    pub fn to_f32(&self) -> Option<f32> {
        let str = self.0.clone();
        str.trim().parse::<f32>().ok()
    }

    pub fn to_type<T: FromStr + Debug + Default>(&self)  -> Option<T> 
    {
        let str = self.0.clone();
        str.trim().parse::<T>().ok()
    }


    pub fn get_prop(prop: &Option<String>) -> Option<String> {
        prop.to_owned()
    }

    pub fn to_bool(&self) -> Option<bool> {
        let str = self.0.clone();
        str.trim().parse().ok()
    }
}



//     - `font`: The name of the font to use 
//     - `line_height`: The height of a line of text (currently in pixels). Defaults to font size * 1.2 which is the firefox default method of calculating line height.
//     - `show_cursor`: If true, displays the default text cursor when hovered.
//     - `size`: The font size (in pixels)
//     - `alignment`: Text alignment.
//     - `user_styles`: Specific styles applied directly to the text itself.
//     - `word_wrap`: Wraps the words if said text would overflow it's parent.


