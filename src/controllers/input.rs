use piston_window::{Key};

#[derive(Default)]
pub struct InputController {
    actions: Actions
}

/// Active actions (toggled by user input)
#[derive(Default)]
pub struct Actions {
    pub move_left: bool,
    pub move_right: bool
}

impl InputController {
    /// Create a new `InputController`
    pub fn new() -> InputController {
        InputController::default()
    }

    /// Returns a shared reference to the underlying actions
    pub fn actions(&self) -> &Actions {
        &self.actions
    }

    /// Processes a key press
    pub fn key_press(&mut self, key: Key) {
        self.handle_key(key, true);
    }

    /// Processes a key release
    pub fn key_release(&mut self, key: Key) {
        self.handle_key(key, false);
    }

    /// Handles a key press or release
    fn handle_key(&mut self, key: Key, pressed: bool) {
        println!("Key {:?} pressed: {}", key, pressed);
        match key {
            Key::Left => self.actions.move_left = pressed,
            Key::Right => self.actions.move_right = pressed,
            _ => ()
        }
    }
}