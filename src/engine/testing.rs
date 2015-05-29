extern crate sfml;
use sfml::system::vector2::ToVec;
use sfml::system::Time;
use sfml::graphics::{RenderWindow, RenderTarget, CircleShape, Color};
use sfml::window::event::Event;
use super::input::Input;
use super::events::EventProcessor;
use super::screen::Screen;

pub fn launch_test() {
    super::easy_launch(TestScreen::new(), 400, 300, "Test Screen")
}

pub struct TestScreen<'a> {
    circle: CircleShape<'a>,
}

impl<'a> Screen for TestScreen<'a> {
    fn init(&mut self, win: &mut RenderWindow) {
        println!("Initializing TestScreen; ESC to quit");
    }
    fn update(&mut self, time: &Time, input: &Input, win: &mut RenderWindow)
        -> Option<Box<Screen>> {
            self.circle.set_position(&win.get_mouse_position().to_vector2f());
            None
        }
    fn draw(&mut self, input: &Input, win: &mut RenderWindow) {
        win.clear(&Color::new_rgb(0, 200, 200));
        win.draw(&self.circle);
    }
}

impl<'a> EventProcessor for TestScreen<'a> {
    fn process_event(&mut self, e: &Event, w: &mut RenderWindow) -> bool {
        println!("TestScreen event: {:?}", e);
        match *e {
            Event::Closed => { w.close(); true }
            _ => false
        }
    }
}

impl<'a> TestScreen<'a> {
    fn new() -> Self {
        let mut circle = CircleShape::new()
            .expect("Failed to create CircleShape");
        let r = 100.;
        circle.set_radius(r);
        circle.set_origin2f(r, r);
        circle.set_fill_color(&Color::white());
        TestScreen {
            circle: circle,
        }
    }
}