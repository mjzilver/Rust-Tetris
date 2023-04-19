use piston_window::types::Color;
use rand::Rng;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BlockColor {
    Red,
    Blue,
    Green,
    Yellow,
    Purple,
    Orange,
    Cyan,
    Pink,
}

impl BlockColor {
    pub fn to_color(self: &BlockColor) -> Color {
        match self {
            BlockColor::Red => [1.00, 0.00, 0.00, 1.0],
            BlockColor::Blue => [0.00, 0.00, 1.00, 1.0],
            BlockColor::Green => [0.00, 1.00, 0.00, 1.0],
            BlockColor::Yellow => [1.00, 1.00, 0.30, 1.0],
            BlockColor::Purple => [0.60, 0.10, 1.00, 1.0],
            BlockColor::Orange => [1.00, 0.60, 0.00, 1.0],
            BlockColor::Cyan => [0.00, 0.00, 0.50, 1.0],
            BlockColor::Pink => [1.00, 0.20, 1.00, 1.0],
        }
    }

    pub fn random() -> Self {
        let colors = [
            BlockColor::Red,
            BlockColor::Blue,
            BlockColor::Green,
            BlockColor::Yellow,
            BlockColor::Purple,
            BlockColor::Orange,
            BlockColor::Cyan,
            BlockColor::Pink,
        ];
        let mut rng = rand::thread_rng();
        colors[rng.gen_range(0..colors.len())]
    }

    pub fn next_color(current_color: BlockColor) -> BlockColor {
        match current_color {
            BlockColor::Red => BlockColor::Blue,
            BlockColor::Blue => BlockColor::Green,
            BlockColor::Green => BlockColor::Yellow,
            BlockColor::Yellow => BlockColor::Purple,
            BlockColor::Purple => BlockColor::Orange,
            BlockColor::Orange => BlockColor::Cyan,
            BlockColor::Cyan => BlockColor::Pink,
            BlockColor::Pink => BlockColor::Red,
        }
    }
}