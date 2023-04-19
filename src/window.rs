use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

pub const BLOCK_SIZE: f64 = 25.0;
const BLOCK_BORDER_SIZE: f64 = 1.0;

pub fn draw_block(color: Color, x: f64, y: f64, context: &Context, g2d: &mut G2d) {
    rectangle(
        color,
        [x * BLOCK_SIZE - BLOCK_BORDER_SIZE, 
            y * BLOCK_SIZE - BLOCK_BORDER_SIZE, 
            BLOCK_SIZE - BLOCK_BORDER_SIZE,
            BLOCK_SIZE - BLOCK_BORDER_SIZE],
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

pub(crate) fn draw_image() {
    todo!()
}