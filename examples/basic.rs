#![deny(clippy::all)]
#![forbid(unsafe_code)]

use icecube::button::Button;
use icecube::font;
use icecube::layout::{Layout, Length};
use icecube::palette::{BLUE_DARK, MAIN_DARK, MAIN_LIGHT, RED_DARK, RED_LIGHT};
use icecube::quad::Quad;
use icecube::text::Text;
use icecube::tree::Node;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

// TODO can we specify a generic default for Node for a nicer API?
fn build_ui_tree(state: &State) -> Node<Message, Layout> {
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
        .padding(2)
        .row();
        container.push(
            Node::new(Text::new(label.into()).with_font(font))
                .width(Length::Grow)
                .height(Length::Grow),
        );
        container
    };

    b.push(b_child());
    b.push(
        menu_item(
            "A long piece of text that no longer overflows its box...",
            &font::BLACKLETTER,
        )
        .width(Length::Grow),
    );
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

    let button_text = Node::new(Text::new("color".into()).with_font(&font::OLDSCHOOL));
    let mut button_quad = Node::new(
        Quad::new()
            .fill(state.text_color)
            .border_thickness(1)
            .border_color(BLUE_DARK),
    )
    .width(Length::Grow) // TODO when this is shrink, the text has zero room. Why?
    .height(Length::Shrink)
    .padding(2)
    .row();
    button_quad.push(button_text);
    let mut change_text_color = Node::new(Button::new().on_press(Message::ButtonClick))
        .width(Length::Grow)
        .height(Length::Shrink);
    change_text_color.push(button_quad);

    panel.push(change_text_color);
    //    panel.push(menu_item("a", builtin_fonts::TEST_FONT)); // TODO
    //    panel.push(menu_item("a", FontType::Image(wiftnywfutn)));
    panel.push(menu_item("b", &font::OLDSCHOOL));
    panel.push(menu_item("c - a long label", &font::OLDSCHOOL));
    panel.push(menu_item("d", &font::OLDSCHOOL));
    panel.push(spacer());
    panel.push(menu_item("這", &font::OLDSCHOOL));

    viewport.push(a);
    viewport.push(b);
    viewport.push(spacer());
    viewport.push(c);

    root.push(panel);
    root.push(viewport);
    root
}

// TODO user-defined
#[derive(Debug, Copy, Clone)]
pub enum Message {
    ButtonClick,
    ButtonHover,
}

// TODO this will be defined by the person writing the UI, not by the icecube library
#[derive(Default)]
struct State {
    text_color: icecube::palette::Color,
}

// TODO this will be defined by the person writing the UI, not by the icecube library
fn update(m: Message, state: &mut State) {
    match m {
        Message::ButtonClick => state.text_color = MAIN_LIGHT,
        Message::ButtonHover => (),
    }
}

// TODO this will be defined by the person writing the UI, not by the icecube library
fn view(state: &State) -> Node<Message, Layout> {
    build_ui_tree(state)
}

fn main() -> Result<(), pixels::Error> {
    let initial_state = State::default();

    icecube::run(initial_state, update, view, 320, 240, MAIN_DARK, |_| None)
}
