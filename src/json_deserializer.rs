use serde_json;
use std::{result::Result};
use serde::{Deserialize, Serialize};

use crate::ui_parser::{build_text_widget, UiNode};

// use crate::morph::build_world;

pub type OptStr = Option<String>;
// type OptNum = Option<u32>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UiParseNode {
   #[serde(skip_serializing_if = "Option::is_none")]
   pub alignment: OptStr,
   pub asset_path: OptStr,
   pub color: OptStr,
   pub background_color: OptStr,
   pub border: OptStr,
   pub border_color: OptStr,
   pub border_radius: OptStr,
   pub col_between: OptStr,
   pub content: OptStr,
   pub cursor: OptStr,   
   pub layout_type: OptStr,
   pub max_height: OptStr,
   pub max_width: OptStr,
   pub min_height: OptStr,
   pub min_width: OptStr,
   pub offset: OptStr,
   pub padding: OptStr,
   pub padding_top: OptStr,
   pub padding_bottom: OptStr,
   pub padding_left: OptStr,
   pub padding_right: OptStr,
   pub row_between: OptStr,
   pub z_index: OptStr,
   pub posx: OptStr,
   pub posy: OptStr,
   pub render_command: OptStr,
   pub word_wrap: OptStr,
    
   // corner
   pub top_left: OptStr,
   pub top_right: OptStr,
   pub bottom_left: OptStr,
   pub bottom_right: OptStr,

   // edge
   pub top: OptStr,
   pub left: OptStr,
   pub right: OptStr,
   pub bottom: OptStr,

   pub text: OptStr,
   pub font: OptStr,
   pub font_size: OptStr,
   pub line_height: OptStr,
   pub show_cursor: OptStr,
   pub size: OptStr,
   pub width: OptStr,
   pub height: OptStr,
   pub child_space: OptStr,
   pub position_type: OptStr,
} 

pub fn parse_ui() -> Result<UiNode, &'static str> {

    let ui_json = r#"
    {
        "width": "2 px",
        "height": "5 px"
    }
    "#;    

    let ui = serde_json::from_str(ui_json).unwrap();
    println!("ui {:#?}", ui);
    build_text_widget(ui)
}