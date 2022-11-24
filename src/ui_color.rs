use bevy::{prelude::Color};
use regex::Regex;

pub fn parse_rgba(color_str: &str) -> Option<(f32, f32, f32, f32)>  {
    let re = Regex::new(r"rgba(\.)").unwrap();
    if let Some(mtch) = re.find(color_str) {        
        let colors: Vec<f32> = mtch.as_str().split(",").map(|s| s.trim().parse::<f32>().unwrap() / 255.).collect();
        Some((colors[0], colors[1], colors[2], colors[3]))
    } else {
        None
    }
}

pub fn parse_color(color_str: &str) -> Option<Color> {    
    let rgba = parse_rgba(color_str);
    if let Some(colors) = rgba {
        let (red, green, blue, alpha) = rgba.unwrap_or((0.,0.,0.,0.));
        Some(Color::rgba(red, green, blue, alpha))
    } else {
        None
    }
}
