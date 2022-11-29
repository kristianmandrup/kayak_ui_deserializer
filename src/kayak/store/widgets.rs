use std::collections::HashMap;

use kayak_ui::widgets::KButton;

pub struct StoredWidgets {
    pub buttons: HashMap<String, KButton>,
}
impl StoredWidgets {
    pub fn new() -> Self {
        Self {
            buttons: HashMap::new(),
        }                    
    }

    pub fn button(&self, id: &str) -> Option<&KButton> {
        self.buttons.get(id)
    }    
}
