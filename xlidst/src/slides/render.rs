use nannou::{color::Srgb, wgpu::Texture};

pub enum ViewElement {
    Texture {
        texture: Option<Texture>,
        x: f32,
        y: f32,
    },
}

pub struct ViewSlide {
    pub background_color: Srgb<u8>,
    pub elements: Vec<ViewElement>,
}
