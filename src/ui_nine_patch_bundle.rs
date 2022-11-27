// pub struct NinePatch {
//     /// The handle to image
//     pub handle: Handle<Image>,
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

use kayak_ui::{widgets::{NinePatch, NinePatchBundle}, prelude::{KStyle, WidgetName}};

use crate::{ui_kstyle::KStyleBuilder, serialized::SNinePatchBundle, kayak_store::KayakStore, ui_nine_patch::NinePatchBuilder};


pub fn build_nine_patch_bundle(store: &KayakStore, bb: SNinePatchBundle) -> Result<NinePatchBundle, &'static str>  {
    NinePatchBundleBuilder::new(store, bb).build().parse()
}

pub struct NinePatchBundleBuilder<'a> {
    store: &'a KayakStore,
    node: SNinePatchBundle,
}
impl<'a> NinePatchBundleBuilder<'a> {
    pub fn new(store: &'a KayakStore, node: SNinePatchBundle) -> Self {
        Self {
            store,
            node
        }
    }

    fn nine_patch(&self) -> Option<NinePatch> {
        let prop = &self.node.nine_patch.clone();
        if let Some(val) = prop.to_owned() {
            NinePatchBuilder::new(self.store, val).parse().ok()
        } else {
            None
        }
        
    }

    fn styles(&self) -> Option<KStyle> {
        let prop = &self.node.styles.clone();
        if let Some(val) = prop {
            KStyleBuilder::new(val.to_owned()).parse().ok()
        } else {
            None
        }        
    }

    fn widget_name(&self) -> String {
        let prop = &self.node.name.clone();
        prop.to_owned()
    }

    pub fn build(&self) -> &Self {
        self.store.extend_kstyle(self.node.styles.to_owned());
        self
    }

    pub fn parse(&self) -> Result<NinePatchBundle, &'static str> {                        
        let nine_patch = self.nine_patch();
        let styles = self.styles();
        let name = self.widget_name();
        // let children = self.children();
        let mut np_bundle = NinePatchBundle::default();
        if let Some(val) = nine_patch {
            np_bundle.nine_patch = val;    
        }
        if let Some(val) = styles {
            np_bundle.styles = val;    
        }
        np_bundle.widget_name = WidgetName(name);            
        Ok(np_bundle)       
    }    
}