#![allow(dead_code)]
pub mod utils;
pub mod events;
pub mod input;
pub mod screen;
pub mod testing;

extern crate sfml;

use sfml::system::{Clock, Time};
use sfml::system::vector2::ToVec;
use sfml::window::{ContextSettings, VideoMode, event, WindowStyle};
use sfml::graphics::{RenderWindow, RenderTarget, CircleShape, Color};

use self::input::Input;
use self::events::EventProcessor;
use self::screen::Screen;

fn easy_launch<T: Screen>(s: T, w: u32, h: u32, title: &str) {
    launch(s, VideoMode::new_init(w, h, 32),
            title,
            WindowStyle::Close,
            &ContextSettings::default())
}

fn launch<T: Screen>(scr: T, v: VideoMode, title: &str,
        w: WindowStyle, c: &ContextSettings) {
    // Create the window of the application
    let mut window = RenderWindow::new(v,title,w,c).unwrap();
    window.set_key_repeat_enabled(false);
    let mut input = Input::new();
    let mut clock = Clock::new();
    let mut screen: Box<Screen> = Box::new(scr);
    screen.init(&mut window);
    while window.is_open() {
        let event_list: Vec<event::Event> = window.events().collect();
        for event in event_list.iter() {
            input.process_event(event, &mut window);
            screen.process_event(event, &mut window);
            // DEBUGGING 
            // if let &event::Closed = event {
            //     window.close();
            // }
        }
        let res = screen.update(&clock.restart(), &input, &mut window);
        if let Some(next) = res {
            screen = next;
            screen.init(&mut window);
        }
        screen.draw(&input, &mut window);

        window.display();
        input.end_frame();
    }
}