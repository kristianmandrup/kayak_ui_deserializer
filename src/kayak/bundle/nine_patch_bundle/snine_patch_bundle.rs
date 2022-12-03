use nanoserde::{DeJson, SerJson, DeRon, SerRon};
use crate::{kayak::{kstyle::skstyle::SKStyle, nine_patch::snine_patch::SNinePatch}};

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct SNinePatchBundle {    
    pub nine_patch: Option<SNinePatch>,
    pub styles: Option<SKStyle>,
    // pub computed_styles: Option<SKStyle>,
    pub name: String,
}
