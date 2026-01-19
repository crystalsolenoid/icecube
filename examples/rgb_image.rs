use icecube::button::Button;
use icecube::image::Image;
use icecube::layout::{Layout, Length};
use icecube::palette::MAIN_DARK;
use icecube::tree::Node;
use icecube::{col, row};

#[derive(Debug, Copy, Clone)]
pub enum Message {
    Invert,
}

struct State {
    data: Vec<[u8; 4]>,
}

const WIDTH: usize = 25;
const HEIGHT: usize = 25;

impl Default for State {
    fn default() -> Self {
        let a = [255, 35, 255, 255];
        let b = [255, 255, 255, 255];
        let c = [35, 255, 255, 255];
        let d = [35, 255, 35, 255];
        let area = WIDTH * HEIGHT;
        Self {
            data: (0..=area)
                .map(|n| (n % WIDTH, n / WIDTH))
                .map(|(x, y)| {
                    let w11 = (WIDTH - x) * (HEIGHT - y);
                    let w12 = (WIDTH - x) * y;
                    let w21 = x * (HEIGHT - y);
                    let w22 = x * y;
                    let r = ((a[0] * w11 + b[0] * w12 + c[0] * w21 + d[0] * w22) / area) as u8;
                    let g = ((a[1] * w11 + b[1] * w12 + c[1] * w21 + d[1] * w22) / area) as u8;
                    let b = ((a[2] * w11 + b[2] * w12 + c[2] * w21 + d[2] * w22) / area) as u8;
                    [r, g, b, 255]
                })
                .collect(),
        }
    }
}

impl State {}

fn update(_m: Message, _state: &mut State) {}

fn view<'a>(state: &State) -> Node<'a, Message, Layout> {
    let image = Node::new(Image::new(state.data.clone(), WIDTH, HEIGHT).scale_factor(8))
        .height(Length::Shrink)
        .width(Length::Shrink);

    let mut button = Node::new(Button::new().on_press(Message::Invert))
        .height(Length::Shrink)
        .width(Length::Shrink);
    button.push(image);

    row![
        Node::spacer(),
        col![Node::spacer(), button, Node::spacer()],
        Node::spacer()
    ]
    .height(Length::Grow)
}

fn main() -> Result<(), pixels::Error> {
    let initial_state = State::default();

    icecube::run(initial_state, update, view, 320, 240, MAIN_DARK, |_| None)
}
