extern crate poglgame;

use poglgame::piston::event::*;
use poglgame::piston::window::WindowSettings;
use poglgame::piston::input::{Key, MouseButton};
use poglgame::opengl_graphics::GlGraphics;
use poglgame::graphics::types::{Color, Vec2d};
use poglgame::graphics::Context;

use poglgame::launch;
use poglgame::screen::*;
use poglgame::game_input::*;

pub fn launch_test_screen() {
    println!("--- Launching test screen ---");
    println!("Square follows mouse");
    println!("[Left click]: cycle colors (creates new Screen instance)");
    println!("[R]: resize and relaunch window (experimental)");
    println!("[Esc]: quit");
    launch(TestScreen::default(), "Test Screen", 800, 600)
}

const COLORS: [Color; 3] = [
    [0.5, 0.0, 0.0, 1.0],
    [0.0, 0.5, 0.0, 1.0],
    [0.0, 0.0, 0.5, 1.0],
];

pub struct TestScreen {
    color_idx: usize,
    position: Vec2d,
    rotation: f64, // radians
}

impl Default for TestScreen {
    fn default() -> Self {
        Self::new(0, 0.)
    }
}

impl TestScreen {
    fn new(idx: usize, rot: f64) -> Self {
        TestScreen {
            color_idx: idx,
            position: [0., 0.],
            rotation: rot,
        }
    }
}

impl Screen for TestScreen {
    fn update(&mut self, args: &UpdateArgs, im: &GameInput)
        -> UpdateResult
    {
        if im.was_key_pressed(&Key::Escape) { 
            UpdateResult::Quit
        } else if im.was_key_pressed(&Key::R) {
            UpdateResult::ReloadWindow(WindowSettings::new(
                "Resized",
                [400,300]
            ))
        } else if im.was_mouse_pressed(&MouseButton::Left) {
            let i = (self.color_idx + 1) % COLORS.len();
            UpdateResult::ChangeScreen(
                    Box::new(TestScreen::new(i, self.rotation)))
        } else {
            self.position = im.mouse_position();
            self.rotation += 2.0 * args.dt;
            UpdateResult::Done
        }
    }
    fn draw(&mut self, args: &RenderArgs, c: Context, gl: &mut GlGraphics) {
        use poglgame::graphics::*;
        clear(COLORS[self.color_idx], gl);
        let square = rectangle::square(0.0, 0.0, 50.0);
        let col = COLORS[(self.color_idx + 1) % COLORS.len()];
        let transform = c.transform.trans(self.position[0], self.position[1])
                .rot_rad(self.rotation)
                .trans(-25.0, -25.0);
        rectangle(col, square, transform, gl);
    }
}