extern crate nannou;

use nannou::{prelude::*, wgpu::Texture};
use utils::sandbox::Sandbox;

use crate::typst::TypstElement;

mod utils;
mod typst;

pub trait ToTexture {
    fn to_texture(&self, app: &App, sandbox: &Sandbox) -> Texture;
}

struct SourceSlide {
    background_color: rgb::Srgb<u8>,
    sources: Vec<Box<dyn ToTexture>>,
}

impl SourceSlide {
    fn parse(&self, app: &App, sandbox: &Sandbox) -> Slide {
        let textures = self.sources.iter().map(|source| {
            return source.to_texture(app, sandbox);
        }).collect::<Vec<Texture>>();

        Slide {
            background_color: self.background_color,
            textures
        }
    }
}

struct Slide {
    background_color: rgb::Srgb<u8>,
    textures: Vec<wgpu::Texture>
}

struct Model {
    current_slide: usize,
    slides: Vec<Slide>,
}

fn model(app: &App) -> Model {
    let sandbox = utils::sandbox::Sandbox::new();

    let source_slides = vec![
        SourceSlide{
            background_color: PURPLE,
            sources: vec![Box::from(TypstElement::from("#set page(width: 100pt, height: 100pt)\n= NAPS"))]
        },
        SourceSlide{
            background_color: RED,
            sources: vec![Box::from(TypstElement::from("#set page(width: 100pt, height: 100pt)\n== Euler\n$e^(i pi) + 1 = 0$"))]
        },
        SourceSlide{
            background_color: BLUE,
            sources: vec![Box::from(TypstElement::from("#set page(width: 100pt, height: 100pt)\n== Hemo munk"))]
        }];

    let slides = source_slides.iter().map(|s| s.parse(app, &sandbox)).collect();

    Model {
        current_slide: 0,
        slides: slides,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn event(_app: &App, model: &mut Model, event: Event) {
    if let Event::WindowEvent { id: _, simple } = event {
        if let Some(KeyPressed(k)) = simple {
            let slides_length = model.slides.len();
            match k {
                Key::Right => {
                    model.current_slide = (slides_length - 1).min(model.current_slide + 1);
                }
                Key::Left => {
                    model.current_slide = 0.max(model.current_slide as isize - 1) as usize;
                }
                _ => {}
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame){
    let current_slide = &model.slides[model.current_slide];
    let draw = app.draw();
    draw.background().color(current_slide.background_color);
    for texture in &current_slide.textures {
        draw.texture(texture);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
}

