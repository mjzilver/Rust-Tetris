mod game;
mod board;
mod renderer;
mod blockshape;
mod block;
mod blockcolor;
mod gamestate;
mod audio;

/// This is the entry point for the application, which creates a new game instance and starts its game loop
fn main() {
    let mut game = game::Game::new();
    game.start_loop()
}
