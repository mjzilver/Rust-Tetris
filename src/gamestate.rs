enum GameStatus {
    Startup,
    Playing,
    Paused,
    GameOver,
}

enum GameEvent {
    Start,
    Pause,
    Unpause,
    End,
}

struct GameState {
    status: GameStatus,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            status: GameStatus::Startup,
        }
    }

    pub fn update(&mut self, event: GameEvent) {
        match self.status {
            GameStatus::Startup => {
                match event {
                    GameEvent::Start => {
                        self.status = GameStatus::Playing;
                    }
                    _ => {}
                }
            }
            GameStatus::Playing => {
                match event {
                    GameEvent::Pause => {
                        self.status = GameStatus::Paused;
                    }
                    GameEvent::End => {
                        self.status = GameStatus::GameOver;
                    }
                    _ => {}
                }
            }
            GameStatus::Paused => {
                match event {
                    GameEvent::Unpause => {
                        self.status = GameStatus::Playing;
                    }
                    GameEvent::End => {
                        self.status = GameStatus::GameOver;
                    }
                    _ => {}
                }
            }
            GameStatus::GameOver => {
                match event {
                    GameEvent::Start => {
                        self.status = GameStatus::Playing;
                    }
                    _ => {}
                }
            }
        }
    }
}
