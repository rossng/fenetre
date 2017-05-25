use models::Scene;

/// The data structure that contains the state of the game
pub struct GameState {
    /// The world contains everything that needs to be drawn
    pub scene: Scene,
    /// The current score of the player
    pub score: u32,
}

impl GameState {
    /// Returns a new `GameState` containing a `World` of the given `Size`
    pub fn new(size: (f64, f64)) -> GameState {
        GameState {
            scene: Scene::new(size),
            score: 0
        }
    }

    /// Reset our game-state
    pub fn reset(&mut self) {
        // Reset player position
        self.scene.player.x = 0.0;
        self.scene.player.y = 0.0;

        // Reset score
        self.score = 0;
    }
}
