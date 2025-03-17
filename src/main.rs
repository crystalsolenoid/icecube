#![deny(clippy::all)]
#![forbid(unsafe_code)]
#![allow(unused_imports)]

use error_iter::ErrorIter as _;

use icecube::button::Button;
use icecube::palette::{BLUE_DARK, BLUE_LIGHT, MAIN_DARK, MAIN_LIGHT, RED_DARK, RED_LIGHT};
use icecube::text::Text;
use log::error;
use pixels::{wgpu, Error, Pixels, SurfaceTexture};
use winit::dpi::{LogicalSize, PhysicalPosition};
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use icecube::layout::LayoutEngineResult;
use icecube::quad::{BorderStyle, Quad, QuadStyle};
use icecube::tree::{Layout, Node};

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
    let mut root = Node::root_node(WIDTH as usize, HEIGHT as usize); // TODO figure out how we want to

    // handle coordinate types everywhere
    let panel_style = QuadStyle {
        fill_style: Some(MAIN_DARK),
        border_style: Some(BorderStyle {
            color: BLUE_DARK,
            thickness: 5,
        }),
    };
    let viewport_style = QuadStyle {
        fill_style: Some(MAIN_LIGHT),
        border_style: Some(BorderStyle {
            color: MAIN_DARK,
            thickness: 0,
        }),
    };

    let mut panel = Node::new(
        Quad::new(100, HEIGHT).style(panel_style),
        Layout::Column,
        LayoutEngineResult::test(0, 0, 100, HEIGHT),
    );
    let mut viewport = Node::new(
        Quad::new(WIDTH - 100, HEIGHT)
            .style(viewport_style)
            .with_padding([5, 10].into()),
        Layout::Column,
        LayoutEngineResult::test(100, 0, WIDTH - 100, HEIGHT),
    );

    let quad_1 = Quad::new(90, 40)
        .fill(RED_DARK)
        .border_thickness(3)
        .border_color(BLUE_LIGHT);

    let widget_1 = Node::new(
        quad_1.clone(),
        Layout::Row,
        LayoutEngineResult::test(0, 0, 90, 40),
    );
    let widget_3 = Node::new(
        quad_1.clone(),
        Layout::Row,
        LayoutEngineResult::test(0, 50, 90, 40),
    );
    let widget_4 = Node::new(
        quad_1.clone(),
        Layout::Row,
        LayoutEngineResult::test(0, 100, 90, 40),
    );

    let text_1 = Text {
        content: " hello world".into(),
    };
    let widget_2 = Node::new(
        text_1.clone(),
        Layout::Row,
        LayoutEngineResult::test(0, 150, 90, 40),
    );

    panel.push(widget_1);
    panel.push(widget_2);
    panel.push(widget_4);
    panel.push(widget_3);

    let text_test = Node::new(
        Text {
            content: "Icecube can render text!!".into(),
        },
        Layout::Row,
        LayoutEngineResult::test(100, 200, 90, 40),
    );
    viewport.push(text_test);
    viewport.push(Node::new(
        Button {
            text: Text {
                content: "Click Me!".into(),
            },
            quad: quad_1.fill(MAIN_DARK).with_padding([0, 0].into()),
        },
        Layout::Row,
        LayoutEngineResult::test(100, 100, 90, 40),
    ));

    root.push(panel);
    root.push(viewport);

    let mut mouse_position: Result<(usize, usize), (isize, isize)> = Err((0, 0));

    let res = event_loop.run(|event, elwt| {
        // Draw the current frame
        if let Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } = event
        {
            //world.draw(pixels.frame_mut());
            root.draw_recursive(pixels.frame_mut(), (0, 0));
            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                elwt.exit();
                return;
            }
        }

        if let Event::WindowEvent {
            event:
                WindowEvent::CursorMoved {
                    device_id: _,
                    position,
                },
            ..
        } = event
        {
            // Convert it to a pixel location
            mouse_position = pixels.window_pos_to_pixel(position.into());
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

            if input.mouse_released(0) {
                if let Ok((x, y)) = mouse_position {
                    root.on_click((x as u32, y as u32));
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
