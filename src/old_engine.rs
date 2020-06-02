use sfml::graphics::{Color, PrimitiveType, RenderTarget, RenderWindow, VertexArray};
use sfml::system::{sleep, Clock, Time, Vector2f};
use sfml::window::{ContextSettings, Event, Key, VideoMode};

use sfml::audio::{Sound, SoundStatus};

pub fn launch<T: Screen>(screen: T, title: &str, w: u32, h: u32) {
    // init window
    let setting: ContextSettings = ContextSettings::default();
    let mut window: RenderWindow = RenderWindow::new(
        VideoMode::new(w, h, 32),
        title,
        Default::default(),
        &setting,
    );
    window.set_vertical_sync_enabled(true);
    window.set_key_repeat_enabled(false);

    // create Game struct
    let mut game = Game::new();

    // grab screen
    let mut s: Box<dyn Screen> = Box::new(screen);
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
                None => break,
                Some(Event::KeyPressed { code, .. }) => {
                    s.key_press(&mut game, &mut window, code);
                }
                Some(Event::KeyReleased { code, .. }) => {
                    s.key_release(&mut game, &mut window, code);
                }
                Some(e) => {
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
            Some(ns) => {
                s = ns;
                s.init(&mut game, &mut window);
            }
            None => {}
        }
        window.display();

        // clean up sounds playing
        for i in 0..game.sounds.len() {
            match game.sounds[i].status() {
                SoundStatus::Stopped => {
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
        Event::Closed => window.close(),
        _ => return false,
    }
    true
}

////////////////////////////
// Game controller object //
////////////////////////////

pub struct Game<'a> {
    sounds: Vec<Sound<'a>>,
    // music: Option<Music>,
    va: VertexArray,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        Game {
            sounds: Vec::new(),
            // music: None,
            va: VertexArray::new(PrimitiveType::Lines, 2),
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

    pub fn draw_line(
        &mut self,
        window: &mut RenderWindow,
        start: &Vector2f,
        end: &Vector2f,
        color: &Color,
    ) {
        self.va.resize(2);
        {
            let a = &mut self.va[0];
            a.position = *start;
            a.color = *color;
        }
        {
            let b = &mut self.va[1];
            b.position = *end;
            b.color = *color;
        }
        self.va.set_primitive_type(PrimitiveType::Lines);
        window.draw(&self.va);
    }

    // pub fn play_sound_buffer(&mut self, _buffer: &SoundBuffer) {
    //     let buf = buffer.clone();
    //     let rc_buf = get_rc_resource(buf);
    //     let mut sound = Sound::with_buffer(rc_buf);
    //     sound.play();
    //     self.sounds.push(sound);
    // }

    // pub fn loop_music_file(&mut self, _song_full_path: &str) {
    //     let mut song = Music::new_from_file(song_full_path).expect("Error loading music file");
    //     song.set_loop(true);
    //     song.play();
    //     self.music = Some(song);
    // }

    // pub fn has_music(&self) -> bool {
    //     self.music.is_some()
    // }
}

///////////////////////////
// Game screen interface //
///////////////////////////

pub trait Screen {
    fn init(&mut self, _game: &mut Game, _window: &mut RenderWindow) { /* empty by default */
    }
    fn update(
        &mut self,
        game: &mut Game,
        window: &mut RenderWindow,
        delta: f32,
    ) -> Option<Box<dyn Screen>>;
    fn event(&mut self, _game: &mut Game, _window: &mut RenderWindow, _event: Event) -> bool {
        false
    }
    fn key_press(&mut self, _game: &mut Game, _window: &mut RenderWindow, _key: Key) -> bool {
        false
    }
    fn key_release(&mut self, _game: &mut Game, _window: &mut RenderWindow, _key: Key) -> bool {
        false
    }
}

//////////////////
// Timer object //
//////////////////

pub struct Timer {
    clock: Clock,
    pub min_delta: Time,
    pub max_delta: Time,
}

fn sleep_sec(amt: f32) {
    sleep(Time::seconds(amt))
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            clock: Clock::start(),
            min_delta: Time::seconds(0.),
            max_delta: Time::seconds(1. / 20.),
        }
    }

    pub fn get_delta(&mut self) -> f32 {
        let min_d = self.min_delta.as_seconds();
        let max_d = self.max_delta.as_seconds();
        let time_diff = self.clock.restart().as_seconds();
        if time_diff < min_d {
            sleep_sec(min_d - time_diff);
            min_d
        } else if time_diff > max_d {
            max_d
        } else {
            time_diff
        }
    }
}
