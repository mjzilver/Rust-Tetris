use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

pub const BLOCK_SIZE: f64 = 25.0;

pub fn draw_block(color: Color, x: f64, y: f64, context: &Context, g2d: &mut G2d) {
    rectangle(
        color,
        [x * BLOCK_SIZE, y * BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE],
        context.transform,
        g2d,
    )
}
