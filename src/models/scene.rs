use models::{Player};
use nphysics2d::world::World;
use na::{Vector1, Vector2, Translation2};

/// A model that contains the other models and renders them
pub struct Scene {
    pub player: Player,
    pub size: (f64, f64),
    pub world: World<f64>
}


fn print_type_of<T>(_: &T) {
    println!("{}", unsafe { ::std::intrinsics::type_name::<T>() });
}

impl Scene {
    /// Returns a new world of the given size
    pub fn new(size: (f64, f64)) -> Scene {
        let mut world = World::new();
        world.set_gravity(Vector2::new(0.0, 0.0));
        print_type_of(&world);
        Scene {
            player: Player::new(&mut world),
            size: size,
            world: world
        }
    }
}
