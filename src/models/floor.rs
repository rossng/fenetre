use graphics::types::Rectangle;

pub struct Floor {
    pub shape: Rectangle,
}

impl Floor {
    /// Create a new `Player` with a random position and direction
    pub fn new() -> Floor {
        Floor { shape: [0.0, 0.0, 50.0, 50.0] }
    }

    pub fn x(&self) -> f64 {
        self.shape[0]
    }

    pub fn y(&self) -> f64 {
        self.shape[1]
    }

    pub fn width(&self) -> f64 { self.shape[2] }

    pub fn height(&self) -> f64 { self.shape[3] }
}
