use piston_window::color::BLACK;
use piston_window::types::Color;
use piston_window::Context;
use piston_window::G2d;

use crate::renderer;

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
                    renderer::draw_block(
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

#[cfg(test)]
mod boards_tests {
    use super::*;

    #[test]
    fn test_update() {
        let mut board = Board::new();

        let test_cell: Cell = Cell{
            color: BLACK,
            status: CellStatus::Frozen,
        };

        /*  this is how the bottom rows will look like 
            [0, 1, 0, 0..] // this row will slide down
            [1, 1, 1, 1..] // this row will be completed and removed
        */
        
        // fills the bottom row 
        for x in 0..WIDTH {
            board.data[HEIGHT - 1][x] = test_cell.clone();
        }

        // block above the completed line to fall into place
        board.data[HEIGHT - 2][1] = test_cell.clone();

        let mut score = 0;
        board.update(&mut score);

        for x in 0..WIDTH {
            // the cell at (HEIGHT - 1, 1) will be filled with the cell that was spawned above it
            // this is to test if the blocks move down
            if x == 1 { 
                assert_eq!(board.data[HEIGHT - 1][x].status, CellStatus::Frozen);
            } else {
                assert_eq!(board.data[HEIGHT - 1][x].status, CellStatus::Empty);
            }
        }
        // check if the block fell down and was removed from the original place
        assert_eq!(board.data[HEIGHT - 2][1].status, CellStatus::Empty);

        // check that the score increased when 1 row was completed
        assert_eq!(score, 1);
    }


    #[test]
    fn test_update_multiple_rows() {
        let mut board = Board::new();

        let test_cell: Cell = Cell{
            color: BLACK,
            status: CellStatus::Frozen,
        };

        /*  this is how the bottom rows will look like 
            [0, 1, 0, 0..] // this row will slide down
            [1, 1, 1, 1..] // this row will be completed and removed
            [1, 1, 1, 1..] // this row will be completed and removed
        */
        
        // fills the bottom rows 
        for x in 0..WIDTH {
            board.data[HEIGHT - 1][x] = test_cell.clone();
        }
        for x in 0..WIDTH {
            board.data[HEIGHT - 2][x] = test_cell.clone();
        }

        // block above the completed line to fall into place
        board.data[HEIGHT - 3][1] = test_cell.clone();
        board.data[HEIGHT - 4][1] = test_cell.clone();

        let mut score = 0;
        board.update(&mut score);

        for x in 0..WIDTH {
            if x == 1 { 
                assert_eq!(board.data[HEIGHT - 1][x].status, CellStatus::Frozen);
            } else {
                assert_eq!(board.data[HEIGHT - 1][x].status, CellStatus::Empty);
            }
        }

        // check if the 2nd row is empty
        for x in 0..WIDTH {
            if x == 1 { 
                assert_eq!(board.data[HEIGHT - 1][x].status, CellStatus::Frozen);
            } else {
                assert_eq!(board.data[HEIGHT - 1][x].status, CellStatus::Empty);
            }
        }
        // check if the block fell down and was removed from the original place
        assert_eq!(board.data[HEIGHT - 3][1].status, CellStatus::Empty);
        assert_eq!(board.data[HEIGHT - 4][1].status, CellStatus::Empty);

        // check that the score increased by 2 when 2 row was completed
        assert_eq!(score, 2);
    }
}