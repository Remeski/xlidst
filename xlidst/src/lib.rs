use nannou::{app::ModelFn, prelude::*};

pub mod elements;
pub mod slides;
mod utils;

use slides::render::{ViewElement, ViewSlide};
pub use xlidst_macros::main;

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
    draw.background().color(current_slide.background_color);
    for element in &current_slide.elements {
        match element {
            ViewElement::Texture { texture, x, y, scale } => {
                if let Some(texture) = texture {
                    draw.scale(*scale).texture(texture).x(*x).y(*y);
                }
            }
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

pub struct Context {
    pub window_pixels: (u32, u32)
}

pub fn get_context(app: &App) -> Context {
    Context {
        window_pixels: app.main_window().inner_size_pixels()
    }
}

pub struct Model {
    pub current_slide: usize,
    pub slides: Vec<ViewSlide>,
}

pub fn start(model: ModelFn<Model>) {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
}
