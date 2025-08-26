use nannou::{prelude::*, wgpu::Texture};
use tiny_skia::Pixmap;
use typst::layout::{Axis, PagedDocument};
use nannou::wgpu;

use crate::utils::{self, sandbox::{self, Sandbox}};

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

fn compile_pixmap(sandbox: &utils::sandbox::Sandbox, source: &str) -> Pixmap {
    let world = sandbox.with_source(String::from(source));
    let document = typst::compile::<PagedDocument>(&world);
    let output = document.output.expect("Typst compilation failed");
    let page = output.pages.get(0).expect("No pages rendered");
    let pixels_per_point = determine_pixels_per_point(page.frame.size()).unwrap();
    let render = typst_render::render(page, pixels_per_point);
    render
}

fn compile_to_texture(source: &str, sandbox: &Sandbox, app: &App) -> Texture {
    let pixmap = compile_pixmap(sandbox, source);
    pixmap_to_texture(app, &pixmap)
}

pub struct TypstElement {
    source: String,
}

impl TypstElement {
    pub fn from(source: &str) -> TypstElement {
        TypstElement {
            source: String::from(source)
        }
    }
}

impl crate::ToTexture for TypstElement {
    fn to_texture(&self, app: &App, sandbox: &Sandbox) -> Texture {
        compile_to_texture(&self.source, sandbox, app)
    }
}

pub struct Text {
    text: String
}

impl Text {
    pub fn from(source: &str) -> Text {
        Text {
            text: String::from(source)
        }
    }
}

impl crate::ToTexture for Text {
    fn to_texture(&self, app: &App, sandbox: &Sandbox) -> Texture {
        let source = format!("#set page(fill: none, width: auto, height: auto)\n#text([{}])", &self.text);
        let typst_element = TypstElement::from(&source);
        typst_element.to_texture(app, sandbox)
    }
}
