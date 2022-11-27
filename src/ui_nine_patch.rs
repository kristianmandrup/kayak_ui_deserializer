// pub struct NinePatch {
//     /// The handle to nine_patch
//     pub handle: Handle<NinePatch>,
//     /// The size of each edge (in pixels)
//     pub border: Edge<f32>,
// }


// pub struct NinePatchBundle {
//     pub nine_patch: NinePatch,
//     pub styles: KStyle,
//     pub children: KChildren,
//     pub on_event: OnEvent,
//     pub widget_name: WidgetName,
// }

use bevy::prelude::{ Handle, Image};
use kayak_ui::{widgets::NinePatch, prelude::Edge};

use crate::{kayak_store::KayakStore, serialized::SNinePatch, ui_image::ImageBuilder, ui_edge::EdgeBuilder};

pub fn build_nine_patch(store: &KayakStore, ib: SNinePatch) -> Result<NinePatch, &'static str>  {
    NinePatchBuilder::new(store, ib).parse()
}

pub struct NinePatchBuilder<'a> {
    store: &'a KayakStore,
    node: SNinePatch,
}
impl<'a> NinePatchBuilder<'a> {
    pub fn new(store: &'a KayakStore, node: SNinePatch) -> Self {
        Self {
            store,
            node
        }
    }

    fn handle(&self) -> Option<Handle<Image>> {
        let prop = &self.node.image.clone();
        if let Some(img) = prop.to_owned() {
            ImageBuilder::new(self.store, img).parse_handle().ok()
        } else {
            None
        }        
    }    

    fn border(&self) -> Option<Edge<f32>> {
        if let Some(val) = self.node.border.clone() {
            let edge = val.clone();
            EdgeBuilder::create_from_str(edge).parse().ok()    
        } else {
            None
        }
    }

    pub fn parse(&self) -> Result<NinePatch, &'static str> {            
        let handle = self.handle();         
        let border = self.border();         
        let mut nine_patch = NinePatch::default();
        if let Some(val) = handle {
            nine_patch.handle = val.into();    
        }
        if let Some(val) = border {
            nine_patch.border = val.into();    
        }
        Ok(nine_patch)
    }
}