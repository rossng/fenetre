use graphics::types::Rectangle;

pub struct Player {
    pub shape: Rectangle,
    pub xvel: f64,
    pub yvel: f64,
    pub acceleration: f64,
    pub deceleration: f64,
    pub friction: f64,
    pub top_speed: f64
}

impl Player {
    /// Create a new `Player` with a random position and direction
    pub fn new() -> Player {
        Player { shape: [0.0, 0.0, 50.0, 50.0], xvel: 0.0, yvel: 0.0, acceleration: 100.0, deceleration: 150.0, friction: 100.0, top_speed: 200.0 }
    }

    pub fn x(&self) -> f64 {
        self.shape[0]
    }

    pub fn y(&self) -> f64 {
        self.shape[1]
    }
}
