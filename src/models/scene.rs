use models::{Player, Floor};
use controllers::Actions;
use ncollide::world::CollisionWorld2;

/// A model that contains the other models and renders them
pub struct Scene {
    pub floor: Floor,
    pub player: Player,
    pub collision_world: CollisionWorld2<f64, ()>,
    pub size: (f64, f64)
}

impl Scene {
    /// Returns a new world of the given size
    pub fn new(size: (f64, f64)) -> Scene {
        Scene {
            player: Player::new(),
            size: size,
            floor: Floor::new(),
            collision_world: CollisionWorld2::new(0.1, false)
        }
    }

    pub fn step(&mut self, dt: f64, actions: &Actions) {
        if actions.move_left == actions.move_right {
            let sign = if self.player.xvel > 0.0 { 1.0 } else { -1.0 };
            if self.player.xvel.abs() < self.player.friction { //sonic said this is physics
                self.player.xvel = 0.0;
            } else {
                self.player.xvel -= self.player.friction * sign * dt;
            }
        } else {
            if actions.move_left {
                if self.player.xvel > 0.0 {
                    self.player.xvel -= self.player.deceleration * dt;
                } else if self.player.xvel > -self.player.top_speed {
                    self.player.xvel -= self.player.acceleration * dt;
                }
            };
            if actions.move_right {
                if self.player.xvel < 0.0 {
                    self.player.xvel += self.player.deceleration * dt;
                } else if self.player.xvel < self.player.top_speed {
                    self.player.xvel += self.player.acceleration * dt;
                }
            };
        }

        self.player.shape[0] += self.player.xvel * dt;
        self.player.shape[1] += self.player.yvel * dt;
    }

}
