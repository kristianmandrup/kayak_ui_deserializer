use kayak_ui::widgets::NinePatchBundle;

use crate::kayak::{nine_patch::{snine_patch::SNinePatch, nine_patch_serializer::serialize_nine_patch}, kstyle::{skstyle::SKStyle, kstyle_serializer::serialize_kstyle}};

use super::snine_patch_bundle::SNinePatchBundle;

pub fn serialize_nine_patch_bundle(bundle: NinePatchBundle) -> Result<SNinePatchBundle, &'static str>  {
    NinePatchBundleSerializer::new(bundle).serialize()
}


pub struct NinePatchBundleSerializer {
    node: NinePatchBundle,
}
impl NinePatchBundleSerializer {
    pub fn new(node: NinePatchBundle) -> Self {
        Self {
            node
        }
    }

    fn name(&self) -> String {
        let prop = &self.node.widget_name.clone();
        prop.to_owned().into()
    }

    fn nine_patch(&self) -> Option<SNinePatch> {
        let style = &self.node.nine_patch.clone();
        serialize_nine_patch(style.to_owned()).ok()
    }

    fn styles(&self) -> Option<SKStyle> {
        let style = &self.node.styles.clone();
        serialize_kstyle(style.to_owned()).ok()
    }

    fn computed_styles(&self) -> Option<SKStyle> {
        let style = &self.node.computed_styles.clone();
        serialize_kstyle(style.0.to_owned()).ok()
    }    

    pub fn serialize(&self) -> Result<SNinePatchBundle, &'static str> {
        let name = self.name();
        let nine_patch = self.nine_patch();
        let styles = self.styles();
        // let computed_styles = self.computed_styles();
        
        let bundle = SNinePatchBundle {
            styles,
            nine_patch,
            // computed_styles,
            name
        };
        Ok(bundle)
    }
}