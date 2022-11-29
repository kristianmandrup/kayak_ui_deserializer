use kayak_ui::widgets::BackgroundBundle;

use crate::kayak::store::KayakStore;

pub struct BackgroundBundleSerializer<'a> {
    store: &'a KayakStore,
    node: BackgroundBundle,
}
impl<'a> BackgroundBundleSerializer<'a> {
    pub fn new(store: &'a KayakStore, node: BackgroundBundle) -> Self {
        Self {
            store,
            node
        }
    }
}
