extern crate nannou;

use nannou::{prelude::*, wgpu::Texture};
use utils::sandbox::Sandbox;

use crate::{sildes::Slideshow, typst::{TypstElement}};

mod utils;
mod typst;
mod sildes;


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

struct Model {
    current_slide: usize,
    slides: Vec<ViewSlide>,
}

enum ViewElement {
    Texture {
        texture: Option<wgpu::Texture>,
        x: f32,
        y: f32
    }
}

struct ViewSlide {
    background_color: rgb::Srgb<u8>,
    elements: Vec<ViewElement>
}

fn model(app: &App) -> Model {
    let mut slideshow = Slideshow::new();

    let math = TypstElement::from("#set page(width: auto, height: auto, fill: none)\n$f(x)$");
    slideshow.slide();
    slideshow.add(math);
    slideshow.slide();

    let math2 = TypstElement::from("#set page(width: auto, height: auto, fill: none)\n$f(x) = x$");
    let mut math3 = TypstElement::from("#set page(width: auto, height: auto, fill: none)\n$f'(x) = 1$");
    math3.set_y(100.0);

    slideshow.add(math2);
    slideshow.add(math3);


    let slides = slideshow.slides().map(|slide| -> ViewSlide {
        let elements = slide.get_elements();
        let mut view_elements: Vec<ViewElement> = Vec::new();
        for element in elements {
            match element {
                sildes::Element::Root(_) => {},
                sildes::Element::Texture(t) => {
                    view_elements.push(ViewElement::Texture { 
                        texture: Some(t.to_texture(app).unwrap()), 
                        x: t.get_x(), 
                        y: t.get_y() 
                    });
                }
            }
        }
        ViewSlide {
            background_color: WHITE,
            elements: view_elements
        }
    }).collect();

    Model { current_slide: 0, slides }
}

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
}

