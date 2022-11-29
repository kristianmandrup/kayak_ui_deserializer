use nanoserde::{DeJson, SerJson, DeRon, SerRon};
use crate::{bevy::{image::simage::SImage, style::sstyle::SBevyStyle, size::ssize::SSize, transform::stransform::STransform}, serialized::OptStr};

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct SImageBundle {
    pub name: String,
    pub image: Option<SImage>,
    pub style: Option<SBevyStyle>,        
    pub image_mode: OptStr,
    pub calculated_size: Option<SSize>,
    // pub struct BackgroundColor(pub Color);
    pub background_color: OptStr,
    pub focus_policy: OptStr,
    pub transform: Option<STransform>,
    pub visibility: OptStr,
    pub computed_visibility: OptStr,
}  
