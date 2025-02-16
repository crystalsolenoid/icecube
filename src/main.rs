#![deny(clippy::all)]
#![forbid(unsafe_code)]

use error_iter::ErrorIter as _;
use icecube::palette::{Color, BLUE_DARK, BLUE_LIGHT, MAIN_DARK, MAIN_LIGHT, RED_DARK};
use log::error;
use pixels::{wgpu, Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let srgb = to_linear_rgb(MAIN_DARK);
    pixels.clear_color(srgb);

    //let mut world = World::new();
    let template_quad = Quad {
        left: 30,
        top: 20,
        width: 200,
        height: 120,
        style: QuadStyle {
            fill_style: Some(BLUE_LIGHT),
            border_style: Some(BorderStyle {
                color: BLUE_DARK,
                thickness: 2,
            }),
        },
    };
    let quad_tests = [
        Quad {
            style: QuadStyle {
                fill_style: Some(MAIN_LIGHT),
                border_style: Some(BorderStyle {
                    color: RED_DARK,
                    thickness: 5,
                }),
            },
            ..template_quad
        },
        Quad {
            left: 10,
            top: 30,
            width: 300,
            height: 60,
            ..template_quad
        },
    ];

    let res = event_loop.run(|event, elwt| {
        // Draw the current frame
        if let Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } = event
        {
            //world.draw(pixels.frame_mut());
            quad_tests
                .iter()
                .for_each(|quad| quad.draw(pixels.frame_mut()));
            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                elwt.exit();
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                elwt.exit();
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    elwt.exit();
                    return;
                }
            }

            // Update internal state and request a redraw
            //            world.update();
            window.request_redraw();
        }
    });
    res.map_err(|e| Error::UserDefined(Box::new(e)))
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

#[derive(Clone)]
struct QuadStyle {
    fill_style: Option<Color>,
    border_style: Option<BorderStyle>,
}

#[derive(Clone)]
struct BorderStyle {
    color: Color,
    thickness: usize,
}

#[derive(Clone)]
struct Quad {
    left: usize,
    top: usize,
    width: usize,
    height: usize,
    style: QuadStyle,
}

impl Quad {
    fn draw(&self, frame: &mut [u8]) {
        //TODO: Consider optimizing this if it is a bottleneck
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = i % WIDTH as usize;
            let y = i / WIDTH as usize;

            let inside_the_box = x >= self.left
                && x < self.left + self.width
                && y >= self.top
                && y < self.top + self.height;

            let rgba = if inside_the_box {
                match &self.style.border_style {
                    Some(border_style) => {
                        let border_thickness = border_style.thickness;
                        if x < self.left + border_thickness
                            || x >= self.left + self.width - border_thickness
                            || y < self.top + border_thickness
                            || y >= self.top + self.height - border_thickness
                        {
                            Some(border_style.color)
                        } else {
                            self.style.fill_style
                        }
                    }
                    None => self.style.fill_style,
                }
            } else {
                None
            };

            if let Some(color) = rgba {
                pixel.copy_from_slice(&color);
            }
        }
    }
}

/// sRGB to linear conversion.
///
/// For changing the color of the black bars from mismatched window/buffer dimensions a
/// specified color that matches the color format Pixels expects when writing to the
/// buffer.
/// Implementation taken from https://www.khronos.org/registry/OpenGL/extensions/EXT/EXT_texture_sRGB_decode.txt
pub fn to_linear_rgb(c: [u8; 4]) -> wgpu::Color {
    let f = |xu: u8| -> f64 {
        let x = xu as f64 / 255.0;
        if x > 0.04045 {
            ((x + 0.055) / 1.055).powf(2.4)
        } else {
            x / 12.92
        }
    };

    wgpu::Color {
        r: f(c[0]),
        g: f(c[1]),
        b: f(c[2]),
        a: 1.,
    }
}
