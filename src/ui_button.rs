use kayak_ui::{widgets::{KButton, KButtonBundle}, prelude::{KStyle, WidgetName}};

use crate::{ui_kstyle::KStyleBuilder, serialized::{SButton, SButtonBundle}};

// pub struct KButtonBundle {
//     pub button: KButton,
//     pub styles: KStyle,
//     pub on_event: OnEvent,
//     pub widget_name: WidgetName,
// }

// TODO: builder and parser
pub fn build_button(btn: SButton) -> Result<KButton, &'static str>  {
    let mut button = KButton::default();
    if let Some(b) = btn.styles {
        let styles = KStyleBuilder::new(b).parse().unwrap();
        button.user_styles = styles;
        Ok(button)            
    } else {
        Err("bad button")
    }
}

pub fn build_button_bundle(bb: SButtonBundle) -> Result<KButtonBundle, &'static str>  {
    ButtonBundleBuilder::new(bb).parse()
}

pub struct ButtonBundleBuilder {
    node: SButtonBundle
    // pub button: KButton,
    // pub styles: KStyle,
    // pub on_event: OnEvent,
    // pub widget_name: WidgetName,
}
impl ButtonBundleBuilder {
    pub fn new(node: SButtonBundle) -> Self {
        Self {
            node
        }
    }

    fn button(&self) -> Option<KButton> {
        let prop = &self.node.button.clone();
        if let Some(val) = prop.clone() {
            build_button(val).ok()
        } else {
            None
        }        
    }

    fn style(&self) -> Option<KStyle> {
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

    pub fn parse(&self) -> Result<KButtonBundle, &'static str> {        
        let button = self.button();
        let style = self.style();
        let widget_name = self.widget_name();
        let mut bb = KButtonBundle::default();
        if let Some(val) = button {
            bb.button = val;    
        }
        if let Some(val) = style {
            bb.styles = val;    
        }
        bb.widget_name = WidgetName(widget_name);    
        Ok(bb)
    }
}