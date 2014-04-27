use rsfml::graphics::{RenderWindow};
use rsfml::window::{VideoMode, ContextSettings, Close, event, keyboard};
use rsfml::window::keyboard::Key;
use rsfml::window::event::Event;
use rsfml::system::{Clock, Time, sleep, Vector2f};

use rsfml::audio::SoundBuffer;
use rsfml::audio::rc::Sound;
use rsfml::audio::Music;
use rsfml::audio::Status;
use rsfml::audio;

use util::get_rc_resource;

pub fn launch(screen : ~Screen, title : &str, w : uint, h : uint) {

	// init window
	let setting : ContextSettings = ContextSettings::default();
	let mut window : RenderWindow = RenderWindow::new(VideoMode::new_init(w, h, 32),
			title, Close, &setting).expect("Cannot create a new Render Window.");
	window.set_vertical_sync_enabled(true);
	window.set_key_repeat_enabled(false);

	// create Game struct
	let mut game = Game::new();

	// grab screen
	let mut s = screen;
	s.init(&mut game, &mut window);

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
		let mut ret = s.update(&mut game, &mut window, t.get_delta());

		// switch to new state if necessary
		match ret {
			Some(ns) => { s = ns; s.init(&mut game, &mut window); },
			None => { }
		}
		window.display();

		// clean up sounds playing
		let mut removed : Vec<uint> = Vec::new();
		let mut len = game.sounds.len();
		let mut i = 0;
		while ( i < len ) {
			let sound = &game.sounds.get(i);
			match sound.get_status() {
				audio::Stopped => {
					removed.push(i);
				}
				_ => {}
			}
			i += 1;
		}

		let mut num_removed = 0;
		for r in removed.iter() {
			game.sounds.remove(r - num_removed);
			num_removed += 1;
		}
	}
}

fn event_default(window : &mut RenderWindow, event : Event) -> bool {
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
	music: Option<Music>
}

impl Game {

	pub fn new() -> Game {
		Game {
			sounds : Vec::new(),
			music : None
		}
	}

	pub fn play_sound_buffer(&mut self, buffer : &SoundBuffer) {
		let buf = buffer.clone().expect("Error cloning buffer reference");
		let rc_buf = get_rc_resource( buf );
		let mut sound = Sound::new_with_buffer( rc_buf ).expect("Error creating sound from buffer");
		sound.play();
		self.sounds.push(sound);
	}

	pub fn loop_music_file(&mut self, song_path : &str) {
		let mut song = Music::new_from_file(song_path).expect("Error loading music file");
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
	fn init(&mut self, game : &mut Game, window : &mut RenderWindow) { /* empty by default */ }
	fn update(&mut self, game : &mut Game, window : &mut RenderWindow, delta : f32) -> Option<~Screen>;
	fn event(&mut self, game : &mut Game, window : &mut RenderWindow, event : Event) -> bool { false }
	fn key_press(&mut self, game : &mut Game, window : &mut RenderWindow, key : Key) -> bool { false }
	fn key_release(&mut self, game : &mut Game, window : &mut RenderWindow, key : Key) -> bool { false }
}

//////////////////
// Timer object //
//////////////////

pub struct Timer {
	clock : Clock,
	min_delta : Time,
	max_delta : Time
}

impl Timer {

	pub fn new() -> Timer {
		Timer {
			clock : Clock::new(),
			min_delta : Time::with_seconds(0.),
			max_delta : Time::with_seconds(1./20.)
		}
	}

	pub fn get_delta(&mut self) -> f32 {
		let time_diff = self.clock.restart();
		if time_diff < self.min_delta {
			sleep( self.min_delta - time_diff );
			self.min_delta
		} else if time_diff > self.max_delta {
			self.max_delta
		} else {
			time_diff
		}.as_seconds()
	}
}