mod game;
mod board;
mod renderer;
mod blockshape;
mod block;
mod blockcolor;
mod gamestate;

fn main() {
    let mut game = game::Game::new();
    game.start_loop()
}
