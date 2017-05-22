/// The `Player` is the rocket controlled by the user
#[derive(Default)]
pub struct Player {
    pub x: f64,
    pub y: f64
}

impl Player {
    /// Create a new `Player` with a random position and direction
    pub fn new() -> Player {
        Player { x: 0.0, y: 0.0 }
    }
}
