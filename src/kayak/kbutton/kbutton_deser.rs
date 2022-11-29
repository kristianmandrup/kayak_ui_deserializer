use kayak_ui::{widgets::{KButton}, prelude::{KStyle}};
use crate::{kayak::{store::KayakStore, kstyle::kstyle_deser::KStyleDeser}};

use super::skbutton::SButton;

pub fn deserialize_button(store: &KayakStore, bb: SButton) -> Result<KButton, &'static str>  {
    KButtonDeser::new(store, bb).build().parse()
}

pub struct KButtonDeser<'a> {
    store: &'a KayakStore,
    node: SButton,
}
impl<'a> KButtonDeser<'a> {
    pub fn new(store: &'a KayakStore, node: SButton) -> Self {
        Self {
            store,
            node
        }
    }

    fn styles(&self) -> Option<KStyle> {
        let prop = &self.node.styles.clone();
        if let Some(val) = prop.clone() {
            KStyleDeser::new(val).deserialize().ok()
        } else {
            None
        }        
    }

    pub fn build(&self) -> &Self {
        self.store.extend_kstyle(self.node.styles.to_owned());
        self
    }

    pub fn parse(&self) -> Result<KButton, &'static str> {        
        let styles = self.styles();
        // let widget_name = self.widget_name();
        let mut button = KButton::default();
        // if let Some(val) = styles {
        //     button.user_styles = val;    
        // }
        // bb.widget_name = WidgetName(widget_name);    
        Ok(button)
    }
}