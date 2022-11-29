use kayak_ui::widgets::KButton;

use crate::kayak::store::KayakStore;

pub struct KButtonSerializer<'a> {
    store: &'a KayakStore,
    node: KButton,
}
impl<'a> KButtonSerializer<'a> {
    pub fn new(store: &'a KayakStore, node: KButton) -> Self {
        Self {
            store,
            node
        }
    }
}