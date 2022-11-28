pub struct Serializer {
    store: &'a KayakStore,
}
impl<'a> Serializer<'a> {
    pub fn new(store: &'a KayakStore) -> Self {
        Self {
            store,
        }
    }
    
    fn serialize(&self) {
        self.serialize_styles();
    }


    fn serialize_styles(&self) {
        for (key, style) in self.store.styles.iter() {
            
        }
    }
}

