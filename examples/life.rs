use icecube::button::Button;
use icecube::font;
use icecube::image::Image;
use icecube::layout::{Layout, Length};
use icecube::mouse_area::MouseArea;
use icecube::palette::{BLUE_DARK, BLUE_LIGHT, MAIN_LIGHT};
use icecube::quad::Quad;
use icecube::text::Text;
use icecube::tree::Node;

use rand::prelude::*;

/*
WISHLIST
- alpha value for images
- default to shrink, not grow (for easier wrapper making)
- bug?: border doesn't show up when you have a Quad wrapping an Image with shrink
    (adding padding fixes)
- default frame or tick event with a delta saying how much time has passed
- key events (do we have this yet?)
- an event for button press that also includes the position
    (to click on individual cells)
*/

const WIDTH: usize = 32;
const HEIGHT: usize = 32;
const SCALE_FACTOR: usize = 4;

#[derive(Debug, Copy, Clone)]
pub enum Message {
    Step,
    Glider,
    Clear,
    Randomize,
    BoardClick((usize, usize)),
    BoardHover((usize, usize)),
    BoardExit,
}

#[derive(Default)]
struct State {
    board: Board,
    hover_position: Option<(usize, usize)>,
}

struct Board {
    cells: Vec<bool>,
    width: usize,
    height: usize,
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Self {
            width: WIDTH,
            height: HEIGHT,
            cells: vec![false; WIDTH * HEIGHT],
        };
        board.spawn_glider((1, 1));
        board
    }
}

impl Board {
    fn spawn_glider(&mut self, c: impl Into<Coord>) {
        //   *
        // * *
        //  **
        let c: Coord = c.into();
        self.enliven(c + (3, 1));
        self.enliven(c + (1, 2));
        self.enliven(c + (3, 2));
        self.enliven(c + (2, 3));
        self.enliven(c + (3, 3));
    }

    fn enliven(&mut self, c: impl Into<Coord>) {
        if let Some(i) = self.coord_to_usize(c.into()) {
            self.cells[i] = true;
        }
    }

    fn toggle(&mut self, c: impl Into<Coord>) {
        if let Some(i) = self.coord_to_usize(c.into()) {
            self.cells[i] = !self.cells[i];
        }
    }

    fn clear(&mut self) {
        self.cells = vec![false; WIDTH * HEIGHT];
    }

    fn randomize(&mut self) {
        let mut rng = rand::rng();
        self.cells = self.cells.iter().map(|_| rng.random_bool(0.5)).collect();
    }

    fn step(&mut self) {
        self.cells = self
            .cells
            .iter()
            .enumerate()
            .map(|(i, cell)| {
                let count = self
                    .neighbors(i)
                    .iter()
                    .filter_map(|i| *i)
                    .filter(|i| self.cells[*i])
                    .count();
                (count, cell)
            })
            .map(|(count, cell)| match count {
                0..=1 => false,
                2 => *cell,
                3 => true,
                4.. => false,
            })
            .collect();
    }

    fn usize_to_coord(&self, i: usize) -> Coord {
        // Assuming i is in bounds
        (i % self.width, i / self.width).into()
    }

    fn coord_to_usize(&self, c: Coord) -> Option<usize> {
        // TODO clean types
        if c.x < 0 || c.y < 0 {
            None
        } else if (c.x as usize) < self.width && (c.y as usize) < self.height {
            Some(c.x as usize + c.y as usize * self.width)
        } else {
            None
        }
    }

    fn neighbors(&self, i: usize) -> Vec<Option<usize>> {
        let c = self.usize_to_coord(i);
        vec![
            self.coord_to_usize(c + (-1, -1)),
            self.coord_to_usize(c + (-1, 0)),
            self.coord_to_usize(c + (-1, 1)),
            self.coord_to_usize(c + (0, -1)),
            self.coord_to_usize(c + (0, 1)),
            self.coord_to_usize(c + (1, -1)),
            self.coord_to_usize(c + (1, 0)),
            self.coord_to_usize(c + (1, 1)),
        ]
    }
}

#[derive(Clone, Copy)]
struct Coord {
    x: isize,
    y: isize,
}

impl From<(usize, usize)> for Coord {
    // TODO types are weird
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0 as isize,
            y: value.1 as isize,
        }
    }
}

impl From<(i32, i32)> for Coord {
    // TODO types are weird
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: value.0 as isize,
            y: value.1 as isize,
        }
    }
}

impl std::ops::Add<(i32, i32)> for Coord {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        (self.x as i32 + rhs.0, self.y as i32 + rhs.1).into()
    }
}

fn update(m: Message, state: &mut State) {
    match m {
        Message::Step => state.board.step(),
        Message::Glider => state.board.spawn_glider((1, 1)),
        Message::Clear => state.board.clear(),
        Message::Randomize => state.board.randomize(),
        Message::BoardClick(pos) => state
            .board
            .enliven((pos.0 / SCALE_FACTOR, pos.1 / SCALE_FACTOR)),
        Message::BoardHover(pos) => {
            state.hover_position = Some((pos.0 / SCALE_FACTOR, pos.1 / SCALE_FACTOR));
        }
        Message::BoardExit => state.hover_position = None,
    }
}

fn view(state: &State) -> Node<Message, Layout> {
    //TODO: width height here, but height width in padding
    let mut root = Node::root_node(320, 240).row();

    // This fills the screen, causing the screen to clear each frame
    let mut container = Node::new(Quad::new().fill(MAIN_LIGHT))
        .column()
        .spacing(10)
        .width(Length::Grow)
        .height(Length::Grow);

    let mut row1 = Node::spacer().row().height(Length::Shrink);
    let mut row2 = Node::spacer().row().height(Length::Shrink).spacing(10);

    let step_button_text = Node::new(Text::new("Step".into()).with_font(&font::BLACKLETTER));
    let mut step_button = Node::new(Button::new().on_press(Message::Step))
        .height(Length::Shrink)
        .width(Length::Shrink);
    let mut button_quad = Node::new(
        Quad::new()
            .fill(MAIN_LIGHT)
            .border_thickness(2)
            .border_color(BLUE_DARK),
    )
    .width(Length::Shrink)
    .height(Length::Shrink)
    .padding([0 + 2, 6 + 2, 5 + 2, 6 + 2]);
    button_quad.push(step_button_text);
    step_button.push(button_quad);

    let clear_button_text = Node::new(Text::new("Clear".into()).with_font(&font::BLACKLETTER));
    let mut clear_button = Node::new(Button::new().on_press(Message::Clear))
        .height(Length::Shrink)
        .width(Length::Shrink);
    let mut button_quad = Node::new(
        Quad::new()
            .fill(MAIN_LIGHT)
            .border_thickness(2)
            .border_color(BLUE_DARK),
    )
    .width(Length::Shrink)
    .height(Length::Shrink)
    .padding([0 + 2, 6 + 2, 5 + 2, 6 + 2]);
    button_quad.push(clear_button_text);
    clear_button.push(button_quad);

    let randomize_button_text =
        Node::new(Text::new("Randomize".into()).with_font(&font::BLACKLETTER));
    let mut randomize_button = Node::new(Button::new().on_press(Message::Randomize))
        .height(Length::Shrink)
        .width(Length::Shrink);
    let mut button_quad = Node::new(
        Quad::new()
            .fill(MAIN_LIGHT)
            .border_thickness(2)
            .border_color(BLUE_DARK),
    )
    .width(Length::Shrink)
    .height(Length::Shrink)
    .padding([0 + 2, 6 + 2, 5 + 2, 6 + 2]);
    button_quad.push(randomize_button_text);
    randomize_button.push(button_quad);

    let glider_button_text = Node::new(Text::new("Glider".into()).with_font(&font::BLACKLETTER));
    let mut glider_button = Node::new(Button::new().on_press(Message::Glider))
        .height(Length::Shrink)
        .width(Length::Shrink);
    let mut button_quad = Node::new(
        Quad::new()
            .fill(MAIN_LIGHT)
            .border_thickness(2)
            .border_color(BLUE_DARK),
    )
    .width(Length::Shrink)
    .height(Length::Shrink)
    .padding([0 + 2, 6 + 2, 5 + 2, 6 + 2]);
    button_quad.push(glider_button_text);
    glider_button.push(button_quad);

    let mut image_data: Vec<_> = state
        .board
        .cells
        .iter()
        .map(|c| match c {
            true => 0,
            false => 1,
        })
        .collect();

    if let Some(hover_position) = state.hover_position {
        image_data[hover_position.0 + hover_position.1 * state.board.height] = 2;
    }

    let image = Node::new(
        Image::new(image_data, state.board.width, state.board.height).scale_factor(SCALE_FACTOR),
    )
    .height(Length::Shrink)
    .width(Length::Shrink);

    container.push(Node::new(Quad::new()));

    let mut wrapper = Node::new(
        Quad::new()
            .border_thickness(3)
            .border_color(BLUE_DARK)
            .fill(BLUE_LIGHT),
    )
    .height(Length::Shrink)
    .width(Length::Shrink)
    .padding(10);

    let mut mouse_image_wrapper: Node<Message, _> = MouseArea::new()
        .whenever_down(|pos| Message::BoardClick(pos))
        .on_hover(|pos| Message::BoardHover(pos))
        .on_exit(|| Message::BoardExit)
        .into();

    mouse_image_wrapper.push(image);

    wrapper.push(mouse_image_wrapper);

    row1.push(Node::spacer());
    row1.push(wrapper);
    row1.push(Node::spacer());
    container.push(row1);

    row2.push(Node::spacer());
    row2.push(step_button);
    row2.push(glider_button);
    row2.push(randomize_button);
    row2.push(clear_button);
    row2.push(Node::spacer());
    container.push(row2);

    container.push(Node::spacer());
    root.push(container);
    root
}

fn main() -> Result<(), pixels::Error> {
    let initial_state = State::default();

    icecube::run(initial_state, update, view, 320, 240, MAIN_LIGHT)
}
