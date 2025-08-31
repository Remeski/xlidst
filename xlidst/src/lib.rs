use nannou::{app::ModelFn, prelude::*};

pub mod animations;
pub mod elements;
pub mod slides;
mod utils;

// use slides::render::{ViewElement, ViewSlide};
pub use xlidst_macros::main;

use crate::{
    animations::Animation,
    slides::{Element, Slide},
};

fn update(_app: &App, _model: &mut Model, _update: Update) {}

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

fn view(app: &App, model: &Model, frame: Frame) {
    let current_slide = &model.slides[model.current_slide];
    let draw = app.draw();
    draw.background().color(WHITE);
    let elements = current_slide.get_elements();
    for element in elements {
        match element {
            Element::Texture(t) => {
                draw.texture(&t.get_texture().expect("Unable to produce texture"));
            }
            _ => {}
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

pub struct Context {
    pub window_pixels: (u32, u32),
}

pub fn get_context(app: &App) -> Context {
    Context {
        window_pixels: app.main_window().inner_size_pixels(),
    }
}

pub struct Model {
    pub current_slide: usize,
    pub slides: Vec<Slide>,
    pub animations: Vec<Vec<Box<dyn Animation>>>,
    pub animation_index: usize,
}

impl Model {
    pub fn new(app: &App, mut slides: Vec<Slide>) -> Self {
        for s in &mut slides {
            for e in s.get_elements_mut() {
                match e {
                    Element::Texture(t) => {
                        t.as_mut().render_texture(app);
                    }
                    _ => {}
                }
            }
        }
        Self {
            current_slide: 0,
            slides,
            animations: vec![],
            animation_index: 0,
        }
    }
}

pub fn start(model: ModelFn<Model>) {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
}
