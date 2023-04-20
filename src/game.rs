use std::path::Path;
use crate::{
    block::{Block, BlockStatus}, 
    board::{self, Board},
    gamestate::{GameStatus, GameEvent}, 
    renderer::{self, Renderer, BORDER}};
use piston_window::types::Color;
use piston_window::*;
extern crate piston_window;

/// The color used for the background of the game
const BACK_COLOR: Color = [0.2, 0.5, 0.5, 1.0];
/// The color that is shown when you are game over
const GAME_OVER_COLOR: Color = [0.8, 0.0, 0.0, 0.8];
/// How many seconds it takes before the piece falls one row down
const MOVING_PERIOD: f64 = 0.5;
/// the width of the window 
pub const SCREEN_WIDTH: f64 = (board::WIDTH as f64) * renderer::BLOCK_SIZE;
/// the height of the window 
pub const SCREEN_HEIGHT: f64 = (board::HEIGHT as f64) * renderer::BLOCK_SIZE;
/// y, x where a block will start at when the game is loaded or a new block is created
const BLOCK_SPAWN_POSITION: (isize, isize) = (0, (board::WIDTH as isize / 2) - 1);
/// the width that images should be in to be used as menu items 
const MENU_IMAGE_WIDTH: f64 = 200.0;
/// the height that images should be in to be used as menu items 
const MENU_IMAGE_HEIGHT: f64 = 80.0;
/// where menu images will be placed X - this is used to offset it so it's neatly centered
const IMAGE_LOCATION_X: f64 = (SCREEN_WIDTH - MENU_IMAGE_WIDTH) / 2.0;
/// where menu images will be placed Y - this is used to offset it so it's neatly centered
const IMAGE_LOCATION_Y: f64 = (SCREEN_HEIGHT - MENU_IMAGE_HEIGHT) / 2.0;

/// struct holding all the game data
pub struct Game {
    board: Board,
    block: Block,
    waiting_time: f64,
    score: u16,
    status: GameStatus
}

impl Game {
    /// Creates a new instance of the game
    pub fn new() -> Game {
        let mut board = board::Board::new();
        let block = Block::new(&mut board, BLOCK_SPAWN_POSITION);

        Game {
            board,
            block,
            score: 0,
            waiting_time: 0.0,
            status: GameStatus::Startup
        }
    }

    /// Starts the main game loop and handles user input and rendering
    pub fn start_loop(&mut self) {
        let mut window: PistonWindow = WindowSettings::new("Tetris", (SCREEN_WIDTH, SCREEN_HEIGHT + BORDER))
            .exit_on_esc(true)
            .automatic_close(true)
            .build()
            .expect("Window failed to load");

        let renderer = Renderer::new(&mut window);
        let mut glyphs = window.load_font(Path::new("assets/gillsans.ttf")).expect("Could not load font");

        while let Some(event) = window.next() {
            if let Some(Button::Keyboard(key)) = event.release_args() {
                self.input(&key)
            }

            window.draw_2d(&event, |context, g2d, device| {
                clear(BACK_COLOR, g2d);
                renderer.draw_image("header", 0.0, 0.0, &context, g2d);

                self.board.draw(&context, g2d);
                let text = format!("Current score: {}", self.score);
                renderer.draw_text(&text, &mut glyphs, &context, g2d);
               
                match self.status {
                    GameStatus::Startup => {
                        renderer.draw_image("startup", IMAGE_LOCATION_X, IMAGE_LOCATION_Y, &context, g2d);
                    },
                    GameStatus::GameOver => {
                        renderer::draw_rect(GAME_OVER_COLOR, 0.0, BORDER, SCREEN_WIDTH, SCREEN_HEIGHT , &context, g2d);
                        renderer.draw_image("game_over", IMAGE_LOCATION_X, IMAGE_LOCATION_Y, &context, g2d);
                    },
                    GameStatus::Paused => {
                        renderer.draw_image("paused", IMAGE_LOCATION_X, IMAGE_LOCATION_Y, &context, g2d);
                    },
                    _ => {}
                }
                glyphs.factory.encoder.flush(device);
            });
            event.update(|update_args: &UpdateArgs| self.update(update_args));
        }
    }

    /// Handles user input by updating the game state according to input
    fn input(&mut self, key: &Key) {
        if self.status == GameStatus::Playing {
            match key {
                Key::A => self.block.move_sideways(&mut self.board, -1),
                Key::D => self.block.move_sideways(&mut self.board, 1),
                Key::S => self.block.move_down(&mut self.board),
                Key::R => self.block.rotate(&mut self.board),
                Key::P => self.status.update(GameEvent::Pause),
                _ => {}
            }
        } else {
            match key {
                Key::P => self.status.update(GameEvent::Pause),
                Key::F => { 
                    if  self.status == GameStatus::GameOver {
                        *self = Game::new()
                    }
                    self.status.update(GameEvent::Start)
                },
                _ => {}
            }
        }
    }

    /// Updates the game state based on the elapsed time since the last update
    fn update(&mut self, update_args: &UpdateArgs) {
        self.waiting_time += update_args.dt;

        if self.waiting_time > MOVING_PERIOD && self.status == GameStatus::Playing {
            if self.block.status == BlockStatus::Frozen {
                self.board.update(&mut self.score);

                match Block::next(&mut self.board, BLOCK_SPAWN_POSITION, &self.block) {
                    Some(block) => self.block = block,
                    None => self.status.update(GameEvent::End),
                }
            } else {
                self.block.move_down(&mut self.board);
            }

            self.waiting_time = 0.0;
        }
    }
}
