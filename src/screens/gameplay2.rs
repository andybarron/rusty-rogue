use poglgame::piston::event::*;
use poglgame::piston::window::WindowSettings;
use poglgame::piston::input::{Key, MouseButton};
use poglgame::opengl_graphics::GlGraphics;
use poglgame::graphics::types::{Color, Vec2d};
use poglgame::graphics::Context;
use poglgame::launch;
use poglgame::screen::*;
use poglgame::game_input::*;
use recs::{EntityId, Ecs};
use rand::{Rand, Rng, thread_rng};
use na;

use utils::*;
use rect::*;
use components::*;

pub struct GameplayScreen {
    ecs: Ecs,
}

impl GameplayScreen {
    pub fn new(w: float, h: float) -> Self {
        let mut ecs = Ecs::new();
        for _ in 0..10 {
            let rng = &mut thread_rng();
            let e = ecs.create_entity();
            let pos = Vec2f::new(rng.gen::<float>() * w,
                    rng.gen::<float>() * h);
            let vel = Vec2f::rand(rng) * rng.gen_range(-100., 100.);
            let rsize = &mut || 10. + rng.gen::<float>() * 10.;
            let col = Collision{w: rsize(), h: rsize()};
            ecs.set(e, &Position(pos));
            ecs.set(e, &Velocity(vel));
            ecs.set(e, &col);
        }
        GameplayScreen {
            ecs: ecs,
        }
    }
}

impl Screen for GameplayScreen {
    fn update(&mut self, args: &UpdateArgs, im: &GameInput)
        -> UpdateResult
    {
        if im.was_key_pressed(&Key::Escape) { 
            return UpdateResult::Quit;
        }
        for id in self.ecs.collect_ids() {
            let pos = self.ecs.get::<Position>(id).unwrap().0;
            let vel = self.ecs.get::<Velocity>(id).unwrap().0;
            let npos = pos + (vel * args.dt);
            self.ecs.set(id, &Position(npos));
            let col: Collision = self.ecs.get(id).unwrap();
            let hbox = Rect::new(pos.x, pos.y, col.w, col.h);
            if hbox.min().x < 0. {
                self.ecs.borrow_mut::<Position>(id).map(|p| p.0.x = 0.);
                self.ecs.borrow_mut::<Velocity>(id).map(|v| v.0.x *= -1.);
            }
            if hbox.min().x > 800. {
                self.ecs.borrow_mut::<Position>(id).map(|p| p.0.x = 800.);
                self.ecs.borrow_mut::<Velocity>(id).map(|v| v.0.x *= -1.);
            }
            if hbox.min().y < 0. {
                self.ecs.borrow_mut::<Position>(id).map(|p| p.0.y = 0.);
                self.ecs.borrow_mut::<Velocity>(id).map(|v| v.0.y *= -1.);
            }
            if hbox.min().y > 600. {
                self.ecs.borrow_mut::<Position>(id).map(|p| p.0.y = 600.);
                self.ecs.borrow_mut::<Velocity>(id).map(|v| v.0.y *= -1.);
            }
        }
        UpdateResult::Done
    }
    fn draw(&self, args: &RenderArgs, c: Context, gl: &mut GlGraphics) {
        use poglgame::graphics::*;
        clear([0.0, 0.0, 0.25, 1.0], gl);
        let color = [0.0, 0.5, 0.5, 1.0];
        for id in self.ecs.iter_ids() {
            let pos = self.ecs.get::<Position>(id).unwrap();
            let col = self.ecs.get::<Collision>(id).unwrap();
            let tf = c.transform.clone();
            // let tf = tf
            //         .trans(
            //             pos.x as f64,
            //             pos.y as f64,
            //         );
                    // .rot_deg(45.0)
                    // .trans(-25.0, -25.0);
            // let square = rectangle::square(pos.x, pos.y, 50.0);
            rectangle(color, Rect::from_components(pos, col).rounded(), tf, gl);
        }
    }
}