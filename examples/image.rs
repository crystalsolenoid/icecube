use icecube::element::Element;
use icecube::image::Image;
use icecube::layout::{CalculatedLayout, Layout, Length};
use icecube::palette::{BLUE_DARK, MAIN_LIGHT};
use icecube::quad::Quad;
use icecube::to_linear_rgb;
use icecube::tree::Node;

use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::KeyCode,
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;
#[derive(Debug, Copy, Clone)]
pub enum Message {}

#[derive(Clone)]
struct State {
    data: [usize; 12],
}

// const DATA: [usize; 12] = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];

impl Default for State {
    fn default() -> Self {
        Self {
            data: [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
        }
    }
}

fn update(_m: Message, state: State) -> State {
    state
}

fn view<'a>(state: &'a State) -> Node<'a, Message, Layout> {
    //TODO: width height here, but height width in padding
    let mut root = Node::root_node(320, 240).row();

    // This fills the screen, causing the screen to clear each frame
    let mut container = Node::new(Quad::new().fill(MAIN_LIGHT))
        .column()
        .padding([100, 140]);

    let image = Node::new(Image::new(&state.data, 3, 4).scale_factor(1))
        .height(Length::Shrink)
        .width(Length::Shrink);

    container.push(Node::new(Quad::new().fill(BLUE_DARK)).height(Length::Fixed(50)));
    container.push(image);
    container.push(Node::new(Quad::new().fill(BLUE_DARK)).height(Length::Fixed(50)));
    root.push(container);
    root
}

fn main() -> Result<(), pixels::Error> {
    let width = 320;
    let height = 320;
    let clear_color = MAIN_LIGHT;

    env_logger::init();
    let initial_state = State::default();

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

    // let init_state = state.clone();
    let mut root: Node<Message, _> = Node::root_node(320, 240).calculate_layout(); //view(&state).calculate_layout();

    let mut mouse_position: Result<(usize, usize), (isize, isize)> = Err((0, 0));

    {
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
                    // log_error("pixels.render", err);
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
                        // log_error("pixels.resize_surface", err);
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
                let input = icecube::button::Input {
                    mouse_released: winit_input.mouse_released(0),
                    mouse_pos: input_mouse_pos,
                };

                // get a message, if any
                let message = root.get_message(&input);

                // TODO handle multiple messages in a frame?
                if let Some(message) = message {
                    state = update(message, state);
                    root = view(&state).calculate_layout();
                }

                // Update internal state and request a redraw
                //            world.update();
                window.request_redraw();
            }
        });
        res.map_err(|e| Error::UserDefined(Box::new(e)))
    }
}
