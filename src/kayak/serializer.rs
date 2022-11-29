use crate::{kayakui_data::KayakUiData, bevy::style::style_serializer::BevyStyleSerializer};

use super::{kstyle::kstyle_serializer::KStyleSerializer, store::KayakStore};

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
        self.serialize_kstyles();
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

    fn serialize_kstyles(&self) {
        for (_key, kstyle) in self.store.kstyles.iter() {
            let kstyle = KStyleSerializer::new(kstyle.to_owned()).serialize();
            let optvec = self.data.kstyles.to_owned();
            if let Some(mut vec) = optvec {
                vec.push(kstyle.unwrap());
            }
            
        }
    }
}

