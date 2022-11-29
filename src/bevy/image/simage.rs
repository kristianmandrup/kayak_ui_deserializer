use nanoserde::{DeJson, SerJson, DeRon, SerRon};
use crate::serialized::OptStr;

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct SImage {
    pub path: OptStr,
    pub ref_id: OptStr,
}    
