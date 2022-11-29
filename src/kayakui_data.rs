use nanoserde::{DeJson, SerJson, DeRon, SerRon};

use crate::{serialized::{SAssets, SWidgets, SBundles}, kayak::kstyle::skstyle::SKStyle, bevy::style::sstyle::SBevyStyle};

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]
pub struct KayakUiData {
    pub assets: Option<SAssets>,
    pub kstyles: Option<Vec<SKStyle>>,
    pub styles: Option<Vec<SBevyStyle>>,
    pub widgets: Option<SWidgets>,
    pub bundles: Option<SBundles>,
}
impl Default for KayakUiData {
    fn default() -> Self {
        Self {
            assets: None,
            kstyles: None,
            styles: None,
            widgets: None,
            bundles: None
        }
    }
}
