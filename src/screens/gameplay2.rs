use poglgame::event::*;
use poglgame::window::WindowSettings;
use poglgame::input::{Key, MouseButton};
use poglgame::GlGraphics;
use poglgame::types::{Color, Vec2d};
use poglgame::Context;
use poglgame::launch;
use poglgame::screen::*;
use poglgame::game_input::*;

use recs::{EntityId, Ecs};
use rand::{Rand, Rng, thread_rng};
use na;

use utils::*;
use rect::*;
use components::*;
use physics::*;

pub struct GameplayScreen {
    ecs: Ecs,
    hits: Vec<Rectf>,
    w: float,
    h: float,
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
            let rsize = &mut || 50. + rng.gen::<float>() * 50.;
            let col = Collision{w: rsize(), h: rsize()};
            ecs.set(e, &Position(pos));
            ecs.set(e, &Velocity(vel));
            ecs.set(e, &col);
        }
        let hits = Vec::with_capacity(ecs.iter_ids().count());
        GameplayScreen {
            ecs: ecs,
            hits: hits,
            w: 9999.0,
            h: 9999.0,
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
        self.hits.clear();
        let colliders =
            self.ecs.collect_with_3::<Position, Velocity, Collision>();
        for &(id, pos, vel, col) in colliders.iter() {
            let npos = pos.0 + (vel.0 * args.dt);
            self.ecs.set(id, &Position(npos));
            let hbox = Rectf::new(npos.x, npos.y, col.w, col.h);
            if hbox.min().x < 0. {
                self.ecs.borrow_mut::<Position>(id).map(|p| p.0.x = 0.);
                self.ecs.borrow_mut::<Velocity>(id).map(|v| v.0.x *= -1.);
            }
            let x_max = self.w - col.w;
            if hbox.min().x > x_max {
                self.ecs.borrow_mut::<Position>(id).map(|p| p.0.x = x_max);
                self.ecs.borrow_mut::<Velocity>(id).map(|v| v.0.x *= -1.);
            }
            if hbox.min().y < 0. {
                self.ecs.borrow_mut::<Position>(id).map(|p| p.0.y = 0.);
                self.ecs.borrow_mut::<Velocity>(id).map(|v| v.0.y *= -1.0);
            }
            let y_max = self.h - col.h;
            if hbox.min().y > y_max {
                self.ecs.borrow_mut::<Position>(id).map(|p| p.0.y = y_max);
                self.ecs.borrow_mut::<Velocity>(id).map(|v| v.0.y *= -1.0);
            }
        }
        let mut cols: Vec<_> = self.ecs
                .collect_with_2::<Position, Collision>()
                .iter()
                .map(|&(id,_,col)| (id, col)).collect();
        cols.sort_by(|&(id1, _), &(id2, _)| id1.cmp(&id2));
        for &(id1, col1) in cols.iter() {
            for &(id2, col2) in cols.iter() {
                if id1 == id2 { break; }
                let a = Rectf::from_components(&self.ecs.get(id1).unwrap(),
                        &col1);
                let b = Rectf::from_components(&self.ecs.get(id2).unwrap(),
                        &col2);
                Rectf::get_overlap(&a, &b).map(
                        |r| self.hits.push(r));
                // collide_rect_weighted(&a, &b, 0.5).map(|c| {
                //     self.ecs.borrow_mut::<Position>(id1)
                //         .map(|p| p.0 = p.0 + c.a);
                //     self.ecs.borrow_mut::<Position>(id2)
                //         .map(|p| p.0 = p.0 + c.b);
                //     self.ecs.borrow_mut::<Velocity>(id1)
                // });
            }
        }
        UpdateResult::Done
    }
    fn draw(&mut self, args: &RenderArgs, c: Context, gl: &mut GlGraphics) {
        use poglgame::*;
        self.w = args.draw_width as float;
        self.h = args.draw_height as float;
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
            rectangle(color, Rectf::from_components(&pos, &col).rounded(), tf, gl);
        }
        let highlight = [0.5, 0.0, 0.5, 1.0];
        for r in self.hits.iter() {
            rectangle(highlight, r.rounded(), c.transform, gl);
        }
    }
}