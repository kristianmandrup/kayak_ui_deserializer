use kayak_ui::widgets::KButtonBundle;

use crate::kayak::store::KayakStore;

pub struct KButtonBundleSerializer<'a> {
    store: &'a KayakStore,
    node: KButtonBundle,
}
impl<'a> KButtonBundleSerializer<'a> {
    pub fn new(store: &'a KayakStore, node: KButtonBundle) -> Self {
        Self {
            store,
            node
        }
    }
}