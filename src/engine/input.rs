// TODO smooth scroll wheel
// store last X wheel values, choose big/small/median

use std::collections::HashSet;
extern crate sfml;
use sfml::graphics::RenderWindow;
use sfml::window::event::Event;
use sfml::window::keyboard::Key;
use sfml::window::mouse::MouseButton;
use super::utils::DebugString;
use super::events::EventProcessor;

// use std::hash::{Hash, Hasher};
// impl Hash for Key {
//     fn hash<H>(&self, state: &mut H) where H: Hasher {
//         format!("{:?}", self).hash(state)
//     }
// }

// TODO change from HashSet<String> into HashSet<Key/MouseButton> when they
// implement Hash...

pub struct Input {
    keys_down: HashSet<String>,
    keys_pressed: HashSet<String>,
    keys_released: HashSet<String>,
    mouse_down: HashSet<String>,
    mouse_pressed: HashSet<String>,
    mouse_released: HashSet<String>,
    mouse_wheel_delta: i32,
}

impl Input {
    pub fn new() -> Self {
        Input {
            keys_down: HashSet::new(),
            keys_pressed: HashSet::new(),
            keys_released: HashSet::new(),
            mouse_down: HashSet::new(),
            mouse_pressed: HashSet::new(),
            mouse_released: HashSet::new(),
            mouse_wheel_delta: 0,
        }
    }
    pub fn is_key_down(&self, key: Key) -> bool {
        self.keys_down.contains(&key.ds())
    }
    pub fn was_key_pressed(&self, key: &Key) -> bool {
        self.keys_pressed.contains(&key.ds())
    }
    pub fn was_key_released(&self, key: &Key) -> bool {
        self.keys_released.contains(&key.ds())
    }
    pub fn is_mouse_down(&self, mouse: &MouseButton) -> bool {
        self.mouse_down.contains(&mouse.ds())
    }
    pub fn was_mouse_pressed(&self, mouse: &MouseButton) -> bool {
        self.mouse_pressed.contains(&mouse.ds())
    }
    pub fn was_mouse_released(&self, mouse: &MouseButton) -> bool {
        self.mouse_released.contains(&mouse.ds())
    }
    pub fn get_wheel_velocity(&self) -> i32 {
        self.mouse_wheel_delta
    }
    // use std::collections::hash_set::Iter;
    // fn keys_down_iter(&self) -> Iter<Key> {
    //     self.keys_down.iter()
    // }
}

impl EventProcessor for Input {
    fn end_frame(&mut self) {
        self.keys_pressed.clear();
        self.keys_released.clear();
        self.mouse_pressed.clear();
        self.mouse_released.clear();
        self.mouse_wheel_delta = 0;
    }
    fn process_event(&mut self, e: &Event, _: &mut RenderWindow) -> bool {
        match e {
            &Event::KeyPressed{code, ..} => {
                self.keys_down.insert(code.ds());
                self.keys_pressed.insert(code.ds());
            }
            &Event::KeyReleased{code, ..} => {
                self.keys_down.remove(&code.ds());
                self.keys_released.insert(code.ds());
            }
            &Event::MouseButtonPressed{button, ..} => {
                self.mouse_down.insert(button.ds());
                self.mouse_pressed.insert(button.ds());
            }
            &Event::MouseButtonReleased{button, ..} => {
                self.mouse_down.remove(&button.ds());
                self.mouse_released.insert(button.ds());
            }
            &Event::MouseWheelMoved{delta, ..} => {
                if delta.abs() > self.mouse_wheel_delta.abs() {
                    self.mouse_wheel_delta = delta;
                }
            }
            _ => return false
        }
        true
    }
}
