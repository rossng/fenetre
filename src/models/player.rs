use std::rc::Rc;
use std::cell::RefCell;
use nphysics2d::object::RigidBody;
use nphysics2d::world::World;
use ncollide::shape::{Plane, Cuboid};
use na::{Vector1, Vector2, Translation2};

/// The `Player` is the rocket controlled by the user
pub struct Player {
    pub rb_handle: Rc<RefCell<RigidBody<f64>>>
}

impl Player {
    /// Create a new `Player` with a random position and direction
    pub fn new(world: &mut World<f64>) -> Player {
        let player = Cuboid::new(Vector2::new(0.5, 0.5));
        let mut rb = RigidBody::new_dynamic(player, 0.1, 0.3, 2.0);
        let rb_handle = world.add_rigid_body(rb);
        Player { rb_handle: rb_handle }
    }
}
