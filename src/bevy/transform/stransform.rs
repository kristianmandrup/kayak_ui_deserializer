use nanoserde::{DeJson, SerJson, DeRon, SerRon};

use crate::serialized::OptStr;

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct STransform {
    pub translation: Option<SVec3>, 
    pub rotation: Option<SQuat>, 
    pub scale: Option<SVec3>
}

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct SVec3 {
    pub x: OptStr,
    pub y: OptStr,
    pub z: OptStr,
}

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct SQuat {
    pub x: OptStr, // f32,
    pub y: OptStr,
    pub z: OptStr,
    pub w: OptStr,
}
