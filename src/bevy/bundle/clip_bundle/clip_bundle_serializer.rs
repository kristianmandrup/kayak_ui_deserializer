use kayak_ui::widgets::ClipBundle;

use crate::kayak::store::KayakStore;

pub struct ClipBundleSerializer<'a> {
    store: &'a KayakStore,
    node: ClipBundle,
}
impl<'a> ClipBundleSerializer<'a> {
    pub fn new(store: &'a KayakStore, node: ClipBundle) -> Self {
        Self {
            store,            
            node
        }
    }
}