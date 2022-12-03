use kayak_ui::{widgets::{KButton}, prelude::{KStyle}};
use crate::{kayak::{store::KayakStore, kstyle::kstyle_deser::KStyleDeser}};

use super::skbutton::SKButton;

pub fn deserialize_button(store: &KayakStore, bb: SKButton) -> Result<KButton, &'static str>  {
    KButtonDeser::new(store, bb).parse()
}

pub struct KButtonDeser<'a> {
    store: &'a KayakStore,
    node: SKButton,
}
impl<'a> KButtonDeser<'a> {
    pub fn new(store: &'a KayakStore, node: SKButton) -> Self {
        Self {
            store,
            node
        }
    }

    // fn styles(&self) -> Option<KStyle> {
    //     let prop = &self.node.styles.clone();
    //     if let Some(val) = prop.clone() {
    //         KStyleDeser::new(val).deserialize().ok()
    //     } else {
    //         None
    //     }        
    // }

    fn text(&self) -> String {
        let prop = &self.node.text.clone();
        prop.to_owned()
    }

    // pub fn build(&self) -> &Self {
    //     self.store.extend_kstyle(self.node.styles.to_owned());
    //     self
    // }

    pub fn parse(&self) -> Result<KButton, &'static str> {        
        // let styles = self.styles();
        let text = self.text();
        // let widget_name = self.widget_name();

        let mut button = KButton::default();
        button.text = text;
        // if let Some(val) = styles {
        //     button.styles = val;    
        // }
        // bb.widget_name = WidgetName(widget_name);    
        Ok(button)
    }
}