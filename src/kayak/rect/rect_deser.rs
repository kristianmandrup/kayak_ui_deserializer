pub fn deserialize_rect(rect: SRect) -> Result<Rect, &'static str>  {
    RectDeser::new(rect).deserialize()
}


pub struct RectDeser {
    node: SRect,
}
impl RectDeser {
    pub fn new(node: SRect) -> Self {
        Self {
            node
        }
    }

    fn to_f32(&self, prop: &Option<String>) -> Option<f32> {
        if let Some(str) = Conv::get_prop(prop) {
            Conv(str).to_f32()
        } else {
            None
        }                    
    }

    fn posx(&self) -> Option<f32> {
        self.to_f32(&self.node.posx.clone())
    }

    fn posy(&self) -> Option<f32> {
        self.to_f32(&self.node.posy.clone())
    }

    fn width(&self) -> Option<f32> {
        self.to_f32(&self.node.width.clone())
    }

    fn height(&self) -> Option<f32> {
        self.to_f32(&self.node.height.clone())
    }

    fn z_index(&self) -> Option<f32> {
        self.to_f32(&self.node.z_index.clone())
    }

    pub fn deserialize(&self) -> Result<Rect, &'static str> {        
        let posx = self.posx();
        let posy = self.posy();
        let width = self.width();
        let height = self.height();
        let z_index = self.z_index();
        let mut rect = Rect::default();
        if let Some(val) = posx {
            rect.posx = val;    
        }
        if let Some(val) = posy {
            rect.posy = val;    
        }
        if let Some(val) = width {
            rect.width = val;    
        }
        if let Some(val) = height {
            rect.height = val;    
        }
        if let Some(val) = z_index {
            rect.z_index = val;    
        }
        Ok(rect)            
    }
    // top: node.top
}