/// This enum represents the possible states of a game
#[derive(PartialEq)]
pub enum GameStatus {
    Startup,
    Playing,
    Paused,
    GameOver,
}
/// This  enum represents the possible events that can occur during a game
pub enum GameEvent {
    Start,
    Pause,
    End,
}

impl GameStatus {
    /// Updates the GameStatus based on the given GameEvent
    pub fn update(&mut self, event: GameEvent) {
        match self {
            GameStatus::Startup => {
                match event {
                    GameEvent::Start => {
                        *self = GameStatus::Playing;
                    }
                    _ => {}
                }
            }
            GameStatus::Playing => {
                match event {
                    GameEvent::Pause => {
                        *self = GameStatus::Paused;
                    }
                    GameEvent::End => {
                        *self = GameStatus::GameOver;
                    }
                    _ => {}
                }
            }
            GameStatus::Paused => {
                match event {
                    GameEvent::Pause => {
                        *self = GameStatus::Playing;
                    }
                    GameEvent::End => {
                        *self = GameStatus::GameOver;
                    }
                    _ => {}
                }
            }
            GameStatus::GameOver => {
                match event {
                    GameEvent::Start => {
                        *self = GameStatus::Playing;
                    }
                    _ => {}
                }
            }
        }
    }
}