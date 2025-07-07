#![deny(clippy::all)]
#![forbid(unsafe_code)]
#![allow(unused_imports)]

use error_iter::ErrorIter as _;

use log::error;
use pixels::{wgpu, Error, Pixels, SurfaceTexture};
use winit::dpi::{LogicalSize, PhysicalPosition};
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use icecube::button::Button;
use icecube::font::{TEST_FONT, TEST_FONT2};
use icecube::layout::{CalculatedLayout, Layout, Length};
use icecube::palette::{BLUE_DARK, BLUE_LIGHT, MAIN_DARK, MAIN_LIGHT, RED_DARK, RED_LIGHT};
use icecube::quad::{BorderStyle, Quad, QuadStyle};
use icecube::text::Text;
use icecube::tree::Node;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

/*
fn _build_ui_tree_simpler() -> Node<Layout> {
    let mut root = Node::root_node(WIDTH as usize, HEIGHT as usize).row(); // TODO figure out how we want to
                                                                           // handle coordinate types everywhere
                                                                           // usize vs u32

    let mut viewport = Node::new(
        Quad::new()
            .fill(MAIN_LIGHT)
            .border_thickness(2)
            .border_color(RED_DARK),
    )
    .width(Length::Grow)
    .height(Length::Grow)
    .spacing(10)
    .padding([4, 10])
    .column();

    let mut b = Node::new(
        Quad::new()
            .fill(MAIN_LIGHT)
            .border_thickness(2)
            .border_color(BLUE_DARK),
    )
    .width(Length::Grow)
    .height(Length::Fixed(60))
    .padding(4)
    .spacing(10)
    .row();

    let b_child = || {
        Node::new(
            Quad::new()
                .fill(MAIN_LIGHT)
                .border_thickness(1)
                .border_color(BLUE_DARK),
        )
        .height(Length::Grow)
        .width(Length::Fixed(10))
        .row()
    };

    let menu_item = |label: &str, font| {
        let mut container = Node::new(
            Quad::new()
                .fill(MAIN_LIGHT)
                .border_thickness(1)
                .border_color(BLUE_DARK),
        )
        .width(Length::Grow)
        .height(Length::Shrink)
        .padding(1)
        .row();
        container.push(Node::new(Text::new(label.into()).with_font(font)));
        container
    };

    b.push(b_child());
    b.push(
        menu_item(
            "A long piece of text that currently overflows its box...",
            &TEST_FONT,
        )
        .width(Length::Grow),
    );
    b.push(b_child());
    b.push(b_child());
    b.push(b_child());
    b.push(b_child());
    b.push(b_child());
    b.push(b_child());

    viewport.push(b);
    //    viewport.push(spacer());

    //    root.push(panel);
    root.push(viewport);
    root
}
*/

// TODO can we specify a generic default for Node for a nicer API?
fn build_ui_tree() -> Node<Layout> {
    /*
     * Intended layout tree:
     *
     * root (row)
     *   | panel (main_dark; blue_dark)
     *   | viewport (row) (main_light; main_dark)
     *   |   | a (red_light; red_dark)
     *   |   | b (blue_light; blue_dark)
     */

    let mut root = Node::root_node(WIDTH as usize, HEIGHT as usize).row(); // TODO figure out how we want to
                                                                           // handle coordinate types everywhere
                                                                           // usize vs u32

    let mut panel = Node::new(
        Quad::new()
            .fill(MAIN_DARK)
            .border_thickness(2)
            .border_color(BLUE_DARK),
    )
    .height(Length::Grow)
    .width(Length::Fixed(50))
    .column()
    .padding(4)
    .spacing(2);

    let mut viewport = Node::new(
        Quad::new()
            .fill(MAIN_LIGHT)
            .border_thickness(2)
            .border_color(RED_DARK),
    )
    .width(Length::Grow)
    .height(Length::Grow)
    .spacing(10)
    .padding([4, 10])
    .column();

    let a = Node::new(
        Quad::new()
            .fill(RED_LIGHT)
            .border_thickness(2)
            .border_color(RED_DARK),
    )
    .width(Length::Grow)
    .height(Length::Fixed(30))
    .row();

    let mut b = Node::new(
        Quad::new()
            .fill(MAIN_LIGHT)
            .border_thickness(2)
            .border_color(BLUE_DARK),
    )
    .width(Length::Grow)
    .height(Length::Fixed(60))
    .padding(4)
    .spacing(10)
    .row();

    let b_child = || {
        Node::new(
            Quad::new()
                .fill(MAIN_LIGHT)
                .border_thickness(1)
                .border_color(BLUE_DARK),
        )
        .height(Length::Grow)
        .width(Length::Fixed(10))
        .row()
    };

    let menu_item = |label: &str, font| {
        let mut container = Node::new(
            Quad::new()
                .fill(MAIN_LIGHT)
                .border_thickness(1)
                .border_color(BLUE_DARK),
        )
        .width(Length::Grow)
        .height(Length::Shrink)
        .padding(1)
        .row();
        container.push(Node::new(Text::new(label.into()).with_font(font)));
        container
    };

    b.push(b_child());
    b.push(b_child());
    b.push(b_child());
    b.push(
        menu_item(
            "A long piece of text that currently overflows its box...",
            &TEST_FONT,
        )
        .width(Length::Grow),
    );
    b.push(b_child());
    b.push(b_child());
    b.push(b_child());

    let c = Node::new(
        Quad::new()
            .fill(RED_DARK)
            .border_thickness(2)
            .border_color(RED_LIGHT),
    )
    .width(Length::Grow)
    .height(Length::Fixed(30))
    .row();

    let spacer = || {
        Node::new(Quad::new().border_thickness(0))
            .width(Length::Grow)
            .height(Length::Grow)
            .row()
    };

    panel.push(menu_item("a", &TEST_FONT2));
    //    panel.push(menu_item("a", builtin_fonts::TEST_FONT)); // TODO
    //    panel.push(menu_item("a", FontType::Image(wiftnywfutn)));
    panel.push(menu_item("b", &TEST_FONT2));
    panel.push(menu_item("c - a long label", &TEST_FONT2));
    panel.push(menu_item("d", &TEST_FONT2));
    panel.push(spacer());
    panel.push(menu_item("這", &TEST_FONT2));

    viewport.push(a);
    viewport.push(b);
    viewport.push(spacer());
    viewport.push(c);

    root.push(panel);
    root.push(viewport);
    root
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(2.0 * WIDTH as f64, 2.0 * HEIGHT as f64);
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

    let root = build_ui_tree().calculate_layout();

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
