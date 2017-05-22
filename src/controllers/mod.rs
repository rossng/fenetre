//! This module contains the game logic
//!
//! There are three main controllers: collisions, input and time

mod input;
mod time;

pub use self::input::{Actions, InputController};
pub use self::time::TimeController;
