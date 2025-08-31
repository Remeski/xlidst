use nannou::{color::Srgb, wgpu::Texture};

use crate::animations::Animation;

pub enum ViewElement {
    Texture {
        texture: Option<Texture>,
        x: f32,
        y: f32,
        scale: f32,
    },
}

pub struct ViewSlide {
    pub background_color: Srgb<u8>,
    pub elements: Vec<ViewElement>,
    pub animations: Vec<Box<dyn Animation>>
}
