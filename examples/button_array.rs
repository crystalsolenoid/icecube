use icecube::font::{self};
use icecube::image::Image;
use icecube::layout::{Layout, Length};
use icecube::mouse_area::MouseArea;
use icecube::palette::{Color, BLUE_DARK, MAIN_LIGHT, RED_DARK};
use icecube::quad::Quad;
use icecube::text::Text;
use icecube::tree::Node;
use icecube::{col, row};

const IMWIDTH: usize = 25;
const IMHEIGHT: usize = 25;
const IMSCALE: usize = 7;

fn imdata() -> Vec<[u8; 4]> {
    let area = IMWIDTH * IMHEIGHT;
    let a = [255, 35, 255, 255];
    let b = [255, 255, 255, 255];
    let c = [35, 255, 255, 255];
    let d = [35, 255, 35, 255];
    (0..=area)
        .map(|n| (n % IMWIDTH, n / IMWIDTH))
        .map(|(x, y)| {
            let w11 = (IMWIDTH - x) * (IMHEIGHT - y);
            let w12 = (IMWIDTH - x) * y;
            let w21 = x * (IMHEIGHT - y);
            let w22 = x * y;
            let r = ((a[0] * w11 + b[0] * w12 + c[0] * w21 + d[0] * w22) / area) as u8;
            let g = ((a[1] * w11 + b[1] * w12 + c[1] * w21 + d[1] * w22) / area) as u8;
            let b = ((a[2] * w11 + b[2] * w12 + c[2] * w21 + d[2] * w22) / area) as u8;
            [r, g, b, 255]
        })
        .collect()
}

#[derive(Debug, Copy, Clone)]
pub enum Message {
    Slide(f32),
    SelectPrimaryColor((usize, usize)),
    SelectSecondaryColor((usize, usize)),
}

struct State {
    value: f32,
    image: Vec<Color>,
    selected_primary_color: Color,
    selected_secondary_color: Color,
}

impl Default for State {
    fn default() -> Self {
        Self {
            value: Default::default(),
            image: imdata(),
            selected_primary_color: BLUE_DARK,
            selected_secondary_color: RED_DARK,
        }
    }
}

fn update(m: Message, state: &mut State) {
    match m {
        Message::Slide(val) => state.value = val,
        Message::SelectPrimaryColor(pos) => {
            let (px, py) = (pos.0 / IMSCALE, pos.1 / IMSCALE);
            state.selected_primary_color = state.image[px + py * IMWIDTH];
        }
        Message::SelectSecondaryColor(pos) => {
            let (px, py) = (pos.0 / IMSCALE, pos.1 / IMSCALE);
            state.selected_secondary_color = state.image[px + py * IMWIDTH];
        }
    }
}

fn view(state: &State) -> Node<Message, Layout> {
    let font = &font::BLACKLETTER;

    let image = Node::new(Image::new(state.image.clone(), IMWIDTH, IMHEIGHT).scale_factor(IMSCALE))
        .height(Length::Shrink)
        .width(Length::Shrink);

    let mut mouse_image_wrapper: Node<Message, _> = MouseArea::new()
        .on_press(Message::SelectPrimaryColor)
        .on_right_press(Message::SelectSecondaryColor)
        .into();
    mouse_image_wrapper.push(image);

    row![
        Node::spacer(),
        col![
            Node::spacer(),
            mouse_image_wrapper,
            Node::spacer(),
            col![
                row![
                    Node::new(Text::new("Primary".to_string()).with_font(font)),
                    Node::spacer(),
                    Node::new(Quad::new().fill(state.selected_primary_color))
                        .height(20)
                        .width(20),
                ]
                .width(Length::Grow),
                row![
                    Node::new(Text::new("Secondary".to_string()).with_font(font)),
                    Node::spacer(),
                    Node::new(Quad::new().fill(state.selected_secondary_color))
                        .height(20)
                        .width(20),
                ]
                .width(Length::Grow),
            ]
            .width(125)
            .spacing(2),
            Node::spacer(),
            Node::spacer(),
        ],
        Node::spacer(),
    ]
    .height(Length::Grow)
}

fn main() -> Result<(), pixels::Error> {
    let initial_state = State::default();

    icecube::run(initial_state, update, view, 320, 240, MAIN_LIGHT, |_| None)
}
