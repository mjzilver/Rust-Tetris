use std::path::Path;
use std::collections::HashMap;
use gfx_device_gl::Resources;
use piston_window::{types::Color, rectangle, image, Context, G2d, PistonWindow, TextureContext, Texture, Flip, TextureSettings, Transformed};

use crate::game;

pub const BLOCK_SIZE: f64 = 25.0;
const BLOCK_BORDER_SIZE: f64 = 1.0;
const IMAGE_WIDTH: f64 = 200.0;
const IMAGE_HEIGHT: f64 = 80.0;

pub struct Renderer {
   textures: HashMap<String, Texture<Resources>>,
}

impl Renderer {
    pub fn new(window: &mut PistonWindow) -> Self {
        let mut texture_context = TextureContext {
            factory: window.factory.clone(),
            encoder: window.factory.create_command_buffer().into(),
        };

        let mut textures = HashMap::new();

        let texture = Texture::from_path(
            &mut texture_context,
            Path::new("paused.png"),
            Flip::None,
            &TextureSettings::new(),
        )
        .expect("Failed to load texture");

        textures.insert(String::from("paused"), texture);

        let texture = Texture::from_path(
            &mut texture_context,
            Path::new("game_over.png"),
            Flip::None,
            &TextureSettings::new(),
        )
        .expect("Failed to load texture");

        textures.insert(String::from("game_over"), texture);

        Renderer { textures }
    }

    pub fn draw_image(&self, name: &str, context: &Context, g2d: &mut G2d) {
        let x = (game::SCREEN_WIDTH - IMAGE_WIDTH) / 2.0;
        let y = (game::SCREEN_HEIGHT - IMAGE_HEIGHT) / 2.0;

       image(self.textures.get(name).expect("Image failed to load!"), context.transform.trans(x, y), g2d);
    }
}

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

