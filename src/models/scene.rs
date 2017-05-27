use models::{Player};

/// A model that contains the other models and renders them
pub struct Scene {
    pub player: Player,
    pub size: (f64, f64)
}

impl Scene {
    /// Returns a new world of the given size
    pub fn new(size: (f64, f64)) -> Scene {
        Scene {
            player: Player::new(),
            size: size
        }
    }
}
