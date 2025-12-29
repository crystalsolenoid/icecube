use icecube::font::{self};
use icecube::image::Image;
use icecube::layout::{Layout, Length};
use icecube::palette::{BLUE_DARK, BLUE_LIGHT, MAIN_DARK, MAIN_LIGHT, RED_DARK};
use icecube::quad::Quad;
use icecube::slider::Slider;
use icecube::text::Text;
use icecube::tree::Node;
use icecube::{col, row, stack};

const IMWIDTH: usize = 25;
const IMHEIGHT: usize = 25;

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
}

#[derive(Default)]
struct State {
    value: f32,
}

fn update(m: Message, state: &mut State) {
    match m {
        Message::Slide(val) => state.value = val,
    }
}

fn view(state: &State) -> Node<Message, Layout> {
    let font = &font::BLACKLETTER;

    let image = Node::new(Image::new(imdata(), IMWIDTH, IMHEIGHT).scale_factor(8))
        .height(Length::Shrink)
        .width(Length::Shrink);

    let mut text2 = Node::new(
        Text::new(format!("{:.1}", state.value))
            .with_font(font)
            .with_color(MAIN_DARK),
    )
    .width(Length::Shrink);
    text2.name = Some("counter value".to_string());

    let quad2 = Node::new(
        Quad::new()
            .fill(BLUE_LIGHT)
            .border_thickness(1)
            .border_color(MAIN_DARK),
    )
    .height(Length::Fixed(100))
    .width(Length::Fixed(150))
    .row();

    let quad3 = Node::new(
        Quad::new()
            .fill(BLUE_DARK)
            .border_thickness(1)
            .border_color(RED_DARK),
    )
    .height(Length::Fixed(80))
    .width(Length::Fixed(30))
    .row();
    let centered_quad3 = row![
        Node::spacer(),
        col![Node::spacer(), quad3, Node::spacer()],
        Node::spacer()
    ]
    .height(Length::Grow);

    row![
        Node::spacer(),
        col![
            Node::spacer(),
            stack![image, quad2, centered_quad3, text2],
            Node::spacer()
        ],
        Node::spacer()
    ]
    .height(Length::Grow)
}

fn main() -> Result<(), pixels::Error> {
    let initial_state = State::default();

    icecube::run(initial_state, update, view, 320, 240, MAIN_LIGHT, |_| None)
}
