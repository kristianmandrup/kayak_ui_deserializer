use nanoserde::{DeJson, SerJson, DeRon, SerRon};

use crate::{serialized::OptStr, bevy::{ui_rect::sui_rect::SUiRect, size::ssize::SSize}};

#[derive(DeJson, SerJson, DeRon, SerRon, Clone)]

pub struct SBevyStyle {
    pub display: OptStr,
    pub extends: OptStr,
    /// Whether to arrange this node relative to other nodes, or positioned absolutely
    pub position_type: OptStr,
    pub direction: OptStr,
    pub flex_direction: OptStr,
    pub flex_wrap: OptStr,
    pub align_items: OptStr,
    pub align_self: OptStr,
    pub align_content: OptStr,
    pub justify_content: OptStr,
    pub position_obj: Option<SUiRect>,
    pub margin_obj: Option<SUiRect>,
    pub padding_obj: Option<SUiRect>,
    pub border_obj: Option<SUiRect>,
    pub position: OptStr,
    pub margin: OptStr,
    pub padding: OptStr,
    pub border: OptStr,
    pub flex_grow: OptStr,
    pub flex_shrink: OptStr,
    pub flex_basis: OptStr,    
    pub size_obj: Option<SSize>,    
    pub size: OptStr,
    pub min_size_obj: Option<SSize>,        
    pub max_size_obj: Option<SSize>,        
    pub min_size: OptStr,        
    pub max_size: OptStr,        
    pub aspect_ratio: OptStr,    
    pub overflow: OptStr,                
}