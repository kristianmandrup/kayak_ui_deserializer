use std::collections::HashMap;

use kayak_ui::prelude::KStyle;
// use serde_json;
// use std::{result::Result};
// use serde::{Deserialize, Serialize};
use nanoserde::{DeJson};

use crate::{ui_style::StyleBuilder};

pub type OptStr = Option<String>;

#[derive(DeJson, Clone)]
pub struct ImageAsset {
    pub name: String,
    pub path: String,
}
#[derive(DeJson, Clone)]
pub struct FontAsset {
    pub name: String,
    pub path: String,
}
#[derive(DeJson, Clone)]
pub struct Assets {
    pub images: Option<Vec<ImageAsset>>,
    pub fonts: Option<Vec<FontAsset>>,
}
#[derive(DeJson, Clone)]
pub struct Style {
    pub name: String,
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

#[derive(DeJson, Clone)]
pub struct Text {
    content: String,
    size: OptStr,
}    

#[derive(DeJson, Clone)]
pub struct Image {
    path: OptStr,
    image_ref: OptStr,
}    


#[derive(DeJson, Clone)]
pub struct Button {
    name: String,
    style: Style,
}

#[derive(DeJson, Clone)]
pub struct ImageBundle {
    name: String,
    image: Image,
    style: Style,        
}    

#[derive(DeJson, Clone)]
pub struct TextWidget {
    name: String,
    text: Text,
    style: Style,
}

#[derive(DeJson, Clone)]
pub struct Widgets {
    pub buttons: Option<Vec<Button>>,
    pub text_widgets: Option<Vec<TextWidget>>,
    pub image_bundles: Option<Vec<ImageBundle>>,
}

pub struct KWidgets {
    text: HashMap<String, TextWidget>
}


pub struct StoredWidgets {
    pub buttons: HashMap<String, KWidgets>,
    pub text_widgets: HashMap<String, TextWidget>
}
impl StoredWidgets {
    pub fn new() -> Self {
        Self {
            buttons: HashMap::new(),
            text_widgets: HashMap::new()
        }                    
    }
}


pub struct KayakStore {
    // pub assets: HashMap<String, Asset>,
    styles: HashMap<String, KStyle>,
    widgets: StoredWidgets
}
impl KayakStore {
    pub fn new() -> Self {
        Self {
            styles: HashMap::new(),
            widgets: StoredWidgets::new()
        }        
    }
}

impl Default for KayakStore {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(DeJson)]
pub struct KayakData {
    pub assets: Option<Assets>,
    pub styles: Option<Vec<Style>>,
    pub widgets: Option<Widgets>,
}

pub struct KayakBuilder {
    pub data: KayakData,
    pub store: KayakStore
}
impl KayakBuilder {
    pub fn new(data: KayakData) -> Self {
        Self {
            data,
            store: KayakStore::new()
        }        
    }

    pub fn build(&self) -> () {
        self.build_styles();
        self.build_widgets();
    }

    pub fn build_styles(&self) -> () {
        if let Some(items) = self.data.styles.to_owned() {
            for item in items {
                let name = item.clone().name;
                let kstyle = StyleBuilder::new(item).parse().unwrap();
                
                self.store.styles.to_owned().insert(name, kstyle);
            }
        }
    }

    pub fn build_widgets(&self) -> () {
        if let Some(items) = self.data.widgets.to_owned() {
            self.build_buttons(items); 
        }
    }

    pub fn build_buttons(&self, buttons: Widgets) -> () { 
    }

    pub fn text_widgets(&self, text_widgets: Vec<TextWidget>) { 
        for item in text_widgets {
            let name = item.clone().name;
            // let text_widget = build_text_widget(item).unwrap();
            // self.store.widgets.text_widgets.to_owned().insert(name, text_widget);
        }
    }
}

pub fn load(json: &str) {
    if let Ok(data) = DeJson::deserialize_json(json) {
        data
    } else {
        panic!("unable to load Kayak UI from JSON string")
    }
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

    let data: KayakData = DeJson::deserialize_json(json).unwrap();
    let builder = KayakBuilder::new(data);
    builder.build()

    // 
    // assert_eq!(data.styles.unwrap().len(), 0);
    // assert_eq!(kayak.assets.unwrap().images.unwrap().len(), 1);
    // assert_eq!(kayak.assets.unwrap().fonts.unwrap().len(), 1);
    // assert_eq!(kayak.assets.unwrap().fonts.unwrap()[0].name, "roboto");
}