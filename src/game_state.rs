use models::World;

/// The data structure that contains the state of the game
pub struct GameState {
    /// The world contains everything that needs to be drawn
    pub world: World,
    /// The current score of the player
    pub score: u32
}

impl GameState {
    /// Returns a new `GameState` containing a `World` of the given `Size`
    pub fn new(size: (f64, f64)) -> GameState {
        GameState {
            world: World::new(size),
            score: 0
        }
    }

    /// Reset our game-state
    pub fn reset(&mut self) {
        // Reset player position
        self.world.player.x = 0.0;
        self.world.player.y = 0.0;

        // Reset score
        self.score = 0;
    }
}
