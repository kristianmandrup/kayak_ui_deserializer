use bevy::{prelude::{Handle, Image, AssetServer}, ui::{UiImage}};
use kayak_ui::{widgets::KImage};

use crate::{ui_parser::Conv, serialized::{SImage}, kayak_store::KayakStore};

pub fn build_image(store: &KayakStore, ib: SImage) -> Result<KImage, &'static str>  {
    ImageBuilder::new(store, ib).parse()
}

pub fn build_image_ui(store: &KayakStore, ib: SImage) -> Result<UiImage, &'static str>  {
    ImageBuilder::new(store, ib).parse_ui()
}

pub struct ImageBuilder<'a> {
    store: &'a KayakStore,
    node: SImage,
}
impl<'a> ImageBuilder<'a> {
    pub fn new(store: &'a KayakStore, node: SImage) -> Self {
        Self {
            store,
            node
        }
    }

    fn image_path(&self) -> Option<String> {
        let prop = &self.node.path.clone();
        Conv::get_prop(prop)
    }    

    fn asset_server(&self) -> AssetServer {
        self.store.asset_server.clone()
    }

    fn image_ref(&self) -> Option<Handle<Image>> {
        let prop = &self.node.ref_id.clone();
        if let Some(img_id) = prop {
            let opt_image = self.store.assets.image(img_id);
            if let Some(img_handle) = opt_image {
                Some(img_handle.to_owned())
            } else {
                None
            }            
        } else {
            None
        }
    }

    fn image(&self) -> Option<Handle<Image>> {
        if let Some(img_path) = self.image_path() {
            let image = self.asset_server().load(img_path);
            Some(image)
        } else {
            if let Some(img) = self.image_ref() {
                Some(img)
            } else {
                None
            }            
        }
    }

    pub fn parse(&self) -> Result<KImage, &'static str> {            
        let image = self.image().unwrap();         
        let kimage = KImage(image);
        Ok(kimage)
    }

    pub fn parse_ui(&self) -> Result<UiImage, &'static str> {            
        let image = self.image().unwrap();         
        let ui_image = UiImage(image);
        Ok(ui_image)
    }
}
