use kayak_ui::{widgets::{KButton, KButtonBundle}, prelude::{KStyle, WidgetName}};

use crate::{serialized::{SButtonBundle}, kayak_store::KayakStore, ui_button::build_button, kstyle::ui_kstyle::KStyleBuilder};

pub fn build_button_bundle(store: &KayakStore, bb: SButtonBundle) -> Result<KButtonBundle, &'static str>  {
    ButtonBundleBuilder::new(store, bb).build().parse()
}

pub struct ButtonBundleBuilder<'a> {
    store: &'a KayakStore,
    node: SButtonBundle,
}
impl<'a> ButtonBundleBuilder<'a> {
    pub fn new(store: &'a KayakStore, node: SButtonBundle) -> Self {
        Self {
            store,
            node
        }
    }

    fn button(&self) -> Option<KButton> {
        let prop = &self.node.button.clone();
        if let Some(val) = prop.clone() {
            build_button(self.store, val).ok()
        } else {
            None
        }        
    }

    fn styles(&self) -> Option<KStyle> {
        let prop = &self.node.styles.clone();
        if let Some(val) = prop.clone() {
            KStyleBuilder::new(val).parse().ok()
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

    pub fn parse(&self) -> Result<KButtonBundle, &'static str> {        
        let button = self.button();
        let styles = self.styles();
        let widget_name = self.widget_name();
        let mut bb = KButtonBundle::default();
        if let Some(val) = button {
            bb.button = val;    
        }
        if let Some(val) = styles {
            bb.styles = val;    
        }
        bb.widget_name = WidgetName(widget_name);    
        Ok(bb)
    }
}