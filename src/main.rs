mod game;
mod board;
mod window;
mod blockshape;
mod block;

fn main() {
    let mut game = game::Game::new();
    game.start_loop()
}
