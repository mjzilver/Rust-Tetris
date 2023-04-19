mod game;
mod board;
mod renderer;
mod blockshape;
mod block;
mod blockcolor;

fn main() {
    let mut game = game::Game::new();
    game.start_loop()
}
