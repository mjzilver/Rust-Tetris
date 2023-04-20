use rand::Rng;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BlockShape {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl BlockShape {
    /// Generates a random BlockShape
    pub fn random() -> Self {
        let blocks = [
            BlockShape::I,
            BlockShape::J,
            BlockShape::L,
            BlockShape::O,
            BlockShape::S,
            BlockShape::T,
            BlockShape::Z,
        ];
        let mut rng = rand::thread_rng();
        blocks[rng.gen_range(0..blocks.len())]
    }

    /// Generates a random BlockShape except for the provided excluded shape
    pub fn random_except(excluded_shape: BlockShape) -> BlockShape {
        let blocks = [
            BlockShape::I,
            BlockShape::J,
            BlockShape::L,
            BlockShape::O,
            BlockShape::S,
            BlockShape::T,
            BlockShape::Z,
        ];
        let mut rng = rand::thread_rng();
        let mut index = rng.gen_range(0..blocks.len());
        while blocks[index] == excluded_shape {
            index = rng.gen_range(0..blocks.len());
        }
        blocks[index]
    }    

    /// Returns the shape matrix of the BlockShape
    pub fn get_shape(&self) -> [[i32; 4]; 4] {
        match *self {
            BlockShape::I => [
                [0, 0, 0, 0],
                [1, 1, 1, 1],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
            BlockShape::J => [
                [1, 0, 0, 0],
                [1, 1, 1, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
            BlockShape::L => [
                [0, 0, 1, 0],
                [1, 1, 1, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
            BlockShape::O => [
                [0, 1, 1, 0],
                [0, 1, 1, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
            BlockShape::S => [
                [0, 1, 1, 0],
                [1, 1, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
            BlockShape::T => [
                [0, 1, 0, 0],
                [1, 1, 1, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
            BlockShape::Z => [
                [1, 1, 0, 0],
                [0, 1, 1, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
        }
    }

    /// rotates a given a given matrix clockwise 90 degrees
    pub fn rotate_matrix(matrix: &mut [[i32; 4]; 4]) {
        // transpose the matrix
        for i in 0..4 {
            for j in i+1..4 {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }
    
        // flip the matrix horizontally
        for i in 0..4 {
            for j in 0..2 {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[i][3-j];
                matrix[i][3-j] = temp;
            }
        }
    }
}