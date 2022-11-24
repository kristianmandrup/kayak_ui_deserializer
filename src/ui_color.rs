use bevy::{prelude::Color};
use regex::Regex;

fn col_to_optstr(color_str: &str) -> Option<String> {
    if color_str.is_empty() { None } else { Some(color_str.to_string()) }
}

// fn corner_from_str(str: String) -> UiCorner {
//     let parts = str.split(' ').collect::<Vec<&str>>();
//     let top_left = part_to_string(parts[0]);
//     let top_right = part_to_string(parts[1]);
//     let bottom_left = part_to_string(parts[2]);
//     let bottom_right = part_to_string(parts[3]);
// }

fn map_color(s: &str) -> Option<f32> {    
    if let Ok(val) = s.trim().parse::<f32>() {
        Some(val)
    } else {
        None
    }
}
    

pub fn parse_rgba(color_str: &str) -> Option<Vec<Option<f32>>>  {
    let re = Regex::new(r"rgba\(\.\)").unwrap();
    if let Some(mtch) = re.find(color_str) {
        let spl = mtch.as_str().split(",");
        let vec = spl.map(|s| map_color(s)).collect();
        Some(vec)
    } else {
        None
    }
}

pub fn parse_color(color_str: &str) -> Option<Color> {    
    let rgba = parse_rgba(color_str);
    if let Some(colors) = rgba {
        if colors.len() < 3 {
            return None
        }
        let r = colors[0].unwrap();
        let g = colors[1].unwrap();
        let b = colors[2].unwrap();
        if colors[3].is_none() {
            Some(Color::rgb(r, g, b))
        } else {
            let a = colors[3].unwrap();
            Some(Color::rgba(r, g, b, a))
        }        
    } else {
        None
    }
}
