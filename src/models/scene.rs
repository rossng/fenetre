use std::cell::Cell;
use models::{Player, Floor};
use controllers::Actions;
use ncollide::world::CollisionWorld2;
use na::{Vector2, Point2, Isometry2, Point3, Translation2};
use na;
use ncollide::world::{CollisionWorld, CollisionGroups, GeometricQueryType, CollisionObject2};
use ncollide::narrow_phase::{ProximityHandler, ContactHandler, ContactAlgorithm2};
use ncollide::shape::{Plane, Ball, Cuboid, ShapeHandle2};
use ncollide::query::{Proximity, Contact};
use na::core::dimension::{U1, U2};
use na::geometry::{PointBase, Translation};
use na::core::MatrixArray;
use alga::linear::AffineTransformation;

/// A model that contains the other models and renders them
pub struct Scene {
    pub collision_world: CollisionWorld2<f64, CollisionObjectData>,
    pub size: (f64, f64)
}

#[derive(Clone)]
pub struct CollisionObjectData {
    pub name: &'static str,
    pub velocity: Option<Cell<Vector2<f64>>>,
    pub depenetration: Option<Cell<Vector2<f64>>>
}

pub enum ObjectType {
    Static,
    Dynamic
}

impl CollisionObjectData {
    pub fn new(name: &'static str, velocity: Option<Vector2<f64>>, object_type: ObjectType) -> CollisionObjectData {
        CollisionObjectData {
            name:     name,
            velocity: match velocity { Some(vel) => Some(Cell::new(vel)), None => None },
            depenetration: match object_type { ObjectType::Dynamic => Some(Cell::new(Vector2::new(0.0, 0.0))), ObjectType::Static => None }
        }
    }
}

struct ObjectStopper;

impl ContactHandler<Point2<f64>, Isometry2<f64>, CollisionObjectData> for ObjectStopper {
    fn handle_contact_started(&mut self,
                              co1: &CollisionObject2<f64, CollisionObjectData>,
                              co2: &CollisionObject2<f64, CollisionObjectData>,
                              alg: &ContactAlgorithm2<f64>) {
        // NOTE: real-life applications would avoid this systematic allocation.
        let mut contacts : Vec<Contact<PointBase<f64, U2, MatrixArray<f64, U2, U1>>>> = Vec::new();
        alg.contacts(&mut contacts);

        // The player is the one with a non-None velocity.
        if let Some(ref vel) = co1.data.velocity {
            println!("Stopping player {} after hitting {}", co1.data.name, co2.data.name);
            if co1.shape.is_shape::<Cuboid<Vector2<f64>>>() {
                if let Some(rect) = co1.shape.as_shape::<Cuboid<Vector2<f64>>>() {
                    for contact in &contacts {
                        let c : &Contact<PointBase<f64, U2, MatrixArray<f64, U2, U1>>> = contact;
                        if c.world1[1] > co1.position.translation.vector[1] + rect.half_extents()[1] - 10.0 {
                            vel.set(Vector2::new(vel.get()[0], 0.0));
                            if let Some(ref depenetration) = co1.data.depenetration {
                                println!("Collision normal: {}, depth {}", c.normal, c.depth);
                                depenetration.set(c.normal * c.depth * -1.0);
                            }
                        }
                    }
                }
            }
        }
        if let Some(ref vel) = co2.data.velocity {
            println!("Stopping player {} after hitting {}", co2.data.name, co1.data.name);
            if co2.shape.is_shape::<Cuboid<Vector2<f64>>>() {
                if let Some(rect) = co2.shape.as_shape::<Cuboid<Vector2<f64>>>() {
                    for contact in &contacts {
                        let c : &Contact<PointBase<f64, U2, MatrixArray<f64, U2, U1>>> = contact;
                        if c.world1[1] > co2.position.translation.vector[1] + rect.half_extents()[1] - 10.0 {
                            vel.set(Vector2::new(vel.get()[0], 0.0));
                            if let Some(ref depenetration) = co2.data.depenetration {
                                println!("Collision normal: {}, depth {}", c.normal, c.depth);
                                depenetration.set(c.normal * c.depth * -1.0);
                            }
                        }
                    }
                }
            }
        }
    }

    fn handle_contact_stopped(&mut self,
                              _: &CollisionObject2<f64, CollisionObjectData>,
                              _: &CollisionObject2<f64, CollisionObjectData>) {
        // We don't care.
    }
}

impl Scene {
    /// Returns a new world of the given size
    pub fn new(size: (f64, f64)) -> Scene {
        let mut collision_world = CollisionWorld2::new(0.0, true);

        // The player is in group 1 and interacts with everything.
        let mut player_groups = CollisionGroups::new();
        player_groups.set_membership(&[1]);

        // The world objects are in group 2 and interact only with the player
        let mut world_groups = CollisionGroups::new();
        world_groups.set_membership(&[2]);
        world_groups.set_whitelist(&[1]);

        let floor = ShapeHandle2::new(Cuboid::new(Vector2::new(110.0, 20.0)));
        let floor_pos = Isometry2::new(Vector2::new(200.0, 200.0), na::zero());
        let floor_data = CollisionObjectData::new("floor", None, ObjectType::Static);

        let player = ShapeHandle2::new(Cuboid::new(Vector2::new(20.0, 50.0)));
        let player_pos = Isometry2::new(Vector2::new(100.0, 100.0), na::zero());
        let player_data = CollisionObjectData::new("player", Some(Vector2::new(0.0, 10.0)), ObjectType::Dynamic);

        collision_world.deferred_add(0, floor_pos, floor, world_groups, GeometricQueryType::Contacts(0.0), floor_data);
        collision_world.deferred_add(1, player_pos, player, player_groups, GeometricQueryType::Contacts(0.0), player_data);

        collision_world.register_contact_handler("ObjectStopper", ObjectStopper);

        collision_world.update();

        Scene {
            size: size,
            collision_world: collision_world
        }
    }

    pub fn step(&mut self, dt: f64, actions: &Actions) {
        let player_pos;
        let gravity = 10.0;

        {
            let player_object = self.collision_world.collision_object(1).unwrap();
            if let Some(ref vel) = player_object.data.velocity {
                vel.set(Vector2::new(vel.get()[0], vel.get()[1] + gravity * dt))
            }
            let player_velocity = player_object.data.velocity.as_ref().unwrap();

            let player_displacement = Translation2::from_vector(dt * player_velocity.get());
            player_pos = player_displacement * player_object.position;
        }

        self.collision_world.deferred_set_position(1, player_pos);
        self.collision_world.update();

        let player_pos;
        {
            let player_object = self.collision_world.collision_object(1).unwrap();

            let player_depenetration = player_object.data.depenetration.as_ref().unwrap();

            let player_displacement = Translation2::from_vector(player_depenetration.get());
            player_pos = player_displacement * player_object.position;
        }

        self.collision_world.deferred_set_position(1, player_pos);
        self.collision_world.update();

        /*if actions.move_left == actions.move_right {
            let sign = if self.player.xvel > 0.0 { 1.0 } else { -1.0 };
            if self.player.xvel.abs() < self.player.friction { //sonic said this is physics
                self.player.xvel = 0.0;
            } else {
                self.player.xvel -= self.player.friction * sign * dt;
            }
        } else {
            if actions.move_left {
                if self.player.xvel > 0.0 {
                    self.player.xvel -= self.player.deceleration * dt;
                } else if self.player.xvel > -self.player.top_speed {
                    self.player.xvel -= self.player.acceleration * dt;
                }
            };
            if actions.move_right {
                if self.player.xvel < 0.0 {
                    self.player.xvel += self.player.deceleration * dt;
                } else if self.player.xvel < self.player.top_speed {
                    self.player.xvel += self.player.acceleration * dt;
                }
            };
        }*/

        //self.player.shape[0] += self.player.xvel * dt;
        //self.player.shape[1] += self.player.yvel * dt;
    }

}
