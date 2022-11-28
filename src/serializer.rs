use crate::{kayak_store::KayakStore, serialized::KayakUiData, ui_bevy_style::serializer::BevyStyleSerializer};

pub struct Serializer<'a> {
    store: &'a KayakStore,
    data: KayakUiData
}
impl<'a> Serializer<'a> {
    pub fn new(store: &'a KayakStore) -> Self {
        Self {
            store,
            data: KayakUiData::default()
        }
    }
    
    fn serialize(&self) {
        self.serialize_styles();
    }


    fn serialize_styles(&self) {
        for (_key, style) in self.store.styles.iter() {
            let sstyle = BevyStyleSerializer::new(style.to_owned()).serialize();
            let optvec = self.data.styles.to_owned();
            if let Some(mut vec) = optvec {
                vec.push(sstyle.unwrap());
            }
            
        }
    }
}

