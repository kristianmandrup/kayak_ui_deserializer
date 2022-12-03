use kayak_ui::widgets::KButton;
use super::skbutton::SKButton;

pub fn serialize_kbutton(bundle: KButton) -> Result<SKButton, &'static str>  {
    KButtonSerializer::new(bundle).serialize()
}

pub struct KButtonSerializer {
    node: KButton,
}
impl KButtonSerializer {
    pub fn new(node: KButton) -> Self {
        Self {
            node
        }
    }

    fn text(&self) -> String {
        let prop = &self.node.text.clone();
        prop.to_owned()
    }

    pub fn serialize(&self) -> Result<SKButton, &'static str> {
        let text = self.text();
        let mut button = SKButton {
            text,
        };
        Ok(button)
    }
}