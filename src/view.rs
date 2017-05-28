use opengl_graphics::GlGraphics;
use piston_window::{self, Context, Transformed};
use piston_window::ellipse::Ellipse;
use piston_window::rectangle::Rectangle;
use piston_window::math::Matrix2d;
use piston_window::types::Color;

use game_state::GameState;
use models::{Player, Scene};
use drawing::colour;
use na::Vector2;
use ncollide::world::CollisionWorld2;
use ncollide::shape::{Plane, Ball, Cuboid, ShapeHandle2};
use models::scene::CollisionObjectData;

/// Renders the game to the screen
pub fn render_game(c: Context, g: &mut GlGraphics, state: &GameState) {
    // Clear everything
    piston_window::clear(colour::BLACK, g);

    // Render the world
    render_scene(&state.scene, c, g);
}

/// Renders the world and everything in it
pub fn render_scene(scene: &Scene, c: Context, g: &mut GlGraphics) {
    render_player(&scene.collision_world, &c, g);
    render_plane(&scene.collision_world, &c, g);
    render_contacts(&scene.collision_world, &c, g);
}


pub fn render_contacts(world: &CollisionWorld2<f64, CollisionObjectData>, c: &Context, gl: &mut GlGraphics) {
    let contacts = world.contacts();
    for (_,_, contact) in contacts {
        //if let Some(contact) = maybe_contact {
            let transform1 = c.transform.trans(contact.world1[0], contact.world1[1]);
            let transform2 = c.transform.trans(contact.world2[0], contact.world2[1]);
            piston_window::ellipse(colour::GREEN, [-5.0, -5.0, 5.0 * 2.0, 5.0 * 2.0], transform1, gl);
            piston_window::ellipse(colour::GREEN, [-5.0, -5.0, 5.0 * 2.0, 5.0 * 2.0], transform2, gl);
        //}
    }
}

pub fn render_plane(world: &CollisionWorld2<f64, CollisionObjectData>, c: &Context, gl: &mut GlGraphics) {
    let player_object = world.collision_object(0).unwrap();
    let player_aabb = player_object.shape.aabb(&player_object.position);
    let transform = c.transform.trans(player_object.position.translation.vector[0], player_object.position.translation.vector[1]);

    // Draw the aabb surrounding the floor
    piston_window::rectangle(colour::RED, [-player_aabb.half_extents()[0], -player_aabb.half_extents()[1], player_aabb.half_extents()[0]*2.0, player_aabb.half_extents()[1]*2.0], transform, gl);
}

/// Render the player
pub fn render_player(world: &CollisionWorld2<f64, CollisionObjectData>, c: &Context, gl: &mut GlGraphics) {
    let player_object = world.collision_object(1).unwrap();
    let player_aabb = player_object.shape.aabb(&player_object.position);
    let transform = c.transform.trans(player_object.position.translation.vector[0], player_object.position.translation.vector[1]);

    // Draw the aabb surrounding the player
    piston_window::rectangle(colour::RED, [-player_aabb.half_extents()[0], -player_aabb.half_extents()[1], player_aabb.half_extents()[0]*2.0, player_aabb.half_extents()[1]*2.0], transform, gl);

    if player_object.shape.is_shape::<Ball<f64>>() {
        if let Some(ball) = player_object.shape.as_shape::<Ball<f64>>() {
            piston_window::ellipse(colour::WHITE, [-ball.radius(), -ball.radius(), ball.radius()*2.0, ball.radius()*2.0], transform, gl);
        }
    }

    if player_object.shape.is_shape::<Cuboid<Vector2<f64>>>() {
        if let Some(rect) = player_object.shape.as_shape::<Cuboid<Vector2<f64>>>() {
            piston_window::rectangle(colour::WHITE, [-rect.half_extents()[0], -rect.half_extents()[1], rect.half_extents()[0]*2.0, rect.half_extents()[1]*2.0], transform, gl);
        }
    }
}
