use crate::block::{Block, BlockStatus};
use crate::board::{self, Board};
use crate::window;
use piston_window::types::Color;
use piston_window::*;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
const GAME_OVER_COLOR: Color = [0.8, 0.0, 0.0, 0.8];
const MOVING_PERIOD: f64 = 0.3;
const SCREEN_WIDTH: f64 = (board::WIDTH as f64) * window::BLOCK_SIZE;
const SCREEN_HEIGHT: f64 = (board::HEIGHT as f64) * window::BLOCK_SIZE;
const BLOCK_SPAWN_POSITION: (isize, isize) = (0, (board::WIDTH as isize / 2) - 1);

pub struct Game {
    board: Board,
    block: Block,
    waiting_time: f64,
    score: u16,
    game_over: bool,
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
            game_over: false
        }
    }

    pub fn start_loop(&mut self) {
        let mut window: PistonWindow = WindowSettings::new("Tetris", (SCREEN_WIDTH, SCREEN_HEIGHT))
            .exit_on_esc(true)
            .automatic_close(true)
            .build()
            .expect("Window failed to load");

        while let Some(event) = window.next() {
            if let Some(Button::Keyboard(key)) = event.release_args() {
                self.input(&key)
            }

            window.draw_2d(&event, |context, g2d, _| {
                clear(BACK_COLOR, g2d);
                self.board.draw(&context, g2d);
                if self.game_over {
                    window::draw_rect(GAME_OVER_COLOR, 0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT , &context, g2d)
                }
            });
            event.update(|arg| self.update(arg));
        }
    }

    fn input(&mut self, key: &Key) {
        match key {
            Key::A => self.block.move_sideways(&mut self.board, -1),
            Key::D => self.block.move_sideways(&mut self.board, 1),
            Key::S => self.block.move_down(&mut self.board),
            Key::R => self.block.rotate(&mut self.board),
            Key::X => *self = Game::new(),
            _ => {}
        }
    }

    fn update(&mut self, arg: &UpdateArgs) {
        self.waiting_time += arg.dt;

        if self.waiting_time > MOVING_PERIOD && !self.game_over {
            if self.block.status == BlockStatus::Frozen {
                self.board.update(&mut self.score);

                match Block::next(&mut self.board, BLOCK_SPAWN_POSITION, &self.block) {
                    Some(block) => self.block = block,
                    None => self.game_over = true,
                }
            } else {
                self.block.move_down(&mut self.board);
            }

            self.waiting_time = 0.0;
        }
    }
}
