use models::{Player};

/// A model that contains the other models and renders them
pub struct World {
    pub player: Player,
    pub size: (f64, f64)
}

impl World {
    /// Returns a new world of the given size
    pub fn new(size: (f64, f64)) -> World {
        World {
            player: Player::new(),
            size: size
        }
    }
}
