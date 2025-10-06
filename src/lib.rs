use error_iter::ErrorIter as _;
use log::error;
use pixels::{wgpu, Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::KeyCode,
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use crate::{layout::Layout, palette::Color, tree::Node};

pub mod buffer;
pub mod button;
pub mod element;
pub mod font;
pub mod image;
pub mod layout;
pub mod palette;
pub mod quad;
pub mod text;
pub mod tree;

pub fn run<State, Message, Update, View>(
    initial_state: State,
    update: Update,
    view: View,
    width: u32,
    height: u32,
    clear_color: Color,
) -> Result<(), Error>
//TODO: make a custom error type
where
    Update: Fn(Message, &mut State),
    View: Fn(&State) -> Node<Message, Layout>,
{
    env_logger::init();

    let mut state = initial_state;

    let event_loop = EventLoop::new().unwrap();
    let mut winit_input = WinitInputHelper::new();
    let window = {
        // TODO: Consider default scaling
        let size = LogicalSize::new(2.0 * width as f64, 2.0 * height as f64);
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
        Pixels::new(width, height, surface_texture)?
    };

    // TODO: Let Users specify the palette/ clear color
    let srgb = to_linear_rgb(clear_color);
    pixels.clear_color(srgb);

    let mut root = view(&state).calculate_layout();

    let mut mouse_position: Result<(usize, usize), (isize, isize)> = Err((0, 0));

    let res = event_loop.run(|event, elwt| {
        // TODO: consider only calculating when necessary
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
        if winit_input.update(&event) {
            // Close events
            if winit_input.key_pressed(KeyCode::Escape) || winit_input.close_requested() {
                elwt.exit();
                return;
            }

            // Resize the window
            if let Some(size) = winit_input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    elwt.exit();
                    return;
                }
            }

            // build input struct
            let input_mouse_pos = if let Ok((x, y)) = mouse_position {
                Some((x as u32, y as u32))
            } else {
                None
            };
            let input = crate::button::Input {
                mouse_released: winit_input.mouse_released(0),
                mouse_pos: input_mouse_pos,
            };

            // get a message, if any
            let message = root.get_message(&input);

            // TODO handle multiple messages in a frame?
            if let Some(message) = message {
                update(message, &mut state);
                root = view(&state).calculate_layout();
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
