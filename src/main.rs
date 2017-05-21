extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct App {
    gl: GlGraphics,      // OpenGL drawing backend.
    rotation: f64,       // Rotation for the square.
    x: f64,
    y: f64,
    speed: f64,
    direction: Option<Direction>
}

enum Direction { Up, Right, Down, Left }

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let x = self.x;
        let y = self.y;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform.trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;

        if let Some(ref d) = self.direction {
            match *d {
                Direction::Up => self.y -= self.speed * args.dt,
                Direction::Down => self.y += self.speed * args.dt,
                Direction::Left => self.x -= self.speed * args.dt,
                Direction::Right => self.x += self.speed * args.dt
            }
        }
    }

    fn move_character(&mut self, direction: Direction) {
        self.direction = Some(direction);
    }

    fn stop_character(&mut self) {
        self.direction = None;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
        "spinning-square",
        [200, 200]
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        x: 100.0,
        y: 100.0,
        speed: 100.0,
        direction: None
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(Button::Keyboard(k)) = e.press_args() {
            match k {
                Key::Right  => app.move_character(Direction::Right),
                Key::Left   => app.move_character(Direction::Left),
                Key::Up     => app.move_character(Direction::Up),
                Key::Down   => app.move_character(Direction::Down),
                _           => ()
            }
        }

        if let Some(Button::Keyboard(k)) = e.release_args() {
            match k {
                Key::Right  => app.stop_character(),
                Key::Left   => app.stop_character(),
                Key::Up     => app.stop_character(),
                Key::Down   => app.stop_character(),
                _           => ()
            }
        }
    }
}