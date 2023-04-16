
use crate::{blockshape::{BlockShape}, board::{Board, Cell}, blockcolor::BlockColor};

pub struct Block {
    shape: BlockShape,
    color: BlockColor,
    position: (usize, usize),
}

impl Block {
    pub fn new(position: (usize, usize)) -> Block {
        Block{
            shape: BlockShape::random(),
            color: BlockColor::random(),
            position
        }
    }
    pub fn update(&mut self, board: &mut Board) {
        let matrix = self.shape.get_shape();
        for y in 0..matrix.len() {
            for x in 0..matrix[y].len() {
                if matrix[y][x] == 1 {
                    board.data[y + self.position.0][x + self.position.1] = Cell::Color(self.color.to_color())
                }
            }
        }
    }
}