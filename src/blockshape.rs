use rand::Rng;

#[derive(Debug, PartialEq, Copy, Clone)]
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
    pub fn random() -> Self {
        let blocks = [
            BlockShape::IBlock,
            BlockShape::JBlock,
            BlockShape::LBlock,
            BlockShape::OBlock,
            BlockShape::SBlock,
            BlockShape::TBlock,
            BlockShape::ZBlock,
        ];
        let mut rng = rand::thread_rng();
        blocks[rng.gen_range(0..blocks.len())]
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