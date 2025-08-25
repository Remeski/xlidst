extern crate nannou;
use nannou::{prelude::*};

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
}

struct Slide {
    color: rgb::Srgb<u8>
}

struct Model {
    current_slide: usize,
    slides: Vec<Slide>
}

fn model(_app: &App) -> Model {
    let slides = vec![Slide{color: PURPLE},Slide{color: RED},Slide{color: BLUE}];
    Model {
        current_slide: 0,
        slides: slides
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn event(_app: &App, _model: &mut Model, _event: Event) {
    if let Event::WindowEvent { id: _, simple } = _event {
        if let Some(KeyPressed(k)) = simple {
            let slides_length = _model.slides.len();
            match k {
                Key::Right => {
                    _model.current_slide = (slides_length - 1).min(_model.current_slide + 1);
                }
                Key::Left => {
                    _model.current_slide = 0.max(_model.current_slide as isize - 1) as usize;
                }
                _ => {}
            }
        }
    }
}

fn view(_app: &App, _model: &Model, frame: Frame){
    frame.clear(_model.slides[_model.current_slide].color);
}
