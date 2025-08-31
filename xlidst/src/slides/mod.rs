use nannou::{App, wgpu::Texture};

use crate::{
    animations::{Animatable, Animation},
    slides::render::{ViewElement, ViewSlide},
};

pub mod render;

pub trait AsTexture {
    fn get_texture(&self) -> Option<Texture>;
    fn render_texture(&mut self, app: &App) -> Option<Texture>;
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
    animations: Vec<Box<dyn Animation>>,
}

impl Slide {
    pub fn new() -> Slide {
        Slide {
            elements: vec![Element::Root(Box::from(RootElement { children: None }))],
            animations: vec![],
        }
    }

    pub fn get_elements_mut(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }

    pub fn get_elements(&self) -> &Vec<Element> {
        &self.elements
    }

    pub fn animate(&mut self, animation: Box<dyn Animation>) {
        self.animations.push(animation);
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

    pub fn animate(&mut self, animation: Box<dyn Animation>) {
        let mut last = self.slides.pop().expect("Slide not initialized.");
        last.animate(animation);
        self.slides.push(last);
    }

    pub fn get_slides(self) -> Vec<Slide> {
        self.slides
    }

}
