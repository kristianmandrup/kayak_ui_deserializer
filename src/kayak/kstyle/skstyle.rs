use nanoserde::{DeJson, SerJson, DeRon, SerRon};

use crate::{serialized::OptStr, bevy::corner::scorner::SCorner, kayak::edge::sedge::SEdge};

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct SKStyle {
    pub name: String,
    pub extends: OptStr,
    pub background_color: OptStr,
    pub border: OptStr,
    pub border_color: OptStr,
    pub border_radius: OptStr, // Corner
    pub border_radius_obj: Option<SCorner>,
    pub bottom: OptStr,
    pub col_between: OptStr,
    pub color: OptStr,
    pub content: OptStr,
    pub cursor: OptStr,   
    pub font: OptStr,
    pub font_size: OptStr,
    pub height: OptStr,
    pub layout_type: OptStr,
    pub left: OptStr,
    pub line_height: OptStr,
    pub max_height: OptStr,
    pub max_width: OptStr,
    pub min_height: OptStr,
    pub min_width: OptStr,
    pub offset: OptStr, // Edge
    pub offset_obj: Option<SEdge>, // Edge
    pub padding: OptStr,
    pub padding_obj: Option<SEdge>, // Edge
    pub padding_top: OptStr,
    pub padding_bottom: OptStr,
    pub padding_left: OptStr,
    pub padding_right: OptStr,
    pub position_type: OptStr,
    pub right: OptStr,
    pub row_between: OptStr,
    pub top: OptStr,
    pub width: OptStr,
    pub z_index: OptStr,
}
impl Default for SKStyle {
    fn default() -> Self {
        Self {
            name: "noname".to_string(),
            extends: None,
            background_color: None,
            border: None,
            border_color: None,
            border_radius: None,
            border_radius_obj: None,
            bottom: None,
            col_between: None,
            color: None,
            content: None,
            cursor: None,   
            font: None,
            font_size: None,
            height: None,
            layout_type: None,
            left: None,
            line_height: None,
            max_height: None,
            max_width: None,
            min_height: None,
            min_width: None,
            offset: None,
            offset_obj: None,
            padding: None,
            padding_obj: None,
            padding_top: None,
            padding_bottom: None,
            padding_left: None,
            padding_right: None,
            position_type: None,
            right: None,
            row_between: None,
            top: None,
            width: None,
            z_index: None,
        }
    }
}
