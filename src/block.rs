use crate::{
    blockcolor::BlockColor,
    blockshape::{self, BlockShape},
    board::{self, Board, Cell},
};

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

        Block {
            shape,
            matrix,
            color,
            status: BlockStatus::Moving,
            position: self_position,
        }
    }

    // erases the block from the board
    fn erase_from_board(&mut self, board: &mut Board) {
        for y in 0..self.matrix.len() {
            for x in 0..self.matrix[y].len() {
                if self.matrix[y][x] == 1 {
                    *self.to_absolute(board, y, x) = Cell::Empty;
                }
            }
        }
    }

    // add the block from the board
    fn add_to_board(&mut self, board: &mut Board, position: (isize, isize)) {
        for y in 0..self.matrix.len() {
            for x in 0..self.matrix[y].len() {
                if self.matrix[y][x] == 1 {
                    *self.to_absolute_from(board, position, y, x) =
                        Cell::Color(self.color.to_color())
                }
            }
        }
    }

    pub fn update(&mut self, board: &mut Board, y_change: i16, x_change: i16) {
        let next_position = (
            self.position.0 + y_change as isize,
            self.position.1 + x_change as isize,
        );
        // checks if the position is valid to move to
        if self.can_move(board, next_position, y_change, x_change) {
            // erases the old position
            self.erase_from_board(board);
            // moves to the new position
            self.add_to_board(board, next_position);
            // change the position
            self.position = next_position;
        }
    }

    fn can_move(
        &mut self, board: &mut Board,
        next_position: (isize, isize),
        y_change: i16,  x_change: i16,
    ) -> bool {
        if self.status != BlockStatus::Moving {
            return false;
        }
        // checks if the block can move
        for y in 0..self.matrix.len() {
            for x in 0..self.matrix[y].len() {
                if self.matrix[y][x] == 1 {
                    if (next_position.0 + y as isize) as usize >= board::HEIGHT {
                        self.status = BlockStatus::Frozen;
                        return false;
                    } else if Block::is_out_of_bounds(next_position, y, x) {
                        return false;
                    } else if *self.to_absolute_from(board, next_position, y, x) != Cell::Empty {
                        let local_pos = Self::coord_add_i16_to_usize((y, x), (y_change, x_change));
                        if local_pos.0 < self.matrix.len() && local_pos.1 < self.matrix[local_pos.0].len() {
                            if self.matrix[local_pos.0][local_pos.1] == 0 {
                                self.status = BlockStatus::Frozen;
                                return false;
                            }
                        } else {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    fn is_out_of_bounds( position: (isize, isize), y: usize, x: usize) -> bool {
        position.0 + y as isize >= board::HEIGHT as isize
            || position.1 >= board::WIDTH as isize
            || (position.1 + x as isize) >= board::WIDTH as isize
            || (position.0 + y as isize) < 0
            || (position.1 + x as isize) < 0
    }

    fn can_rotate(&mut self, board: &mut Board, matrix: &[[i32; 4]; 4]) -> bool {
        if self.status != BlockStatus::Moving {
            return false;
        }
        // checks if the block can be rotated (yes this is different than moving)
        for y in 0..matrix.len() {
            for x in 0..matrix[y].len() {
                if matrix[y][x] == 1 {
                    if Block::is_out_of_bounds(self.position, y, x) {
                        return false
                    } else if *self.to_absolute(board, y, x) != Cell::Empty && self.matrix[y][x] == 0  {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn coord_add_i16_to_usize(u: (usize, usize), i: (i16, i16)) -> (usize, usize) {
        (
            (u.0 as i64 + i.0 as i64) as usize,
            (u.1 as i64 + i.1 as i64) as usize,
        )
    }

    fn to_absolute<'a>(&mut self, board: &'a mut Board, y: usize, x: usize) -> &'a mut Cell {
        &mut board.data[(self.position.0 + y as isize) as usize][(self.position.1 + x as isize) as usize]
    }

    fn to_absolute_from<'a>(
        &mut self, board: &'a mut Board,
        position: (isize, isize),
        y: usize, x: usize,
    ) -> &'a mut Cell {
        &mut board.data[(position.0 + y as isize) as usize][(position.1 + x as isize) as usize]
    }

    pub fn move_sideways(&mut self, board: &mut Board, x_change: i16) {
        self.update(board, 0, x_change)
    }

    pub fn move_down(&mut self, board: &mut Board) {
        self.update(board, 1, 0)
    }

    pub fn rotate(&mut self, board: &mut Board) {
        if self.shape == blockshape::BlockShape::O {
            return;
        }

        let mut rotated_matrix: [[i32; 4]; 4] = self.matrix;
        BlockShape::rotate_matrix(&mut rotated_matrix);

        if self.can_rotate(board, &rotated_matrix) {
            // erases the old position
            self.erase_from_board(board);

            // changes the matrix to be rotated
            self.matrix = rotated_matrix;

            // moves to the new position
            self.add_to_board(board, self.position);
            
        }
    }
}
