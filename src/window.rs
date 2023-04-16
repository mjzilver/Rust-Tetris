use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

pub const BLOCK_SIZE: f64 = 25.0;

pub fn draw_block(color: Color, x: f64, y: f64, context: &Context, g2d: &mut G2d) {
    rectangle(
        color,
        [x * BLOCK_SIZE -1.0, y * BLOCK_SIZE -1.0, BLOCK_SIZE-1.0, BLOCK_SIZE-1.0],
        context.transform,
        g2d,
    )
}

pub fn draw_rect(color: Color, x: f64, y: f64, x2: f64, y2: f64, context: &Context, g2d: &mut G2d) {
    rectangle(
        color,
        [x, y, x2, y2],
        context.transform,
        g2d,
    )
}