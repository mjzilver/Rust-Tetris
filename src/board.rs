use piston_window::color::BLACK;
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

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Cell {
    pub color: Color,
    pub status: CellStatus,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CellStatus {
    Empty,
    Moving,
    Frozen,
}

impl Board {
    pub fn new() -> Board {
        let data = vec![vec![Cell{color: BLACK, status: CellStatus::Empty}; WIDTH]; HEIGHT];

        Board { data }
    }

    pub fn draw(&self, context: &Context, g2d: &mut G2d) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.data[y][x].status != CellStatus::Empty {
                    window::draw_block(
                        self.data[y][x].color,
                        x as f64, y as f64,
                        context, g2d,
                    );
                };
            }
        }
    }

    /// This checks for board updates and to remove tiles
    pub fn update(&mut self, score: &mut u16) {
        for y in (0..HEIGHT).rev() {
            let mut cell_count = 0;
            for x in 0..WIDTH {
                if self.data[y][x].status != CellStatus::Empty {
                    cell_count += 1;
                }

            }
            if cell_count == WIDTH {
                for x in 0..WIDTH {
                    self.data[y][x].status = CellStatus::Empty;
                }
                self.move_down(y);
                *score += 1;
                return self.update(score);
            }
        }
    }

    fn move_down(&mut self, y: usize) {
        for y in (0..y).rev() {
            for x in 0..WIDTH {
                if self.data[y][x].status == CellStatus::Frozen {
                    self.data[y + 1][x] = self.data[y][x];
                    self.data[y][x].status = CellStatus::Empty;
                }
            }

        }
    }
}
