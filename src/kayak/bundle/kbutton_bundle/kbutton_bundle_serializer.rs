use kayak_ui::widgets::KButtonBundle;

use crate::kayak::{kbutton::{skbutton::SKButton, kbutton_serializer::serialize_kbutton}, kstyle::{skstyle::SKStyle, kstyle_serializer::serialize_kstyle}};

use super::skbutton_bundle::SKButtonBundle;

pub fn serialize_kbutton_bundle(bundle: KButtonBundle) -> Result<SKButtonBundle, &'static str>  {
    KButtonBundleSerializer::new(bundle).serialize()
}


pub struct KButtonBundleSerializer {
    node: KButtonBundle,
}
impl KButtonBundleSerializer {
    pub fn new(node: KButtonBundle) -> Self {
        Self {
            node
        }
    }

    fn name(&self) -> String {
        let prop = &self.node.widget_name.clone();
        prop.to_owned().into()
    }

    fn button(&self) -> Option<SKButton> {
        let style = &self.node.button.clone();
        serialize_kbutton(style.to_owned()).ok()
    }

    fn styles(&self) -> Option<SKStyle> {
        let style = &self.node.styles.clone();
        serialize_kstyle(style.to_owned()).ok()
    }

    fn computed_styles(&self) -> Option<SKStyle> {
        let style = &self.node.computed_styles.clone();
        serialize_kstyle(style.0.to_owned()).ok()
    }    

    pub fn serialize(&self) -> Result<SKButtonBundle, &'static str> {
        let name = self.name();
        let button = self.button();
        let styles = self.styles();
        let computed_styles = self.computed_styles();
        
        let bundle = SKButtonBundle {
            button,
            styles,
            computed_styles,
            name
        };
        Ok(bundle)
    }
}