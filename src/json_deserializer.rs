use std::{collections::HashMap};

use bevy::{prelude::{AssetServer, ImageBundle}, asset::FileAssetIo};
use kayak_ui::{prelude::KStyle, widgets::{TextWidgetBundle, KButton, WindowBundle, TextureAtlasBundle}};
use nanoserde::{DeJson};

use crate::{ui_kstyle::KStyleBuilder, ui_button::build_button, ui_window::build_window_bundle, ui_text_widget::build_text_widget_bundle, ui_texture_atlas::build_texture_atlas_bundle, ui_image::build_image_bundle};

pub type OptStr = Option<String>;

#[derive(DeJson, Clone)]
pub struct SSize {
    /// The width of the 2-dimensional area.
    pub width: OptStr,
    /// The height of the 2-dimensional area.
    pub height: OptStr,
}


#[derive(DeJson, Clone)]
pub struct SUiRect {
    pub left: OptStr,
    /// The value corresponding to the right side of the UI rect.
    pub right: OptStr,
    /// The value corresponding to the top side of the UI rect.
    pub top: OptStr,
    /// The value corresponding to the bottom side of the UI rect.
    pub bottom: OptStr,
}


#[derive(DeJson, Clone)]
pub struct SRect {
    pub posy: OptStr,
    pub posx: OptStr,
    pub width: OptStr,
    pub height: OptStr,
    pub z_index: OptStr,
}

#[derive(DeJson, Clone)]
pub struct SChildren {
    pub widgets: Option<SWidgets>
}

#[derive(DeJson, Clone)]
pub struct STextureAtlasProps {
    /// The handle to image
    pub handle: SImage,
    /// The position of the tile (in pixels)
    pub position: Option<Vec<OptStr>>,
    /// The size of the tile (in pixels)
    pub tile_size: Option<Vec<OptStr>>,
}


#[derive(DeJson, Clone)]
pub struct STextureAtlasBundle {
    pub atlas: STextureAtlasProps,
    pub styles: SKStyle,
    pub name: String,
}

#[derive(DeJson, Clone)]
pub struct SWindow {
    /// If true, allows the window to be draggable by its title bar
    pub draggable: OptStr,
    /// The initial position at which to display the window in pixels
    pub initial_position: Option<Vec<OptStr>>,
    /// The size of the window in pixels
    pub size: Option<Vec<OptStr>>,
    /// The text to display in the window's title bar
    pub title: Option<String>,
    /// Styles for the main window quad.
    pub window_styles: SKStyle,
    /// A set of styles to apply to the children element wrapper.
    pub children_styles: SKStyle,
}

#[derive(DeJson, Clone)]
pub struct SWindowBundle {
    pub window: SWindow,
    pub styles: SKStyle,
    pub children: SChildren,
    pub name: String
}

#[derive(DeJson, Clone)]
pub struct SImageAsset {
    pub name: String,
    pub path: String,
}
#[derive(DeJson, Clone)]
pub struct SFontAsset {
    pub name: String,
    pub path: String,
}
#[derive(DeJson, Clone)]
pub struct SAssets {
    pub images: Option<Vec<SImageAsset>>,
    pub fonts: Option<Vec<SFontAsset>>,
}

#[derive(DeJson, Clone)]

pub struct SBevyStyle {
    pub display: OptStr,
    /// Whether to arrange this node relative to other nodes, or positioned absolutely
    pub position_type: OptStr,
    pub direction: OptStr,
    pub flex_direction: OptStr,
    pub flex_wrap: OptStr,
    pub align_items: OptStr,
    pub align_self: OptStr,
    pub align_content: OptStr,
    pub justify_content: OptStr,
    pub position: Option<SUiRect>,
    pub margin: Option<SUiRect>,
    pub padding: Option<SUiRect>,
    pub border: Option<SUiRect>,
    pub flex_grow: OptStr,
    pub flex_shrink: OptStr,
    pub flex_basis: OptStr,    
    pub size: Option<SSize>,    
    pub min_size: Option<SSize>,        
    pub max_size: Option<SSize>,        
    pub aspect_ratio: OptStr,    
    pub overflow: OptStr,                
}

#[derive(DeJson, Clone)]
pub struct SKStyle {
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
pub struct SText {
    pub alignment: OptStr,
    pub content: OptStr,
    pub font: OptStr,
    pub line_height: OptStr,
    pub show_cursor: OptStr,
    pub size: OptStr,
}    

#[derive(DeJson, Clone)]
pub struct SImage {
    pub path: OptStr,
    pub image_ref: OptStr,
}    

#[derive(DeJson, Clone)]
pub struct SButton {
    pub name: String,
    pub style: SKStyle,
}

#[derive(DeJson, Clone)]
pub struct SImageBundle {
    pub name: String,
    pub image: SImage,
    pub style: SBevyStyle,        
}    

#[derive(DeJson, Clone)]
pub struct STextWidgetBundle {
    pub name: String,
    pub text: SText,
    pub style: SKStyle,
}

#[derive(DeJson, Clone)]
pub struct SWidgets {
    pub buttons: Option<Vec<SButton>>,
    pub text_widget_bundles: Option<Vec<STextWidgetBundle>>,
    pub image_bundles: Option<Vec<SImageBundle>>,
    pub window_bundles: Option<Vec<SWindowBundle>>,
    pub texture_atlas_bundles: Option<Vec<STextureAtlasBundle>>,
}

pub struct StoredBundles {
    pub text_widget_bundles: HashMap<String, TextWidgetBundle>,
    pub window_bundles: HashMap<String, WindowBundle>,
    pub texture_atlas_bundles: HashMap<String, TextureAtlasBundle>,
    pub image_bundles: HashMap<String, ImageBundle>
}
impl StoredBundles {
    pub fn new() -> Self {
        Self {
            text_widget_bundles: HashMap::new(),
            window_bundles: HashMap::new(),
            texture_atlas_bundles: HashMap::new(),
            image_bundles: HashMap::new()
        }                    
    }

    pub fn text_widget_bundle(&self, id: &str) -> &TextWidgetBundle {
        self.text_widget_bundles.get(id).unwrap()
    }

    pub fn window_bundle(&self, id: &str) -> &WindowBundle {
        self.window_bundles.get(id).unwrap()
    }

    pub fn texture_atlas_bundle(&self, id: &str) -> &TextureAtlasBundle {
        self.texture_atlas_bundles.get(id).unwrap()
    }    

    pub fn image_bundle(&self, id: &str) -> &ImageBundle {
        self.image_bundles.get(id).unwrap()
    }    
}

pub struct StoredWidgets {
    pub buttons: HashMap<String, KButton>,
}
impl StoredWidgets {
    pub fn new() -> Self {
        Self {
            buttons: HashMap::new(),
        }                    
    }

    pub fn button(&self, id: &str) -> &KButton {
        self.buttons.get(id).unwrap()
    }    
}


pub struct KayakStore {
    // pub assets: HashMap<String, Asset>,
    pub styles: HashMap<String, KStyle>,
    pub widgets: StoredWidgets,
    pub bundles: StoredBundles
}
impl KayakStore {
    pub fn new() -> Self {
        Self {
            styles: HashMap::new(),
            widgets: StoredWidgets::new(),
            bundles: StoredBundles::new()
        }        
    }

    pub fn style(&self, id: &str) -> &KStyle {
        self.styles.get(id).unwrap()
    }

    pub fn button(&self, id: &str) -> &KButton {
        self.widgets.button(id)
    }

    pub fn text_widget_bundle(&self, id: &str) -> &TextWidgetBundle {
        self.bundles.text_widget_bundle(id)
    }

    pub fn window_bundle(&self, id: &str) -> &WindowBundle {
        self.bundles.window_bundle(id)
    }

    pub fn texture_atlas_bundle(&self, id: &str) -> &TextureAtlasBundle {
        self.bundles.texture_atlas_bundle(id)
    }

    pub fn image_bundle(&self, id: &str) -> &ImageBundle {
        self.bundles.image_bundle(id)
    }
}

impl Default for KayakStore {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(DeJson)]
pub struct KayakData {
    pub assets: Option<SAssets>,
    pub styles: Option<Vec<SKStyle>>,
    pub widgets: Option<SWidgets>,
}

pub struct KayakBuilder {
    pub asset_server: AssetServer,
    pub data: KayakData,
    pub store: KayakStore
}
impl KayakBuilder {
    pub fn new(asset_server: AssetServer, data: KayakData) -> Self {
        Self {
            asset_server,
            data,
            store: KayakStore::new()
        }        
    }

    pub fn build(&mut self) -> & mut KayakBuilder {
        self.build_styles();
        self.build_widgets();
        self
    }

    pub fn build_styles(&self) -> () {
        if let Some(items) = self.data.styles.to_owned() {
            for item in items {
                let name = item.clone().name;
                let kstyle = KStyleBuilder::new(item).parse().unwrap();                
                self.store.styles.to_owned().insert(name, kstyle);
            }
        }
    }

    pub fn build_widgets(&mut self) -> () {
        if let Some(items) = self.data.widgets.to_owned() {
            if let Some(buttons) = items.buttons {
                self.build_buttons(buttons);     
            }            
            if let Some(text_widgets) = items.text_widget_bundles {
                self.build_text_widget_bundles(text_widgets);     
            }            
            if let Some(window_bundles) = items.window_bundles {
                self.build_window_bundles(window_bundles);     
            }            
        }
    }

    pub fn build_buttons(&mut self, buttons: Vec<SButton>) -> () { 
        for item in buttons {
            let name = item.clone().name;
            let button = build_button(item).unwrap();
            self.store.widgets.buttons.insert(name, button);
        }
    }

    pub fn build_text_widget_bundles(&mut self, text_widget_bundles: Vec<STextWidgetBundle>) { 
        for item in text_widget_bundles {
            let name = item.to_owned().name;
            let text_widget_bundle = build_text_widget_bundle(item).unwrap();
            self.store.bundles.text_widget_bundles.insert(name, text_widget_bundle);
        }
    }

    pub fn build_window_bundles(&mut self, windows: Vec<SWindowBundle>) { 
        for item in windows {
            let name = item.to_owned().name;
            let window_bundle = build_window_bundle(item).unwrap();
            self.store.bundles.window_bundles.insert(name, window_bundle);
        }
    }

    pub fn build_texture_atlas_bundles(&mut self, tabs: Vec<STextureAtlasBundle>) { 
        for item in tabs {
            let name = item.to_owned().name;
            let tab = build_texture_atlas_bundle(item).unwrap();
            self.store.bundles.texture_atlas_bundles.insert(name, tab);
        }
    }

    pub fn build_image_bundles(&mut self, image_bundles: Vec<SImageBundle>) { 
        for item in image_bundles {
            let name = item.to_owned().name;
            let ib = build_image_bundle(self.asset_server.clone(), item).unwrap();
            self.store.bundles.image_bundles.insert(name, ib);
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
    let source_io = FileAssetIo::new("path", false);
    let asset_server = AssetServer::new(source_io);
    let builder = KayakBuilder::new(asset_server, data).build();
    // 
    // assert_eq!(data.styles.unwrap().len(), 0);
    // assert_eq!(kayak.assets.unwrap().images.unwrap().len(), 1);
    // assert_eq!(kayak.assets.unwrap().fonts.unwrap().len(), 1);
    // assert_eq!(kayak.assets.unwrap().fonts.unwrap()[0].name, "roboto");
}