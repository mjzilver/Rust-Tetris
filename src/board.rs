use piston_window::types::Color;
use piston_window::Context;
use piston_window::G2d;

use crate::window;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 16;

#[derive(PartialEq)]
pub struct Board {
    pub data: Vec<Vec<Cell>>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Cell {
    Empty,
    Color(Color),
}

impl Cell {
    pub fn get_color(&self) -> Option<Color> {
        match self {
            Cell::Color(color) => Some(*color),
            _ => None,
        }
    }
}

impl Board {
    pub fn new() -> Board {
        let data = vec![vec![Cell::Empty; WIDTH]; HEIGHT];

        Board { data }
    }

    pub fn draw(&self, context: &Context, g2d: &mut G2d) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.data[y][x] != Cell::Empty {
                    window::draw_block(
                        self.data[y][x].get_color().unwrap(),
                        x as f64, y as f64,
                        context, g2d,
                    );
                };
            }
        }
    }
}
