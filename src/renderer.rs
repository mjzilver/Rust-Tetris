use std::path::Path;
use std::collections::HashMap;
use gfx_device_gl::{Resources};
use piston_window::{types::Color, rectangle, image, Context, G2d, PistonWindow, TextureContext, Texture, Flip, TextureSettings, Transformed, Text, color::BLACK, Glyphs};

use crate::game;

pub const BLOCK_SIZE: f64 = 25.0;
const BLOCK_BORDER_SIZE: f64 = 1.0;
const MENU_IMAGE_WIDTH: f64 = 200.0;
const MENU_IMAGE_HEIGHT: f64 = 80.0;

pub struct Renderer {
   textures: HashMap<String, Texture<Resources>>,
}

impl Renderer {
    pub fn new(window: &mut PistonWindow) -> Self {
        let mut texture_context: TextureContext<gfx_device_gl::Factory, Resources, gfx_device_gl::CommandBuffer> = TextureContext {
            factory: window.factory.clone(),
            encoder: window.factory.create_command_buffer().into(),
        };

        let textures = HashMap::new();
        let mut renderer = Renderer { textures };

        renderer.add_image_file("paused", &mut texture_context);
        renderer.add_image_file("game_over", &mut texture_context);

        renderer
    }

    fn add_image_file(&mut self, name: &str, texture_context: &mut TextureContext<gfx_device_gl::Factory, Resources, gfx_device_gl::CommandBuffer> ) {
        let filename: &str = &(name.to_owned() + ".png");

        let texture = Texture::from_path(
            texture_context,
            Path::new(filename),
            Flip::None,
            &TextureSettings::new(),
        )
        .expect("Failed to load texture");

        self.textures.insert(String::from(name), texture);
    }

    pub fn draw_image(&self, name: &str, context: &Context, g2d: &mut G2d) {
        let x = (game::SCREEN_WIDTH - MENU_IMAGE_WIDTH) / 2.0;
        let y = (game::SCREEN_HEIGHT - MENU_IMAGE_HEIGHT) / 2.0;

       image(self.textures.get(name).expect("Image failed to load!"), context.transform.trans(x, y), g2d);
    }

    pub fn draw_text(&self, text: &str, glyphs: &mut Glyphs, context: &Context, g2d: &mut G2d) {
        Text::new_color(BLACK, 20)
        .draw(
            text,
            glyphs,
            &context.draw_state,
            context.transform.trans(10.0, 30.0),
            g2d,
        )
        .unwrap();
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

