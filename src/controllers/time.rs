use std::f64;

use super::Actions;
use game_state::GameState;
use na::Vector2;
use na::Matrix;
use na;
use nphysics2d::object::RigidBody;

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
        state.scene.step_scene(dt);

        let mut player_rb = state.scene.player.rb_handle.borrow_mut();

        // Update rocket rotation
        if actions.move_left {
            player_rb.append_lin_force(Vector2::new(-1.0*dt, 0.0));
        };
        if actions.move_right {
            player_rb.append_lin_force(Vector2::new(1.0*dt, 0.0));
        };
        let lin_vel = player_rb.lin_vel().clone();
        //
        // player_rb.append_lin_force(-0.1* lin_vel * na::norm_squared(&lin_vel))
    }
}
