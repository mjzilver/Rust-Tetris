use std::path::Path;
use std::collections::HashMap;
use piston_window::{types::Color, rectangle, image, Context, G2d, PistonWindow, TextureContext, Texture, Flip, TextureSettings, Transformed, Text, color::BLACK, Glyphs};

/// how big the blocks will be in pixels
pub const BLOCK_SIZE: f64 = 25.0;
/// this is used to take out a border on the blocks to give it a more retro feel
const BLOCK_BORDER_SIZE: f64 = 1.0;
/// the size of the border at the top of the screen wherein the score is displayed
pub const BORDER: f64 = 80.0;

/// Renderer struct holds all the images
pub struct Renderer {
   images: HashMap<String, Texture<gfx_device_gl::Resources>>,
}

impl Renderer {
    /// Creates a new instance of Renderer and loads all necessary textures
    pub fn new(window: &mut PistonWindow) -> Self {
        let mut texture_context: TextureContext<gfx_device_gl::Factory, gfx_device_gl::Resources, gfx_device_gl::CommandBuffer> = TextureContext {
            factory: window.factory.clone(),
            encoder: window.factory.create_command_buffer().into(),
        };

        let textures = HashMap::new();
        let mut renderer = Renderer { images: textures };

        renderer.add_image_file("paused", &mut texture_context);
        renderer.add_image_file("game_over", &mut texture_context);
        renderer.add_image_file("startup", &mut texture_context);
        renderer.add_image_file("header", &mut texture_context);

        renderer
    }

    /// Adds an image file to the textures in the struct according to a given string
    fn add_image_file(&mut self, name: &str, texture_context: &mut TextureContext<gfx_device_gl::Factory, gfx_device_gl::Resources, gfx_device_gl::CommandBuffer> ) {
        let filename = format!("assets/{}.png", name);

        let texture = Texture::from_path(
            texture_context,
            Path::new(filename.as_str()),
            Flip::None,
            &TextureSettings::new(),
        )
        .expect("Failed to load texture");

        self.images.insert(String::from(name), texture);
    }

    /// Draws an image from the textures in the struct according to a given string
    pub fn draw_image(&self, name: &str, x: f64, y: f64, context: &Context, g2d: &mut G2d) {
       image(self.images.get(name).expect("Image failed to load!"), context.transform.trans(x, y), g2d);
    }

    /// Draws text on the screen with a given string
    pub fn draw_text(&self, text: &str, glyphs: &mut Glyphs, context: &Context, g2d: &mut G2d) {
        Text::new_color(BLACK, 20)
        .draw(
            text,
            glyphs,
            &context.draw_state,
            context.transform.trans(10.0, 50.0),
            g2d,
        )
        .unwrap();
    }
}

/// Draws a block with a given color at the specified coordinates on the screen, the size is always BLOCK_SIZE - BLOCK_BORDER_SIZE
pub fn draw_block(color: Color, x: f64, y: f64, context: &Context, g2d: &mut G2d) {
    rectangle(
        color,
        [x * BLOCK_SIZE - BLOCK_BORDER_SIZE, 
            y * BLOCK_SIZE - BLOCK_BORDER_SIZE + BORDER, 
            BLOCK_SIZE - BLOCK_BORDER_SIZE,
            BLOCK_SIZE - BLOCK_BORDER_SIZE],
        context.transform,
        g2d,
    )
}

/// Draws a rectangle at the given coordinates (this allows you to set the size of the rect)
pub fn draw_rect(color: Color, x: f64, y: f64, x2: f64, y2: f64, context: &Context, g2d: &mut G2d) {
    rectangle(
        color,
        [x, y, x2, y2],
        context.transform,
        g2d,
    )
}

