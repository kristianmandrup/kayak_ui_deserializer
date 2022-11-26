use bevy::{prelude::{AssetServer}, asset::FileAssetIo};
use nanoserde::{DeJson};

use crate::{ui_kstyle::KStyleBuilder, ui_button::{build_button}, ui_text_widget::build_text_widget_bundle, ui_texture_atlas::build_texture_atlas_bundle, ui_background::build_background_bundle, ui_clip::build_clip_bundle, ui_text_box_bundle::build_text_box_bundle, ui_element_bundle::build_element_bundle, kayak_store::KayakStore, serialized::{KayakData, SAssets, SButton, SButtonBundle, SWindowBundle, STextWidgetBundle, STextureAtlasBundle, SImageBundle, SBackgroundBundle, SClipBundle, STextBoxBundle, SElementBundle}, ui_button_bundle::build_button_bundle, ui_window_bundle::build_window_bundle, ui_image_bundle::build_image_bundle};

pub struct KayakBuilder {
    // pub asset_server: &'a AssetServer,
    pub store: KayakStore,
    pub data: KayakData,    
}
impl KayakBuilder {
    pub fn new(asset_server: AssetServer, data: KayakData) -> Self {
        Self {
            // asset_server,
            data,
            store: KayakStore::new(asset_server)
        }        
    }

    pub fn process(&mut self) -> &Self {
        self.build();
//        self.extend();
        self
    }

    // pub fn extend(&mut self) -> &Self {
    //     self.extend_widgets();
    //     self
    // }

    // pub fn extend_widgets(&self) {
    //     if let Some(widgets) = self.data.widgets.to_owned() {
    //         if let Some(btns) = widgets.buttons {
    //             for btn in btns {
    //                 self.store.extend_kstyle(btn.styles);
    //             }    
    //         }
    //     }
    // }

    pub fn build(&mut self) -> &Self {
        self.build_styles();
        self.build_widgets();
        self.build_bundles();
        self.build_assets();
        self
    }

    pub fn build_assets(&self) -> &Self {
        if let Some(assets) = self.data.assets.to_owned() {
            self.build_images(assets);
        }        
        self
    }

    fn asset_server(&self) -> AssetServer {
        self.store.asset_server.clone()
    }

    pub fn build_images(&self, assets: SAssets) -> &Self {
        if let Some(images) = assets.images {
            for image in images {
                let handle = self.asset_server().load(image.path);
                let mut images = self.store.assets.images.to_owned();
                images.insert(image.name, handle);    
            }    
        }
        self
    }

    pub fn build_fonts(&self, assets: SAssets) -> &Self {
        if let Some(fonts) = assets.fonts {
            for font in fonts {
                let handle = self.asset_server().load(font.path);
                let mut fonts = self.store.assets.fonts.to_owned();
                fonts.insert(font.name, handle);    
            }    
        }
        self
    }

    pub fn build_styles(&self) -> &Self {
        if let Some(items) = self.data.styles.to_owned() {
            for item in items {
                let name = item.clone().name;
                let kstyle = KStyleBuilder::new(item).parse().unwrap();                
                self.store.styles.to_owned().insert(name, kstyle);
            }
        }
        self
    }

    pub fn build_widgets(&mut self) -> &Self {
        if let Some(items) = self.data.widgets.to_owned() {
            if let Some(buttons) = items.buttons {
                self.build_buttons(buttons);     
            }            
        }
        self
    }

    pub fn build_bundles(&mut self) -> &Self {
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
        self                              
    }

    pub fn build_buttons(&mut self, buttons: Vec<SButton>) -> &Self { 
        for item in buttons {
            let name = item.clone().name;
            let button = build_button(&self.store, item).unwrap();
            self.store.widgets.buttons.insert(name, button);
        }
        self
    }
    
    pub fn build_button_bundles(&mut self, button_bundles: Vec<SButtonBundle>) -> &Self { 
        for item in button_bundles {
            let name = item.to_owned().name;
            let button_bundle = build_button_bundle(&self.store, item).unwrap();
            self.store.bundles.button_bundles.insert(name, button_bundle);
        }
        self
    }

    pub fn build_text_widget_bundles(&mut self, text_widget_bundles: Vec<STextWidgetBundle>) -> &Self { 
        for item in text_widget_bundles {
            let name = item.to_owned().name;
            let text_widget_bundle = build_text_widget_bundle(item).unwrap();
            self.store.bundles.text_widget_bundles.insert(name, text_widget_bundle);
        }
        self
    }

    pub fn build_window_bundles(&mut self, windows: Vec<SWindowBundle>) -> &Self { 
        for item in windows {
            let name = item.to_owned().name;
            let window_bundle = build_window_bundle(&self.store, item).unwrap();
            self.store.bundles.window_bundles.insert(name, window_bundle);
        }
        self
    }

    pub fn build_texture_atlas_bundles(&mut self, tabs: Vec<STextureAtlasBundle>)-> &Self { 
        for item in tabs {
            let name = item.to_owned().name;
            let tab = build_texture_atlas_bundle(&self.store, item).unwrap();
            self.store.bundles.texture_atlas_bundles.insert(name, tab);
        }
        self
    }

    pub fn build_image_bundles(&mut self, image_bundles: Vec<SImageBundle>) -> &Self { 
        for item in image_bundles {
            let name = item.to_owned().name;
            let ib = build_image_bundle(&self.store, item).unwrap();
            self.store.bundles.image_bundles.insert(name, ib);
        }
        self
    }

    pub fn build_background_bundles(&mut self, background_bundles: Vec<SBackgroundBundle>) -> &Self { 
        for item in background_bundles {
            let name = item.to_owned().name;
            let ib = build_background_bundle(&self.store,item).unwrap();
            self.store.bundles.background_bundles.insert(name, ib);
        }
        self
    }

    pub fn build_clip_bundles(&mut self, clip_bundles: Vec<SClipBundle>) -> &Self { 
        for item in clip_bundles {
            let name = item.to_owned().name;
            let ib = build_clip_bundle(&self.store, item).unwrap();
            self.store.bundles.clip_bundles.insert(name, ib);
        }
        self
    }

    pub fn build_text_box_bundles(&mut self, text_box_bundles: Vec<STextBoxBundle>) -> &Self { 
        for item in text_box_bundles {
            let name = item.to_owned().name;
            let ib = build_text_box_bundle(&self.store,item).unwrap();
            self.store.bundles.text_box_bundles.insert(name, ib);
        }
        self
    }

    pub fn build_element_bundles(&mut self, element_bundles: Vec<SElementBundle>) -> &Self { 
        for item in element_bundles {
            let name = item.to_owned().name;
            let ib = build_element_bundle(&self.store, item).unwrap();
            self.store.bundles.element_bundles.insert(name, ib);
        }
        self
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