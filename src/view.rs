use opengl_graphics::GlGraphics;
use piston_window::{self, Context, Transformed};
use piston_window::ellipse::Ellipse;
use piston_window::math::Matrix2d;
use piston_window::types::Color;

use game_state::GameState;
use models::{Player, Scene};
use drawing::colour;

/// Renders the game to the screen
pub fn render_game(c: Context, g: &mut GlGraphics, state: &GameState) {
    // Clear everything
    piston_window::clear(colour::BLACK, g);

    // Render the world
    render_scene(&state.scene, c, g);
}

/// Renders the world and everything in it
pub fn render_scene(world: &Scene, c: Context, g: &mut GlGraphics) {
    render_player(&world.player, &c, g);
}

/*fn ellipse(color: Color, rectangle: [f64; 4], transform: Matrix2d, graphics: &mut GlGraphics)
{
    // There's piston_window::ellipse, but it uses a resolution of 128
    // which is unnecessarily high. Using 16 is much quicker to draw,
    // without looking any different.
    Ellipse {
        color: color,
        border: None,
        resolution: 16,
    }.draw(
        rectangle,
        &Default::default(),
        transform,
        graphics);
}*/

/*fn print_type_of<T>(_: &T) {
    println!("{}", unsafe { ::std::intrinsics::type_name::<T>() });
}*/

/// Render the player
pub fn render_player(player: &Player, c: &Context, gl: &mut GlGraphics) {
    let transform = c.transform
        .trans(player.shape[0], player.shape[1]);

    // Draw an ellipse on the position of the player
    piston_window::ellipse(colour::RED, [0.0, 0.0, 50.0, 50.0], transform, gl)
}
