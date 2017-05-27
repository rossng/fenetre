use models::{Player};
use nphysics2d::world::World;
use nphysics2d::object::RigidBody;
use na::{Vector1, Vector2, Translation2};
use ncollide::shape::Plane;

/// A model that contains the other models and renders them
pub struct Scene {
    pub player: Player,
    pub size: (f64, f64),
    pub world: World<f64>
}

impl Scene {
    /// Returns a new world of the given size
    pub fn new(size: (f64, f64)) -> Scene {
        let mut world = World::new();
        world.set_gravity(Vector2::new(0.0, 9.81));
        let mut ground = RigidBody::new_static(Plane::new(Vector2::new(0.0, -1.0)), 0.3, 0.6);
        ground.append_translation(&Translation2::new(0.0, 300.0));
        world.add_rigid_body(ground);
        Scene {
            player: Player::new(&mut world),
            size: size,
            world: world
        }
    }

    pub fn step_scene(&mut self, dt: f64) {
        self.world.step(dt);
    }
}
