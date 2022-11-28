use kayak_ui::{widgets::{KButton}, prelude::{KStyle}};
use crate::{serialized::{SButton}, kayak_store::KayakStore, kstyle::ui_kstyle::KStyleBuilder};

pub fn build_button(store: &KayakStore, bb: SButton) -> Result<KButton, &'static str>  {
    ButtonBuilder::new(store, bb).build().parse()
}

pub struct ButtonBuilder<'a> {
    store: &'a KayakStore,
    node: SButton,
}
impl<'a> ButtonBuilder<'a> {
    pub fn new(store: &'a KayakStore, node: SButton) -> Self {
        Self {
            store,
            node
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

    pub fn build(&self) -> &Self {
        self.store.extend_kstyle(self.node.styles.to_owned());
        self
    }

    pub fn parse(&self) -> Result<KButton, &'static str> {        
        let styles = self.styles();
        // let widget_name = self.widget_name();
        let mut button = KButton::default();
        if let Some(val) = styles {
            button.user_styles = val;    
        }
        // bb.widget_name = WidgetName(widget_name);    
        Ok(button)
    }
}