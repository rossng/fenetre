use graphics::types::Rectangle;

pub struct Player {
    pub shape: Rectangle,
}

impl Player {
    /// Create a new `Player` with a random position and direction
    pub fn new() -> Player {
        Player { shape: [0.0, 0.0, 50.0, 50.0] }
    }

    pub fn x(&self) -> f64 {
        self.shape[0]
    }

    pub fn y(&self) -> f64 {
        self.shape[1]
    }
}
