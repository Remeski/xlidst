extern crate nannou;

use nannou::{prelude::*};
use typst::foundations::Str;
use typst::layout::{Axis, PagedDocument};
use nannou::wgpu;
use nannou::wgpu::TextureBuilder;
use typst::World;
mod sandbox;

const DESIRED_RESOLUTION: f32 = 1000.0;
const MAX_SIZE: f32 = 10000.0;
const MAX_PIXELS_PER_POINT: f32 = 5.0;

#[derive(Debug)]
pub struct TooBig {
	size: f32,
	axis: Axis,
}

fn pixmap_to_texture(app: &App, pixmap: &tiny_skia::Pixmap) -> wgpu::Texture {
    // create a texture from pixmap
    // pixmap.data() is [u8] where byteorder RGBA
    let window = app.main_window();
    let device = window.device();
    let queue = window.queue();

    let image = nannou::image::RgbaImage::from_raw(pixmap.width(), pixmap.height(), pixmap.data().to_vec()).expect("rgbaimage creation failed");
    let dynamic_image = nannou::image::DynamicImage::ImageRgba8(image);
    wgpu::Texture::load_from_image(device, queue, wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT, &dynamic_image)
}

fn determine_pixels_per_point(size: typst::layout::Size) -> Result<f32, TooBig> {
	// We want to truncate.
	#![allow(clippy::cast_possible_truncation)]

	let x = size.x.to_pt() as f32;
	let y = size.y.to_pt() as f32;

	if x > MAX_SIZE {
		Err(TooBig {
			size: x,
			axis: Axis::X,
		})
	} else if y > MAX_SIZE {
		Err(TooBig {
			size: y,
			axis: Axis::Y,
		})
	} else {
		let area = x * y;
		let nominal = DESIRED_RESOLUTION / area.sqrt();
		Ok(nominal.min(MAX_PIXELS_PER_POINT))
	}
}

fn compile_pixmap(sandbox: &crate::sandbox::Sandbox, source: &str) -> tiny_skia::Pixmap {
    let world = sandbox.with_source(String::from(source));
    let document = typst::compile::<PagedDocument>(&world);
    let output = document.output.expect("Typst compilation failed");
    let page = output.pages.get(0).expect("No pages rendered");
    let pixels_per_point = determine_pixels_per_point(page.frame.size()).unwrap();
    let render = typst_render::render(page, pixels_per_point);
    render
}

struct SourceSlide {
    background_color: rgb::Srgb<u8>,
    source: String,
}

impl SourceSlide {
    fn parse(&self, app: &App, sandbox: &sandbox::Sandbox) -> Slide {
        let pixmap = compile_pixmap(sandbox, &self.source);
        let texture = pixmap_to_texture(app, &pixmap);
        Slide {
            background_color: self.background_color,
            textures: vec![texture]
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
    // sandbox: crate::sandbox::Sandbox,
}

fn model(app: &App) -> Model {
    let sandbox = crate::sandbox::Sandbox::new();

    let source_slides = vec![
        SourceSlide{
            background_color: PURPLE,
            source: String::from("#set page(width: 100pt, height: 100pt)\n= NAPS")
        },
        SourceSlide{
            background_color: RED,
            source: String::from("#set page(width: 100pt, height: 100pt)\n== Euler\n$e^(i pi) + 1 = 0$")
        },
        SourceSlide{
            background_color: BLUE,
            source: String::from("#set page(width: 100pt, height: 100pt)\n== Hemo munk")
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

