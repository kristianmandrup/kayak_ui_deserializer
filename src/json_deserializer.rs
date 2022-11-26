use std::{collections::HashMap};

use bevy::{prelude::{AssetServer, ImageBundle}, asset::FileAssetIo};
use kayak_ui::{prelude::KStyle, widgets::{TextWidgetBundle, KButton, WindowBundle, TextureAtlasBundle, KButtonBundle, BackgroundBundle, ClipBundle, TextBoxBundle, ElementBundle}};
use nanoserde::{DeJson};

use crate::{ui_kstyle::KStyleBuilder, ui_button::{build_button, build_button_bundle}, ui_window::build_window_bundle, ui_text_widget::build_text_widget_bundle, ui_texture_atlas::build_texture_atlas_bundle, ui_image::build_image_bundle, ui_background::build_background_bundle, ui_clip::build_clip_bundle, ui_text_box_bundle::build_text_box_bundle, ui_element_bundle::build_element_bundle};

pub type OptStr = Option<String>;

#[derive(DeJson, Clone)]
pub struct SClipBundle {
    pub clip: OptStr,
    pub style: SKStyle,
    pub name: String,
}

#[derive(DeJson, Clone)]
pub struct SBackgroundBundle {
    pub background: OptStr,
    pub style: SKStyle,
    pub name: String,
}

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
    pub atlas: Option<STextureAtlasProps>,
    pub styles: Option<SKStyle>,
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
    pub window: Option<SWindow>,
    pub styles: Option<SKStyle>,
    // pub children: SChildren,
    pub name: String
}

#[derive(DeJson, Clone)]
pub struct SElementBundle {
    pub element: OptStr,
    pub styles: Option<SKStyle>,
    // pub children: SChildren,
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
pub struct STextProps {
    pub alignment: OptStr,
    pub content: OptStr,
    pub font: OptStr,
    pub line_height: OptStr,
    pub show_cursor: OptStr,
    pub size: OptStr,
    pub user_styles: SKStyle,
    pub word_wrap: OptStr
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
pub struct SButtonBundle {
    pub button: Option<SButton>,
    pub styles: Option<SKStyle>,
    // pub on_event: OnEvent,
    pub name: String,
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
    pub text: STextProps,
    pub style: SKStyle,
}

#[derive(DeJson, Clone)]
pub struct SWidgets {
    pub buttons: Option<Vec<SButton>>,
}


#[derive(DeJson, Clone)]
pub struct SBundles {
    pub text_widget_bundles: Option<Vec<STextWidgetBundle>>,
    pub image_bundles: Option<Vec<SImageBundle>>,
    pub window_bundles: Option<Vec<SWindowBundle>>,
    pub texture_atlas_bundles: Option<Vec<STextureAtlasBundle>>,
    pub button_bundles: Option<Vec<SButtonBundle>>,
    pub background_bundles: Option<Vec<SBackgroundBundle>>,    
    pub clip_bundles: Option<Vec<SClipBundle>>,    
    pub text_box_bundles: Option<Vec<STextBoxBundle>>,        
    pub element_bundles: Option<Vec<SElementBundle>>,        
}

pub struct StoredBundles {
    pub text_widget_bundles: HashMap<String, TextWidgetBundle>,
    pub window_bundles: HashMap<String, WindowBundle>,
    pub texture_atlas_bundles: HashMap<String, TextureAtlasBundle>,
    pub image_bundles: HashMap<String, ImageBundle>,
    pub button_bundles: HashMap<String, KButtonBundle>,
    pub background_bundles: HashMap<String, BackgroundBundle>,
    pub clip_bundles: HashMap<String, ClipBundle>,
    pub text_box_bundles: HashMap<String, TextBoxBundle>,
    pub element_bundles: HashMap<String, ElementBundle>,
}
impl StoredBundles {
    pub fn new() -> Self {
        Self {
            text_widget_bundles: HashMap::new(),
            text_box_bundles: HashMap::new(),            
            window_bundles: HashMap::new(),
            texture_atlas_bundles: HashMap::new(),
            image_bundles: HashMap::new(),
            button_bundles: HashMap::new(),
            background_bundles: HashMap::new(),
            clip_bundles: HashMap::new(),
            element_bundles: HashMap::new(),            
        }                    
    }

    pub fn text_widget_bundle(&self, id: &str) -> &TextWidgetBundle {
        self.text_widget_bundles.get(id).unwrap()
    }

    pub fn text_box_bundle(&self, id: &str) -> &TextBoxBundle {
        self.text_box_bundles.get(id).unwrap()
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

    pub fn button_bundle(&self, id: &str) -> &KButtonBundle {
        self.button_bundles.get(id).unwrap()
    }    

    pub fn background_bundle(&self, id: &str) -> &BackgroundBundle {
        self.background_bundles.get(id).unwrap()
    }    

    pub fn clip_bundle(&self, id: &str) -> &ClipBundle {
        self.clip_bundles.get(id).unwrap()
    }    

    pub fn element_bundle(&self, id: &str) -> &ElementBundle {
        self.element_bundles.get(id).unwrap()
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

    pub fn text_box_bundle(&self, id: &str) -> &TextBoxBundle {
        self.bundles.text_box_bundle(id)
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

    pub fn button_bundle(&self, id: &str) -> &KButtonBundle {
        self.bundles.button_bundle(id)
    }

    pub fn background_bundle(&self, id: &str) -> &BackgroundBundle {
        self.bundles.background_bundle(id)
    }

    pub fn clip_bundle(&self, id: &str) -> &ClipBundle {
        self.bundles.clip_bundle(id)
    }

    pub fn element_bundle(&self, id: &str) -> &ElementBundle {
        self.bundles.element_bundle(id)
    }
}

impl Default for KayakStore {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(DeJson, Clone)]
pub struct STextBoxProps {
    pub disabled: OptStr,
    pub placeholder: OptStr,
    pub value: OptStr,
}

#[derive(DeJson, Clone)]
pub struct STextBoxBundle {
    pub text_box: Option<STextBoxProps>,
    pub style: Option<SKStyle>,
    // pub on_event: OnEvent,
    // pub on_layout: OnLayout,
    // pub on_change: OnChange,
    pub focusable: OptStr,
    pub name: String,
}


#[derive(DeJson)]
pub struct KayakData {
    pub assets: Option<SAssets>,
    pub styles: Option<Vec<SKStyle>>,
    pub widgets: Option<SWidgets>,
    pub bundles: Option<SBundles>,
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
        self.build_bundles();
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
        }
    }

    pub fn build_bundles(&mut self) -> () {
        if let Some(items) = self.data.bundles.to_owned() {
            if let Some(text_widgets) = items.text_widget_bundles {
                self.build_text_widget_bundles(text_widgets);     
            }            
            if let Some(window_bundles) = items.window_bundles {
                self.build_window_bundles(window_bundles);     
            }            
            if let Some(button_bundles) = items.button_bundles {
                self.build_button_bundles(button_bundles);     
            }            
            if let Some(texture_atlas_bundles) = items.texture_atlas_bundles {
                self.build_texture_atlas_bundles(texture_atlas_bundles);     
            }            
            if let Some(image_bundles) = items.image_bundles {
                self.build_image_bundles(image_bundles);     
            }            
            if let Some(background_bundles) = items.background_bundles {
                self.build_background_bundles(background_bundles);     
            }            
            if let Some(clip_bundles) = items.clip_bundles {
                self.build_clip_bundles(clip_bundles);     
            }                    
            if let Some(text_box_bundles) = items.text_box_bundles {
                self.build_text_box_bundles(text_box_bundles);     
            }
            if let Some(element_bundles) = items.element_bundles {
                self.build_element_bundles(element_bundles);     
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
    
    pub fn build_button_bundles(&mut self, button_bundles: Vec<SButtonBundle>) { 
        for item in button_bundles {
            let name = item.to_owned().name;
            let button_bundle = build_button_bundle(item).unwrap();
            self.store.bundles.button_bundles.insert(name, button_bundle);
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

    pub fn build_background_bundles(&mut self, background_bundles: Vec<SBackgroundBundle>) { 
        for item in background_bundles {
            let name = item.to_owned().name;
            let ib = build_background_bundle(item).unwrap();
            self.store.bundles.background_bundles.insert(name, ib);
        }
    }

    pub fn build_clip_bundles(&mut self, clip_bundles: Vec<SClipBundle>) { 
        for item in clip_bundles {
            let name = item.to_owned().name;
            let ib = build_clip_bundle(item).unwrap();
            self.store.bundles.clip_bundles.insert(name, ib);
        }
    }

    pub fn build_text_box_bundles(&mut self, text_box_bundles: Vec<STextBoxBundle>) { 
        for item in text_box_bundles {
            let name = item.to_owned().name;
            let ib = build_text_box_bundle(item).unwrap();
            self.store.bundles.text_box_bundles.insert(name, ib);
        }
    }

    pub fn build_element_bundles(&mut self, element_bundles: Vec<SElementBundle>) { 
        for item in element_bundles {
            let name = item.to_owned().name;
            let ib = build_element_bundle(item).unwrap();
            self.store.bundles.element_bundles.insert(name, ib);
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