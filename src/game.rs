use std::path::Path;

use crate::block::{Block, BlockStatus};
use crate::board::{self, Board};
use crate::renderer::{self, Renderer, BORDER};
use piston_window::types::Color;
use piston_window::*;
extern crate piston_window;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
const GAME_OVER_COLOR: Color = [0.8, 0.0, 0.0, 0.8];
const MOVING_PERIOD: f64 = 0.3;
pub const SCREEN_WIDTH: f64 = (board::WIDTH as f64) * renderer::BLOCK_SIZE;
pub const SCREEN_HEIGHT: f64 = (board::HEIGHT as f64) * renderer::BLOCK_SIZE;
const BLOCK_SPAWN_POSITION: (isize, isize) = (0, (board::WIDTH as isize / 2) - 1);
const MENU_IMAGE_WIDTH: f64 = 200.0;
const MENU_IMAGE_HEIGHT: f64 = 80.0;
const IMAGE_LOCATION_X: f64 = (SCREEN_WIDTH - MENU_IMAGE_WIDTH) / 2.0;
const IMAGE_LOCATION_Y: f64 = (SCREEN_HEIGHT - MENU_IMAGE_HEIGHT) / 2.0;

pub struct Game {
    board: Board,
    block: Block,
    waiting_time: f64,
    score: u16,
    status: GameStatus
}

#[derive(PartialEq)]
pub enum GameStatus {
    Startup,
    Paused,
    Playing,
    GameOver,
}

impl Game {
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

    pub fn start_loop(&mut self) {
        let mut window: PistonWindow = WindowSettings::new("Tetris", (SCREEN_WIDTH, SCREEN_HEIGHT + BORDER))
            .exit_on_esc(true)
            .automatic_close(true)
            .build()
            .expect("Window failed to load");

        let renderer = Renderer::new(&mut window);
        let mut glyphs = window.load_font(Path::new("gillsans.ttf")).expect("Could not load font");

        while let Some(event) = window.next() {
            if let Some(Button::Keyboard(key)) = event.release_args() {
                self.input(&key)
            }

            window.draw_2d(&event, |context, g2d, device| {
                clear(BACK_COLOR, g2d);
                //renderer::draw_rect(BORDER_COLOR, 0.0, 0.0, SCREEN_WIDTH as f64, BORDER, &context, g2d);
                renderer.draw_image("header", 0.0, 0.0, &context, g2d);

                self.board.draw(&context, g2d);
                let text = format!("Current score: {}", self.score);
                renderer.draw_text(&text, &mut glyphs, &context, g2d);
               
                match self.status {
                    GameStatus::Startup => {
                        renderer.draw_image("startup", IMAGE_LOCATION_X, IMAGE_LOCATION_Y, &context, g2d);
                    },
                    GameStatus::GameOver => {
                        renderer::draw_rect(GAME_OVER_COLOR, 0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT , &context, g2d);
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

    fn input(&mut self, key: &Key) {
        match key {
            Key::A => self.block.move_sideways(&mut self.board, -1),
            Key::D => self.block.move_sideways(&mut self.board, 1),
            Key::S => self.block.move_down(&mut self.board),
            Key::R => self.block.rotate(&mut self.board),
            Key::X => *self = Game::new(),
            Key::P => { 
                if self.status == GameStatus::Startup {
                    self.status = GameStatus::Playing
                } else if self.status == GameStatus::Paused {
                    self.status = GameStatus::Playing
                } else if self.status == GameStatus::Playing { 
                    self.status = GameStatus::Paused 
                }
             },
            _ => {}
        }
    }

    fn update(&mut self, update_args: &UpdateArgs) {
        self.waiting_time += update_args.dt;

        if self.waiting_time > MOVING_PERIOD && self.status == GameStatus::Playing {
            if self.block.status == BlockStatus::Frozen {
                self.board.update(&mut self.score);

                match Block::next(&mut self.board, BLOCK_SPAWN_POSITION, &self.block) {
                    Some(block) => self.block = block,
                    None => self.status = GameStatus::GameOver,
                }
            } else {
                self.block.move_down(&mut self.board);
            }

            self.waiting_time = 0.0;
        }
    }
}
