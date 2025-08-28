use nannou::{App, wgpu::Texture};

use crate::slides::render::{ViewElement, ViewSlide};

pub mod render;

pub trait AsTexture {
    fn to_texture(&self, app: &App) -> Option<Texture>;
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
    fn get_scale(&self) -> f32;
}

pub trait AsRoot {
    fn get_children(&self) -> &Option<Vec<Element>>;
}

pub trait ToElement {
    fn to(self) -> Element;
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
    elements: Vec<Element>,
}

impl Slide {
    pub fn new() -> Slide {
        Slide {
            elements: vec![Element::Root(Box::from(RootElement { children: None }))],
        }
    }
    pub fn get_elements(&self) -> &Vec<Element> {
        &self.elements
    }
}

pub struct Slideshow {
    slides: Vec<Slide>,
}

impl Slideshow {
    pub fn new() -> Slideshow {
        Slideshow { slides: Vec::new() }
    }

    pub fn slide(&mut self) {
        let slide = Slide::new();
        self.slides.push(slide)
    }

    pub fn add(&mut self, element: impl ToElement) {
        // this probably isn't the smartest way to do this
        let mut last = self.slides.pop().expect("Slide not initialized.");
        last.elements.push(element.to());
        self.slides.push(last);
    }

    pub fn slides(&self) -> core::slice::Iter<'_, Slide> {
        self.slides.iter()
    }

    pub fn to_view_slides(&self, app: &App) -> Vec<ViewSlide> {
        self.slides()
            .map(|slide| -> ViewSlide {
                let elements = slide.get_elements();
                let mut view_elements: Vec<ViewElement> = Vec::new();
                for element in elements {
                    match element {
                        Element::Root(_) => {}
                        Element::Texture(t) => {
                            view_elements.push(ViewElement::Texture {
                                texture: Some(t.to_texture(app).unwrap()),
                                x: t.get_x(),
                                y: t.get_y(),
                                scale: t.get_scale()
                            });
                        }
                    }
                }
                ViewSlide {
                    background_color: nannou::prelude::WHITE,
                    elements: view_elements,
                }
            })
            .collect()
    }
}
