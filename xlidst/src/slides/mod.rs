use nannou::{wgpu::Texture, App};

// use crate::utils::sandbox::Sandbox;

pub trait AsTexture {
    fn to_texture(&self, app: &App) -> Option<Texture>;
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
}

pub trait AsRoot {
    fn get_children(&self) -> &Option<Vec<Element>>;
}

pub enum Element {
    Root(Box<dyn AsRoot>),
    Texture(Box<dyn AsTexture>),
}

struct RootElement {
    children: Option<Vec<Element>>,
}

impl AsRoot for RootElement {
    fn get_children(&self) -> &Option<Vec<Element>> {
        &self.children
    }
}

pub struct Slide {
    elements: Vec<Element>
}

pub struct Slideshow {
    slides: Vec<Slide>
}

impl Slide {
    pub fn new() -> Slide {
        Slide {
            elements: vec![Element::Root(Box::from(RootElement { children: None }))]
        }
    }
    pub fn get_elements(&self) -> &Vec<Element> {
        &self.elements
    }
}

pub trait ToElement {
    fn to(self) -> Element;
}

impl Slideshow {
    pub fn new() -> Slideshow {
        Slideshow {
            slides: Vec::new()
        }
    }

    pub fn slide(&mut self) {
        let slide = Slide::new();
        self.slides.push(slide)
    }

    pub fn add(&mut self, element: impl ToElement) {
        // this probably isn't the smartest way to do this
        let mut last = self.slides.pop().expect("Slide not initilized.");
        last.elements.push(element.to());
        self.slides.push(last);
    }

    pub fn slides(&self) -> core::slice::Iter<'_, Slide> {
        self.slides.iter()
    }
}

