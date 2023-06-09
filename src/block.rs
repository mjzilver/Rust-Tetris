use crate::{
    blockcolor::BlockColor,
    blockshape::{self, BlockShape},
    board::{self, Board, Cell, CellStatus},
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
    /// This method creates a new Block instance and adds it to the board at the specified position
    pub fn new(board: &mut Board, position: (isize, isize)) -> Block {
        let shape = BlockShape::random();
        let color = BlockColor::random();
        let matrix = shape.get_shape();

        let mut block = Block {
            shape,
            matrix,
            color,
            status: BlockStatus::Moving,
            position,
        };

        block.add_to_board(board, position);
        block
    }

    /// This method creates a new Block instance with a different shape and color from the old block
    /// Adds it to the board at the specified position if the space is not already filled
    /// If the space is filled it returns a None and the game should end
    pub fn next(board: &mut Board, position: (isize, isize), old_block: &Block) -> Option<Block> {
        let shape = BlockShape::random_except(old_block.shape);
        let color = BlockColor::next_color(old_block.color);
        let matrix = shape.get_shape();

        let mut block = Block {
            shape,
            matrix,
            color,
            status: BlockStatus::Moving,
            position,
        };

        if !block.check_if_space_filled(board) {
            block.add_to_board(board, position);
            Some(block)
        } else {
            None
        }
    }

    /// This method erases the block from the board
    fn erase_from_board(&mut self, board: &mut Board) {
        for y in 0..self.matrix.len() {
            for x in 0..self.matrix[y].len() {
                if self.matrix[y][x] == 1 {
                    self.get_cell_at_current_position(board, y, x).status = CellStatus::Empty;
                }
            }
        }
    }

    /// This method adds the block to the board
    fn add_to_board(&mut self, board: &mut Board, position: (isize, isize)) {
        for y in 0..self.matrix.len() {
            for x in 0..self.matrix[y].len() {
                if self.matrix[y][x] == 1 {
                    self.get_cell_at_specific_position(board, position, y, x).color = self.color.to_color();
                    self.get_cell_at_specific_position(board, position, y, x).status = CellStatus::Moving;
                }
            }
        }
    }

    /// This method updates the block's position on the board
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
        } else if self.status == BlockStatus::Frozen {
            for y in 0..self.matrix.len() {
                for x in 0..self.matrix[y].len() {
                    if self.matrix[y][x] == 1 {
                        self.get_cell_at_current_position(board, y, x).status = CellStatus::Frozen;
                    }
                }
            }
        }
    }

    /// This method checks if the block can move to the specified position
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
                    if Block::is_out_of_bounds(next_position, y, x) {
                        return self.freeze_if(y_change);
                    } else if self.get_cell_at_specific_position(board, next_position, y, x).status != CellStatus::Empty {
                        let local_pos: (usize, usize) = Self::coord_add_i16_to_usize((y, x), (y_change, x_change));
                        if local_pos.0 < self.matrix.len() && local_pos.1 < self.matrix[local_pos.0].len() {
                            if self.matrix[local_pos.0][local_pos.1] == 0 {
                                return self.freeze_if(y_change);
                            }
                        } else {
                            return self.freeze_if(y_change);
                        }
                    }
                }
            }
        }
        true
    }

    /// This method changes the block's status to Frozen if it cannot move downward
    fn freeze_if(&mut self, y_change: i16) -> bool {
        if y_change >= 1 {
            self.status = BlockStatus::Frozen;
        }
        return false
    }

    /// This method checks if the specified position is out of bounds of the game board
    fn is_out_of_bounds(position: (isize, isize), y: usize, x: usize) -> bool {
        position.0 + y as isize >= board::HEIGHT as isize
            || position.1 >= board::WIDTH as isize
            || (position.1 + x as isize) >= board::WIDTH as isize
            || (position.0 + y as isize) < 0
            || (position.1 + x as isize) < 0
    }

    /// This method checks if the block can be rotated
    fn can_rotate(&mut self, board: &mut Board, matrix: &[[i32; 4]; 4]) -> bool {
        if self.status != BlockStatus::Moving {
            return false;
        }
        // checks if the block can be rotated (yes this is different than moving)
        for y in 0..matrix.len() {
            for x in 0..matrix[y].len() {
                if matrix[y][x] == 1 {
                    // is it inside the board
                    if Block::is_out_of_bounds(self.position, y, x) {
                        return false
                    // is the block it is moving into empty
                    } else if self.get_cell_at_current_position(board, y, x).status != CellStatus::Empty && self.matrix[y][x] == 0  {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// This method checks if the space the block would fill is already taken and returns true if it is taken
    fn check_if_space_filled(&mut self, board: &mut Board) -> bool {
        for y in 0..self.matrix.len() {
            for x in 0..self.matrix[y].len() {
                if self.matrix[y][x] == 1 && self.get_cell_at_current_position(board, y, x).status != CellStatus::Empty {
                    return true
                }
            }
        }
        false
    }

    /// This helper function takes two tuples u: (usize, usize), i: (i16, i16)
    /// Add them together and returns a (usize, usize)
    fn coord_add_i16_to_usize(u: (usize, usize), i: (i16, i16)) -> (usize, usize) {
        (
            (u.0 as i64 + i.0 as i64) as usize,
            (u.1 as i64 + i.1 as i64) as usize,
        )
    }

    /// This gets the associated cell from the board given the current position and the associated y and x inside the block matrix
    fn get_cell_at_current_position<'a>(&mut self, board: &'a mut Board, y: usize, x: usize) -> &'a mut Cell {
        &mut board.data[(self.position.0 + y as isize) as usize][(self.position.1 + x as isize) as usize]
    }

    /// This gets the associated cell from the board given a specified position and the associated y and x inside the block matrix
    fn get_cell_at_specific_position<'a>(
        &mut self, board: &'a mut Board,
        position: (isize, isize),
        y: usize, x: usize,
    ) -> &'a mut Cell {
        &mut board.data[(position.0 + y as isize) as usize][(position.1 + x as isize) as usize]
    }

    /// This method tries to move a block sideways on the game board
    pub fn move_sideways(&mut self, board: &mut Board, x_change: i16) {
        self.update(board, 0, x_change)
    }

    /// This method tries to move a block down on the game board
    pub fn move_down(&mut self, board: &mut Board) {
        self.update(board, 1, 0)
    }

    /// This method tries to rotate a block 90 degrees clockwise on the game board
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


#[cfg(test)]
mod block_tests {
    use super::*;
    #[test]
    fn test_new_block() {
        let mut board = Board::new();
        let position = (0, 0);
        let mut block = Block::new(&mut board, position);

        // check if the block is added to the board
        for y in 0..block.matrix.len() {
            for x in 0..block.matrix[y].len() {
                if block.matrix[y][x] == 1 {
                    assert_eq!(block.get_cell_at_current_position(&mut board, y, x).status, CellStatus::Moving);
                }
            }
        }
    }

    #[test]
    fn test_erase_from_board() {
        let mut board = Board::new();
        let position = (0, 0);
        let mut block = Block::new(&mut board, position);

        // erase the block from the board
        block.erase_from_board(&mut board);

        // check if the block is erased from the board
        for y in 0..block.matrix.len() {
            for x in 0..block.matrix[y].len() {
                if block.matrix[y][x] == 1 {
                    assert_eq!(block.get_cell_at_current_position(&mut board, y, x).status, CellStatus::Empty);
                }
            }
        }
    }

    #[test]
    fn test_add_to_board() {
        let mut board = Board::new();
        let position = (0, 0);
        let mut block = Block::new(&mut board, position);

        // erase the block from the board
        block.erase_from_board(&mut board);

        // add the block to the board at a new position
        let new_position = (1, 1);
        block.add_to_board(&mut board, new_position);

        block.position = new_position;

        // check if the block is at the new position
        for y in 0..block.matrix.len() {
            for x in 0..block.matrix[y].len() {
                if block.matrix[y][x] == 1 {
                    assert_eq!(block.get_cell_at_current_position(&mut board, y, x).status, CellStatus::Moving);
                }
            }
        }
    } 

    #[test]
    fn test_can_rotate() {
        let mut board = Board::new();
        let position = (1, 1);
        let i_position: (isize, isize) = (1, 1);
        let mut block = Block::new(&mut board, position);

        let mut rotated_matrix: [[i32; 4]; 4] = block.matrix;
        BlockShape::rotate_matrix(&mut rotated_matrix);    

        // it can rotate
        assert_eq!(block.can_rotate(&mut board, &rotated_matrix), true);

        // it cant rotate into other blocks
        for y in 0..block.matrix.len() {
            for x in 0..block.matrix[y].len() {
                if block.matrix[y][x] == 0 {
                    block.get_cell_at_specific_position(&mut board, i_position, y, x).status = CellStatus::Frozen;
                }
            }
        } 
        // it cannot rotate because the cells inserted above are in the way
        assert_eq!(block.can_rotate(&mut board, &rotated_matrix), false);

        // it cannot rotate out of bounds
        block.position = (20, 20);
        assert_eq!(block.can_rotate(&mut board, &rotated_matrix), false);

    }

    #[test]
    fn test_game_over() {
        let mut board = Board::new();
        let position = (1, 1);
        let block = Block::new(&mut board, position);

        let new_block: Option<Block> = Block::next(&mut board, (5, 1), &block); 
        assert_eq!(new_block.is_some(), true);

        let new_block2: Option<Block> = Block::next(&mut board, (1, 1), &block); 
        assert_eq!(new_block2.is_none(), true);
    }
}