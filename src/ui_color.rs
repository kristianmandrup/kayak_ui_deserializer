use bevy::{prelude::Color};
use regex::Regex;

fn col_to_optstr(color_str: &str) -> Option<String> {
    if color_str.is_empty() { None } else { Some(color_str.to_string()) }
}

fn map_color(s: &str) -> Option<f32> {    
    if let Ok(val) = s.trim().parse::<f32>() {
        Some(val)
    } else {
        None
    }
}


fn parse_hsla(color_str: &str) -> Option<Vec<Option<f32>>> {
    let re = Regex::new(r"hsla\((\.)\)").unwrap();
    if let Some(mtch) = re.find(color_str) {
        let spl = mtch.as_str().split(",");
        let vec = spl.map(|s| map_color(s)).collect();
        Some(vec)
    } else {
        None
    }
}

pub fn parse_hsla_color(color_str: &str) -> Option<Color> {    
    let hsla = parse_hsla(color_str);
    if let Some(colors) = hsla {
        build_hsla(colors)
    } else {
        None
    }
}    

fn parse_rgba(color_str: &str) -> Option<Vec<Option<f32>>>  {
    let re = Regex::new(r"rgba?\((\.)\)").unwrap();
    if let Some(mtch) = re.find(color_str) {
        let spl = mtch.as_str().split(",");
        let vec = spl.map(|s| map_color(s)).collect();
        Some(vec)
    } else {
        None
    }
}

pub fn parse_color_alias(color: &str) -> Color {    
    match color {
        "alice-blue" => Color::ALICE_BLUE,
        "antique-white" => Color::ANTIQUE_WHITE,
        "aquamarine" => Color::AQUAMARINE,
        "azure" => Color::AZURE,
        "beige" => Color::BEIGE,
        "bisque" => Color::BISQUE,
        "black" => Color::BLACK,
        "blue" => Color::BLUE,
        "white" => Color::WHITE,
        "crimson" => Color::CRIMSON,
        "cyan" => Color::CYAN,
        "dark-gray" => Color::DARK_GRAY,
        "dark-green" => Color::DARK_GREEN,
        "fuchsia" => Color::FUCHSIA,
        "gold" => Color::GOLD,
        "gray" => Color::GRAY,
        "green" => Color::GREEN,
        "indigo" => Color::INDIGO,
        "lime-green" => Color::LIME_GREEN,
        "maroon" => Color::MAROON,
        "midnight-blue" => Color::MIDNIGHT_BLUE,
        "navy" => Color::NAVY,
        "none" => Color::NONE,
        "olive" => Color::OLIVE,
        "orange" => Color::ORANGE,
        "organge-red" => Color::ORANGE_RED,
        "pink" => Color::PINK,
        "purple" => Color::PURPLE,
        "red" => Color::RED,
        "salmon" => Color::SALMON,
        "sea-green" => Color::SEA_GREEN,
        "silver" => Color::SILVER,
        "teal" => Color::TEAL,
        "tomato" => Color::TOMATO,
        "turquoise" => Color::TURQUOISE,
        "violet" => Color::VIOLET,
        "yellow" => Color::YELLOW,
        "yellow-green" => Color::YELLOW_GREEN,
        _ => Color::WHITE
    }
}

pub fn parse_color(color_str: &str) -> Option<Color> {    
    if let Some(col) = parse_rgba_color(color_str) {
        Some(col)
    } else if let Some(col) = parse_hsla_color(color_str) {
        Some(col)
    } else {
        Some(parse_color_alias(color_str))
    }
}

pub fn build_hsla(colors: Vec<Option<f32>>) -> Option<Color> {
    if colors.len() < 4 {
        return None
    }
    let h = colors[0].unwrap();
    let s = colors[1].unwrap();
    let l = colors[2].unwrap();
    let a = colors[3].unwrap();
    Some(Color::rgba(h, s, l, a))
}

pub fn parse_rgba_color(color_str: &str) -> Option<Color> {    
    let rgba = parse_rgba(color_str);
    if let Some(colors) = rgba {
        build_rgba(colors)
    } else {
        None
    }
}

pub fn build_rgba(colors: Vec<Option<f32>>) -> Option<Color> {
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
}


