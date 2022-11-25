use bevy::{prelude::{AssetServer, Handle, Image, ImageBundle}, ui::{UiImage, Style}};
use kayak_ui::{widgets::KImage, prelude::{KStyle, WidgetName}};

use crate::{json_deserializer::{SImageBundle, SImage}, ui_parser::Conv, ui_kstyle::KStyleBuilder};


// pub struct KImage(pub Handle<bevy::prelude::Image>);
// pub struct KImageBundle {
//     pub image: KImage,
//     pub styles: KStyle,
//     pub widget_name: WidgetName,
// }

struct ImageBuilder {
    pub asset_server: AssetServer,
    node: SImage,
}
impl ImageBuilder {
    pub fn new(asset_server: AssetServer, node: SImage) -> Self {
        Self {
            asset_server,
            node
        }
    }

    fn image_path(&self) -> Option<String> {
        let prop = &self.node.path.clone();
        Conv::get_prop(prop)
    }    

    fn image(&self) -> Option<Handle<Image>> {
        if let Some(img_path) = self.image_path() {
            let image = self.asset_server.load(img_path);
            Some(image)
        } else {
            None
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


pub fn build_image_bundle(asset_server: AssetServer, ib: SImageBundle) -> Result<ImageBundle, &'static str>  {
    ImageBundleBuilder::new(asset_server, ib).parse()
}

pub struct ImageBundleBuilder {
    asset_server: AssetServer,
    node: SImageBundle,
}
impl ImageBundleBuilder {
    pub fn new(asset_server: AssetServer, node: SImageBundle) -> Self {
        Self {
            asset_server,
            node
        }
    }

    fn image(&self) -> Option<UiImage> {
        let prop = &self.node.image.clone();
        ImageBuilder::new(self.asset_server.clone(), prop.to_owned()).parse_ui().ok()
    }

    // fn style(&self) -> Option<Style> {
    //     let prop = &self.node.style.clone();
    //     BevyStyleBuilder::new(prop.to_owned()).parse().ok()
    // }

    fn widget_name(&self) -> String {
        let prop = &self.node.name.clone();
        prop.to_owned()
    }

    pub fn parse(&self) -> Result<ImageBundle, &'static str> {                        
        let image = self.image();
        // let style = self.style();
        let name = self.widget_name();
        // let children = self.children();
        let mut image_bundle = ImageBundle::default();
        if let Some(val) = image {
            image_bundle.image = val;    
        }
        // if let Some(val) = style {
        //     image_bundle.style = val;    
        // }

        Ok(image_bundle)       
    }    
}