extern crate sfml;
use sfml::system::vector2::ToVec;
use sfml::system::Time;
use sfml::graphics::{RenderWindow, RenderTarget, CircleShape, Color};
use sfml::window::event::Event;
use super::input::Input;
use super::events::EventProcessor;

pub trait Screen: EventProcessor {
    #[allow(unused_variables)]
    fn init(&mut self, win: &mut RenderWindow) {}
    fn update(&mut self, time: &Time, input: &Input, win: &mut RenderWindow)
        -> Option<Box<Screen>>;
    fn draw(&mut self, input: &Input, win: &mut RenderWindow);
}