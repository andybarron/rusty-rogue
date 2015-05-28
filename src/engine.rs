use sfml::graphics::{RenderWindow, RenderTarget};
use sfml::window::{VideoMode, ContextSettings, DefaultStyle, event};
use sfml::window::keyboard::Key;
use sfml::window::event::Event;
use sfml::system::{Clock, Time, sleep, Vector2f};
use sfml::graphics::{Color,VertexArray,Lines};

use sfml::audio::SoundBuffer;
use sfml::audio::rc::Sound;
use sfml::audio::Music;
use sfml::audio;

use util::get_rc_resource;

pub fn launch<T: Screen>(screen: T, title: &str, w: u32, h: u32) {

	// init window
	let setting: ContextSettings = ContextSettings::default();
	let mut window: RenderWindow = RenderWindow::new(VideoMode::new_init(w, h, 32),
			title, DefaultStyle, &setting).expect("Cannot create a new Render Window.");
	window.set_vertical_sync_enabled(true);
	window.set_key_repeat_enabled(false);

	// create Game struct
	let mut game = Game::new();

	// grab screen
	let mut s: Box<Screen> = Box::new(screen);
	s.init(&mut game, &mut window);

	// init sound idx remover
	let mut removed: Vec<usize> = Vec::new();

	// init timer
	let mut t = Timer::new();

	// game loop
	while window.is_open() {

		// TODO implement special functions
		// for handling various input events, i.e. key
		// presses, key releases, etc.

		// process events
		loop {
			match window.poll_event() {
				event::NoEvent => break,
				event::KeyPressed{code, ..} => {
					s.key_press(&mut game, &mut window, code);
				}
				event::KeyReleased{code, ..} => {
					s.key_release(&mut game, &mut window, code);
				}
				e => {
					if !s.event(&mut game, &mut window, e) {
						event_default(&mut window, e);
					}
				}
			}
		}

		// update game state
		let ret = s.update(&mut game, &mut window, t.get_delta());

		// switch to new state if necessary
		match ret {
			Some(ns) => { s = ns; s.init(&mut game, &mut window); },
			None => { }
		}
		window.display();

		// clean up sounds playing
		for i in 0..game.sounds.len() {
			match game.sounds[i].get_status() {
				audio::Stopped => {
					removed.push(i);
				}
				_ => {}
			}
		}
		
		for r in removed.iter().rev() {
			game.sounds.swap_remove(*r);
		}
		removed.clear();
	}
}

fn event_default(window: &mut RenderWindow, event: Event) -> bool {
	match event {
		event::Closed => window.close(),
		_ => return false
	}
	true
}

////////////////////////////
// Game controller object //
////////////////////////////

pub struct Game {
	sounds: Vec<Sound>,
	music: Option<Music>,
	va: VertexArray,
}

impl Game {

	pub fn new() -> Game {
		Game {
			sounds: Vec::new(),
			music: None,
			va: VertexArray::new().expect("Couldn't create VertexArray for Game"),
		}
	}

	// pub fn draw_line(&mut self, window: &mut RenderWindow, start: &Vector2f, end: &Vector2f, color: &Color) {
	// 	let mut a = Vertex::default();
	// 	let mut b = Vertex::default();
	// 	a.position = *start;
	// 	b.position = *end;
	// 	a.color = *color;
	// 	b.color = *color;
	// 	self.va.resize(2);
	// 	*self.va.get_vertex(0) = a;
	// 	*self.va.get_vertex(1) = b;
	// 	self.va.set_primitive_type(Lines);
	// 	window.draw(&self.va);
	// }

	pub fn draw_line(&mut self, window: &mut RenderWindow, start: &Vector2f, end: &Vector2f, color: &Color) {
		self.va.resize(2);
		{
			let a = self.va.get_vertex(0);
			a.position = *start;
			a.color = *color;
		}
		{
			let b = self.va.get_vertex(1);
			b.position = *end;
			b.color = *color;
		}
		self.va.set_primitive_type(Lines);
		window.draw(&self.va);
	}

	pub fn play_sound_buffer(&mut self, buffer: &SoundBuffer) {
		let buf = buffer.clone().expect("Error cloning buffer reference");
		let rc_buf = get_rc_resource( buf );
		let mut sound = Sound::new_with_buffer( rc_buf ).expect("Error creating sound from buffer");
		sound.play();
		self.sounds.push(sound);
	}

	pub fn loop_music_file(&mut self, song_full_path: &str) {
		let mut song = Music::new_from_file(song_full_path).expect("Error loading music file");
		song.set_loop(true);
		song.play();
		self.music = Some(song);
	}

	pub fn has_music(&self) -> bool {
		return self.music.is_some();
	}

}

///////////////////////////
// Game screen interface //
///////////////////////////

pub trait Screen {
	fn init(&mut self, game: &mut Game, window: &mut RenderWindow) { /* empty by default */ }
	fn update(&mut self, game: &mut Game, window: &mut RenderWindow, delta: f32) -> Option<Box<Screen>>;
	fn event(&mut self, game: &mut Game, window: &mut RenderWindow, event: Event) -> bool { false }
	fn key_press(&mut self, game: &mut Game, window: &mut RenderWindow, key: Key) -> bool { false }
	fn key_release(&mut self, game: &mut Game, window: &mut RenderWindow, key: Key) -> bool { false }
}

//////////////////
// Timer object //
//////////////////

pub struct Timer {
	clock: Clock,
	pub min_delta: Time,
	pub max_delta: Time
}

fn sleep_sec(amt: f32) {
	return sleep(Time::with_seconds(amt))
}

impl Timer {

	pub fn new() -> Timer {
		Timer {
			clock: Clock::new(),
			min_delta: Time::with_seconds(0.),
			max_delta: Time::with_seconds(1./20.)
		}
	}

	pub fn get_delta(&mut self) -> f32 {
		let min_d = self.min_delta.as_seconds();
		let max_d = self.max_delta.as_seconds();
		let time_diff = self.clock.restart().as_seconds();
		if time_diff < min_d {
			sleep_sec( min_d - time_diff );
			min_d
		} else if time_diff > max_d {
			max_d
		} else {
			time_diff
		}
	}
}