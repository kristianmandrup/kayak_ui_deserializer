// use serde_json;
// use std::{result::Result};
// use serde::{Deserialize, Serialize};
use nanoserde::{DeJson};

pub type OptStr = Option<String>;

#[derive(DeJson)]
pub struct ImageAsset {
    pub name: String,
    pub path: String,
}
#[derive(DeJson)]
pub struct FontAsset {
    pub name: String,
    pub path: String,
}
#[derive(DeJson)]
pub struct Assets {
    pub images: Option<Vec<ImageAsset>>,
    pub fonts: Option<Vec<FontAsset>>,
}
#[derive(DeJson)]
pub struct Style {
    pub background_color: OptStr,
    pub border: OptStr,
    pub border_color: OptStr,
    pub border_radius: OptStr,
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
    pub offset: OptStr,
    pub padding: OptStr,
    pub padding_top: OptStr,
    pub padding_bottom: OptStr,
    pub padding_left: OptStr,
    pub padding_right: OptStr,
    pub right: OptStr,
    pub row_between: OptStr,
    pub top: OptStr,
    pub width: OptStr,
    pub z_index: OptStr,
}

#[derive(DeJson)]
pub struct Text {
    content: String,
    size: OptStr,
}    

#[derive(DeJson)]
pub struct Image {
    path: OptStr,
    image_ref: OptStr,
}    


#[derive(DeJson)]
pub struct Button {
    name: String,
    style: Style,
}

#[derive(DeJson)]
pub struct ImageBundle {
    name: String,
    image: Image,
    style: Style,        
}    

#[derive(DeJson)]
pub struct TextWidget {
    name: String,
    text: Text,
    style: Style,
}

#[derive(DeJson)]
pub struct Widgets {
    pub buttons: Option<Vec<Button>>,
    pub text_widgets: Option<Vec<TextWidget>>,
    pub image_bundles: Option<Vec<ImageBundle>>,
}

#[derive(DeJson)]
pub struct KayakStruct {
    pub assets: Option<Assets>,
    pub styles: Option<Vec<Style>>,
    pub widgets: Option<Vec<Widgets>>,
}

#[test]
fn array() {


    let json = r#"{
        "assets": {
          "images": [
            {
              "name": "profile-image",
              "type": "image",
              "path": "path/to/profile.png"
            }
          ],
          "fonts": [
            {
              "name": "roboto",
              "type": "font",
              "path": "path/to/roboto.tff"
            }
          ]
        },
      "#;

    let kayak: KayakStruct = DeJson::deserialize_json(json).unwrap();

    // 
    assert_eq!(kayak.styles.unwrap().len(), 0);
    // assert_eq!(kayak.assets.unwrap().images.unwrap().len(), 1);
    // assert_eq!(kayak.assets.unwrap().fonts.unwrap().len(), 1);
    // assert_eq!(kayak.assets.unwrap().fonts.unwrap()[0].name, "roboto");
}