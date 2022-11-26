use bevy::{prelude::{AssetServer, Handle, Image, ImageBundle}, ui::{UiImage, Style, FocusPolicy}};
use kayak_ui::{widgets::KImage};

use crate::{ui_parser::Conv, ui_bevy_style::BevyStyleBuilder, serialized::{SImage, SImageBundle}};


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

    fn style(&self) -> Option<Style> {
        let prop = &self.node.styles.clone();
        BevyStyleBuilder::new(prop.to_owned()).parse().ok()
    }

    fn focus_policy(&self) -> Option<FocusPolicy> {
        let prop = &self.node.focus_policy.clone();
        if let Some(val) = prop {
            let fp = match val.as_str() {
                "block" => FocusPolicy::Block,
                "pass" => FocusPolicy::Pass,
                _ => FocusPolicy::Block
            };
            Some(fp)
        } else {
            None
        }            
    }

    pub fn parse(&self) -> Result<ImageBundle, &'static str> {                        
        let image = self.image();
        let style = self.style();
        let focus_policy = self.focus_policy();
        // let widget_name = self.widget_name();
        // let children = self.children();
        let mut image_bundle = ImageBundle::default();
        if let Some(val) = image {
            image_bundle.image = val;    
        }
        if let Some(val) = style {
            image_bundle.style = val;    
        }
        if let Some(val) = focus_policy {
            image_bundle.focus_policy = val;    
        }
        // image_bundle.widget_name = widget_name;
        Ok(image_bundle)       
    }    
}
// pub image_mode: ImageMode,
// /// The calculated size based on the given image
// pub calculated_size: CalculatedSize,
// /// The background color, which serves as a "fill" for this node
// ///
// /// When combined with `UiImage`, tints the provided image.
// pub background_color: BackgroundColor,
// /// The image of the node
// pub image: UiImage,
// /// Whether this node should block interaction with lower nodes
// pub focus_policy: FocusPolicy,
// /// The transform of the node
// ///
// /// This field is automatically managed by the UI layout system.
// /// To alter the position of the `NodeBundle`, use the properties of the [`Style`] component.
// pub transform: Transform,
// /// The global transform of the node
// ///
// /// This field is automatically managed by the UI layout system.
// /// To alter the position of the `NodeBundle`, use the properties of the [`Style`] component.
// pub global_transform: GlobalTransform,
// /// Describes the visibility properties of the node
// pub visibility: Visibility,
// /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
// pub computed_visibility: ComputedVisibility,
// /// Indicates the depth at which the node should appear in the UI
// pub z_index: ZIndex,