use std::f64;

use super::Actions;
use game_state::GameState;

/// Timers to handle creation of bullets, enemies and particles
pub struct TimeController {
    current_time: f64
}

impl TimeController {
    pub fn new() -> TimeController {
        TimeController {
            current_time: 0.0
        }
    }

    /// Updates the game
    ///
    /// `dt` is the amount of seconds that have passed since the last update
    pub fn update_seconds(&mut self, dt: f64, actions: &Actions, state: &mut GameState) {
        self.current_time += dt;
        state.scene.step(dt, actions);
    }
}
