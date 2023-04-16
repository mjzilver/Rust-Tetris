pub enum BlockShape {
    IBlock,
    JBlock,
    LBlock,
    OBlock,
    SBlock,
    TBlock,
    ZBlock,
}

impl BlockShape {
    pub fn new(shape: &str) -> BlockShape {
        match shape {
            "I" => BlockShape::IBlock,
            "J" => BlockShape::JBlock,
            "L" => BlockShape::LBlock,
            "O" => BlockShape::OBlock,
            "S" => BlockShape::SBlock,
            "T" => BlockShape::TBlock,
            "Z" => BlockShape::ZBlock,
            _ => panic!("Invalid block shape!"),
        }
    }

    pub fn get_shape(&self) -> [[i32; 4]; 4] {
        match *self {
            BlockShape::IBlock => [
                [0, 0, 0, 0],
                [1, 1, 1, 1],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
            BlockShape::JBlock => [
                [1, 0, 0, 0],
                [1, 1, 1, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
            BlockShape::LBlock => [
                [0, 0, 1, 0],
                [1, 1, 1, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
            BlockShape::OBlock => [
                [0, 1, 1, 0],
                [0, 1, 1, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
            BlockShape::SBlock => [
                [0, 1, 1, 0],
                [1, 1, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
            BlockShape::TBlock => [
                [0, 1, 0, 0],
                [1, 1, 1, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
            BlockShape::ZBlock => [
                [1, 1, 0, 0],
                [0, 1, 1, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
        }
    }
}