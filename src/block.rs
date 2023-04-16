
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
            let next_position = Self::coord_add_i16_to_usize(self.position, (y_change, x_change));

            for y in 0..matrix.len() {
                for x in 0..matrix[y].len() {
                    if matrix[y][x] == 1 {
                        if next_position.0 + y == board::HEIGHT {  
                            return self.status = BlockStatus::Frozen
                        } else if board.data[next_position.0 + y][next_position.1] == Cell::Empty {
                            board.data[y + self.position.0][x + self.position.1] = Cell::Empty;
                        } else {
                            let local_pos = Self::coord_add_i16_to_usize((y, x), (y_change, x_change));
                            if matrix[local_pos.0][local_pos.1] == 0 {
                                return self.status = BlockStatus::Frozen
                            } else {
                                board.data[y + self.position.0][x + self.position.1] = Cell::Empty;
                            }
                        }
                    }
                }
            }

            self.position = next_position;

            for y in 0..matrix.len() {
                for x in 0..matrix[y].len() {
                    let new_y = y + self.position.0;
                    let new_x = x + self.position.1;
                    if matrix[y][x] == 1 {
                        board.data[new_y][new_x] = Cell::Color(self.color.to_color());
                    }
                }
            }
        }
    }

    fn coord_add_i16_to_usize(u: (usize, usize), i: (i16, i16) ) -> (usize, usize){
        (
            (u.0 as i64 + i.0 as i64) as usize,
            (u.1 as i64 + i.1 as i64) as usize
        )
    }

    pub fn move_sideways(&mut self, board: &mut Board, x_change: i16) {
        self.update(board, 0, x_change)
    }

    pub fn move_down(&mut self, board: &mut Board) {
        self.update(board, 1, 0)
    }
}