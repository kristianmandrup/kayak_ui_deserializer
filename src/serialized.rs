use nanoserde::{DeJson, SerJson, DeRon, SerRon};

use crate::{kayak::{kbutton::skbutton::SKButton, bundle::{text_widget_bundle::stext_widget_bundle::STextWidgetBundle, window_bundle::swindow_bundle::SWindowBundle, texture_atlas_bundle::stexture_atlas_bundle::STextureAtlasBundle, kbutton_bundle::skbutton_bundle::SKButtonBundle, text_box_bundle::stext_box_bundle::STextBoxBundle, nine_patch_bundle::snine_patch_bundle::SNinePatchBundle}}, bevy::bundle::{image_bundle::simage_bundle::SImageBundle, background_bundle::sbackground_bundle::SBackgroundBundle, clip_bundle::sclip_bundle::SClipBundle, element_bundle::selement_bundle::SElementBundle}};
pub type OptStr = Option<String>;

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct SChildren {
    pub widgets: Option<SWidgets>
}

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct SImageAsset {
    pub name: String,
    pub path: String,
}

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct SFontAsset {
    pub name: String,
    pub path: String,
}
#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct SAssets {
    pub images: Option<Vec<SImageAsset>>,
    pub fonts: Option<Vec<SFontAsset>>,
}

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct SVec2 {
    pub x: OptStr,
    pub y: OptStr,
}

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct SWidgets {
    pub buttons: Option<Vec<SKButton>>,
}

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct SBundles {
    pub text_widget_bundles: Option<Vec<STextWidgetBundle>>,
    pub image_bundles: Option<Vec<SImageBundle>>,
    pub window_bundles: Option<Vec<SWindowBundle>>,
    pub texture_atlas_bundles: Option<Vec<STextureAtlasBundle>>,
    pub button_bundles: Option<Vec<SKButtonBundle>>,
    pub background_bundles: Option<Vec<SBackgroundBundle>>,    
    pub clip_bundles: Option<Vec<SClipBundle>>,    
    pub text_box_bundles: Option<Vec<STextBoxBundle>>,        
    pub element_bundles: Option<Vec<SElementBundle>>,        
    pub nine_patch_bundles: Option<Vec<SNinePatchBundle>>,        
}