use nannou::{prelude::*, wgpu::Texture};
use tiny_skia::Pixmap;
use typst::layout::{Axis, PagedDocument};
use nannou::wgpu;

use crate::{slides::{AsTexture, Element, ToElement}, utils::{self, sandbox::Sandbox}};

const DESIRED_RESOLUTION: f32 = 1000.0;
const MAX_SIZE: f32 = 10000.0;
const MAX_PIXELS_PER_POINT: f32 = 5.0;

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

fn determine_pixels_per_point(size: typst::layout::Size) -> Result<f32, String> {
	let x = size.x.to_pt() as f32;
	let y = size.y.to_pt() as f32;

	if x > MAX_SIZE {
		Err(String::from("x too big"))
	} else if y > MAX_SIZE {
		Err(String::from("y too bgg"))
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
    x: f32,
    y: f32
}

impl TypstElement {
    pub fn from(source: &str) -> TypstElement {
        TypstElement {
            source: String::from(format!("#set page(width: auto, height: auto, fill: none)\n{}", source)),
            x: 0.0,
            y: 0.0
        }
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
}

impl ToElement for TypstElement {
    fn to(self) -> Element {
        Element::Texture(Box::from(self))
    }
}

impl AsTexture for TypstElement {
    fn to_texture(&self, app: &App) -> Option<Texture> {
        let sandbox = Sandbox::new();
        Some(compile_to_texture(&self.source, &sandbox, app))
    }
    fn get_x(&self) -> f32 {
        self.x
    }
    fn get_y(&self) -> f32 {
        self.y
    }
}

// pub struct Text {
//     text: String,
//     x: f32,
//     y: f32
// }
//
// impl Text {
//     pub fn from(source: &str) -> Text {
//         Text {
//             text: String::from(source),
//             x: 0.0,
//             y: 0.0
//         }
//     }
//     pub fn with_pos(source: &str, (x,y): (f32, f32)) -> Text {
//         let mut text = Self::from(source);
//         text.set_x(x);
//         text.set_y(y);
//         text
//     }
//     pub fn set_x(&mut self, x: f32) {
//         self.x = x;
//     }
//     pub fn set_y(&mut self, y: f32) {
//         self.y = y;
//     }
// }
//
// impl AsTexture for Text {
//     fn to_texture(&self, app: &App) -> Option<Texture> {
//         let source = format!("#set page(fill: none, width: auto, height: auto)\n#text([{}])", &self.text);
//
//         if let Element::Texture(typst_element) = TypstElement::from(&source) {
//             typst_element.to_texture(app)
//         } else {
//             None
//         }
//
//     }
//     fn get_x(&self) -> f32 {
//         self.x
//     }
//     fn get_y(&self) -> f32 {
//         self.y
//     }
// }
