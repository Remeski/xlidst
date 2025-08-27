extern crate nannou;

extern crate xlidst_macros;

use nannou::{app::ModelFn, prelude::*};

mod utils;
pub mod typst;
pub mod slides;

pub use xlidst_macros::main as main;

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
    for element in &current_slide.elements {
        match element {
            ViewElement::Texture { texture, x, y } => {
                if let Some(texture) = texture {
                        draw.texture(texture).x(*x).y(*y);
                }
            }
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

pub struct Model {
    pub current_slide: usize,
    pub slides: Vec<ViewSlide>,
}

pub enum ViewElement {
    Texture {
        texture: Option<wgpu::Texture>,
        x: f32,
        y: f32
    }
}

pub struct ViewSlide {
    pub background_color: rgb::Srgb<u8>,
    pub elements: Vec<ViewElement>
}

// fn model(app: &App) -> Model {
//     let mut slideshow = Slideshow::new();
//
//     let math = TypstElement::from("#set page(width: auto, height: auto, fill: none)\n$f(x)$");
//     slideshow.slide();
//     slideshow.add(math);
//     slideshow.slide();
//
//     let math2 = TypstElement::from("#set page(width: auto, height: auto, fill: none)\n$f(x) = x$");
//     let mut math3 = TypstElement::from("#set page(width: auto, height: auto, fill: none)\n$f'(x) = 1$");
//     math3.set_y(100.0);
//
//     slideshow.add(math2);
//     slideshow.add(math3);
//
//
// }

pub fn start(model: ModelFn<Model>) {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
}

