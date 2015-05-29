extern crate sfml;
use sfml::window::event::Event;
use sfml::graphics::RenderWindow;

pub trait EventProcessor {
    #[allow(unused_variables)]
    fn process_event(&mut self, e: &Event, w: &mut RenderWindow) -> bool { false }
    fn end_frame(&mut self) { /* do nothing */ }
}
