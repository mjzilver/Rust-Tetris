
use crate::{blockshape::{BlockShape, self}, board::{Board, Cell, self}, blockcolor::BlockColor};

pub struct Block {
    shape: BlockShape,
    matrix: [[i32; 4]; 4],
    pub color: BlockColor,
    pub status: BlockStatus,
    pub position: (isize, isize),
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
        let self_position = (position.0 as isize, position.1 as isize);

        for y in 0..matrix.len() {
            for x in 0..matrix[y].len() {
                if matrix[y][x] == 1 {
                    
                    board.data[position.0 + y][position.1 + x] = Cell::Color(color.to_color());
                }
            }
        }

        Block{
            shape,
            matrix,
            color,
            status: BlockStatus::Moving,
            position: self_position
        }
    }

    pub fn update(&mut self, board: &mut Board, y_change: i16, x_change: i16) {
        let next_position = (self.position.0 + y_change as isize, self.position.1 + x_change as isize);

        if self.can_move(board, &self.matrix.clone(), next_position, y_change, x_change) {
            // erases the old position
            for y in 0..self.matrix.len() {
                for x in 0..self.matrix[y].len() {
                    if self.matrix[y][x] == 1 {
                        *self.to_absolute(board, y, x) = Cell::Empty;
                    }
                }
            }

            // moves to the new position
            for y in 0..self.matrix.len() {
                for x in 0..self.matrix[y].len() {
                    if self.matrix[y][x] == 1 {
                        *self.to_absolute(board, y, x) = Cell::Color(self.color.to_color())
                    }
                }
            }

            self.position = next_position;
        }
    }

    fn can_move(&mut self, board: &mut Board, matrix: &[[i32; 4]; 4], 
    next_position: (isize, isize),  y_change: i16, x_change: i16)
        -> bool { 
        if self.status != BlockStatus::Moving {
            return false
        }
        // checks if the block can move 
        for y in 0..matrix.len() {
            for x in 0..matrix[y].len() {
                if matrix[y][x] == 1 {
                    if (next_position.0 + y as isize) as usize >= board::HEIGHT {  
                        self.status = BlockStatus::Frozen;
                        return false
                    } else if next_position.1 >= board::WIDTH as isize||(next_position.1 + x as isize) >= board::WIDTH as isize{
                        return false
                    } else if *self.to_absolute(board, y, x) != Cell::Empty {
                        let local_pos = Self::coord_add_i16_to_usize((y, x), (y_change, x_change));
                        if local_pos.0 < matrix.len() && local_pos.1 < matrix[local_pos.0].len() {
                            if matrix[local_pos.0][local_pos.1] == 0 {
                                self.status = BlockStatus::Frozen;
                                return false
                            } 
                        } else {
                            return false
                        }
                    }
                }
            }
        }
        return true
    }

    fn can_rotate(&mut self, board: &mut Board, matrix: &[[i32; 4]; 4]) -> bool {
        if self.status != BlockStatus::Moving {
            return false
        }
        // checks if the block can move 
        for y in 0..matrix.len() {
            for x in 0..matrix[y].len() {
                if matrix[y][x] == 1 {
                    if self.position.0 + y as isize >= board::HEIGHT as isize{  
                        return false
                    } else if self.position.1 >= board::WIDTH as isize ||(self.position.1 + x as isize) >= board::WIDTH as isize {
                        return false
                    } else if *self.to_absolute(board, y, x) != Cell::Empty {
                        if self.matrix[y][x] == 0 {
                            return false
                        }
                    }
                }
            }
        }

        return true;
    }

    fn coord_add_i16_to_usize(u: (usize, usize), i: (i16, i16) ) -> (usize, usize){
        (
            (u.0 as i64 + i.0 as i64) as usize,
            (u.1 as i64 + i.1 as i64) as usize
        )
    }

    fn to_absolute<'a> (& mut self, board: &'a mut Board, y: usize, x: usize ) -> &'a mut Cell {
        return &mut board.data[(self.position.0 + y as isize) as usize][(self.position.1 + x as isize) as usize];
    }

    pub fn move_sideways(&mut self, board: &mut Board, x_change: i16) {
        self.update(board, 0, x_change)
    }

    pub fn move_down(&mut self, board: &mut Board) {
        self.update(board, 1, 0)
    }

    pub fn rotate(&mut self, board: &mut Board) {
        if self.shape == blockshape::BlockShape::OBlock {
            return;
        }

        let mut rotated_matrix: [[i32; 4]; 4] = self.matrix.clone();
        BlockShape::rotate_matrix(&mut rotated_matrix);

        if self.can_rotate(board, &rotated_matrix) {
            // erases the old position
            for y in 0..self.matrix.len() {
                for x in 0..self.matrix[y].len() {
                    if self.matrix[y][x] == 1 {
                        *self.to_absolute(board, y, x) = Cell::Empty;
                    }
                }
            }

            // moves to the new position
            for y in 0..rotated_matrix.len() {
                for x in 0..rotated_matrix[y].len() {
                    if rotated_matrix[y][x] == 1 {
                        *self.to_absolute(board, y, x) = Cell::Color(self.color.to_color());
                    }
                }
            }
            self.matrix = rotated_matrix;
        }
    }
}