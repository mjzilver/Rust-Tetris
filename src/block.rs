use crate::{blockshape::BlockShape, board::Board};

pub struct Block {
    shape: BlockShape,
    position: (i32, i32),
}

impl Block {
    pub fn new(position: (i32, i32)) -> Block {
        Block{
            shape: BlockShape::new("O"),
            position
        }
    }
    pub fn update(&mut self, board: &mut Board) {}
}