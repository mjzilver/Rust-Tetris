use crate::block::{Block, BlockStatus};
use crate::board::{self, Board};
use crate::window;
use piston_window::types::Color;
use piston_window::*;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
const MOVING_PERIOD: f64 = 0.5;
const SCREEN_WIDTH: f64 = (board::WIDTH as f64) * window::BLOCK_SIZE;
const SCREEN_HEIGHT: f64 = (board::HEIGHT as f64) * window::BLOCK_SIZE;

pub struct Game {
    board: Board,
    block: Block,
    waiting_time: f64
}

impl Game {
    pub fn new() -> Game {
        let mut board = board::Board::new();
        let block = Block::new(&mut board, (0, (board::WIDTH / 2) - 1));

        Game { 
            board, 
            block,
            waiting_time: 0.0 
        }
    }

    pub fn start_loop(&mut self) {
        let mut window: PistonWindow = WindowSettings::new(
            "Tetris",
            ( SCREEN_WIDTH, SCREEN_HEIGHT ),
        )
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
            });
            event.update(|arg| {
                self.update(arg)
            });
        }
    }

    fn input (&mut self, key: &Key) {
        match key {
            Key::A => self.block.move_sideways(&mut self.board, -1),
            Key::D => self.block.move_sideways(&mut self.board, 1),
            Key::S => {
                self.block.move_down(&mut self.board); 
                self.waiting_time = 0.0
            },
            Key::R => {self.block.rotate(&mut self.board)},
            Key::X => {*self = Game::new()},

            _ => {}
        }
    }

    fn update(&mut self, arg: &UpdateArgs) {
        self.waiting_time += arg.dt;
    
        if self.waiting_time > MOVING_PERIOD {
            if self.block.status == BlockStatus::Frozen {
                self.block = Block::new(&mut self.board, (0, (board::WIDTH / 2) - 1));
            } else {
                self.block.move_down(&mut self.board);
            }
            
            self.waiting_time = 0.0;
        }
    }
}


