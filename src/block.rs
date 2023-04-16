
use crate::{blockshape::{BlockShape}, board::{Board, Cell, self}, blockcolor::BlockColor};

pub struct Block {
    shape: BlockShape,
    color: BlockColor,
    pub status: BlockStatus,
    position: (usize, usize),
}
#[derive(PartialEq)]
pub enum BlockStatus {
    Moving,
    Frozen,
}

impl Block {
    pub fn new(position: (usize, usize)) -> Block {
        Block{
            shape: BlockShape::random(),
            color: BlockColor::random(),
            status: BlockStatus::Moving,
            position
        }
    }

    pub fn update(&mut self, board: &mut Board, y_change: i16, x_change: i16) {
        if self.status == BlockStatus::Moving {
            let matrix = self.shape.get_shape();

            for y in 0..matrix.len() {
                for x in 0..matrix[y].len() {
                    if matrix[y][x] == 1 {
                        board.data[y + self.position.0][x + self.position.1] = Cell::Empty;
                    }
                }
            }

            self.position.0 = (self.position.0 as i64 + y_change as i64) as usize;
            self.position.1 = (self.position.1 as i64 + x_change as i64) as usize;
            for y in 0..matrix.len() {
                for x in 0..matrix[y].len() {
                    let new_y = y + self.position.0;
                    let new_x = x + self.position.1;
                    if matrix[y][x] == 1 {
                        board.data[new_y][new_x] = Cell::Color(self.color.to_color());

                        if new_y == board::HEIGHT -1 {
                            self.status = BlockStatus::Frozen;
                        }
                    }
                }
            }
        }
    }

    pub fn move_sideways(&mut self, board: &mut Board, x_change: i16) {
        self.update(board, 0, x_change)
    }

    pub fn move_down(&mut self, board: &mut Board) {
        self.update(board, 1, 0)
    }
}