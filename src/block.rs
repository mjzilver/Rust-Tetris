
use crate::{blockshape::{BlockShape}, board::{Board, Cell, self}, blockcolor::BlockColor};

pub struct Block {
    shape: BlockShape,
    pub color: BlockColor,
    pub status: BlockStatus,
    pub position: (usize, usize),
}
#[derive(PartialEq)]
pub enum BlockStatus {
    Moving,
    Frozen,
}

impl Block {
    pub fn new(board: &mut Board, position: (usize, usize)) -> Block {
        let shape = BlockShape::random();
        let color = BlockColor::random();
        let matrix = shape.get_shape();

        for y in 0..matrix.len() {
            for x in 0..matrix[y].len() {
                if matrix[y][x] == 1 {
                    board.data[position.0 + y][position.1 + x] = Cell::Color(color.to_color());
                }
            }
        }

        Block{
            shape,
            color,
            status: BlockStatus::Moving,
            position
        }
    }

    pub fn update(&mut self, board: &mut Board, y_change: i16, x_change: i16) {
        let matrix = self.shape.get_shape();
        let next_position = Self::coord_add_i16_to_usize(self.position, (y_change, x_change));

        for y in (0..matrix.len()).rev() {
            for x in 0..matrix[y].len() {
                if matrix[y][x] == 1 && self.status == BlockStatus::Moving {
                    if next_position.0 + y == board::HEIGHT {  
                        return self.status = BlockStatus::Frozen
                    } else if board.data[next_position.0 + y][next_position.1 + x] != Cell::Empty {
                        let local_pos = Self::coord_add_i16_to_usize((y, x), (y_change, x_change));
                        if matrix[local_pos.0][local_pos.1] == 0 {
                            return self.status = BlockStatus::Frozen
                        } 
                    }
                }
            }
        }

        if self.status == BlockStatus::Moving {
            for y in 0..matrix.len() {
                for x in 0..matrix[y].len() {
                    if matrix[y][x] == 1 {
                        board.data[y + self.position.0][x + self.position.1] = Cell::Empty;
                    }
                }
            }

            for y in 0..matrix.len() {
                for x in 0..matrix[y].len() {
                    if matrix[y][x] == 1 {
                        board.data[y + next_position.0][x + next_position.1] = Cell::Color(self.color.to_color());
                    }
                }
            }

            self.position = next_position;
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