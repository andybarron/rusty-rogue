use std::collections::HashMap;
use std::ops::Range;

use std::sync::{Arc, RwLock};

use sfml::graphics::CircleShape;
use sfml::graphics::Color;
use sfml::graphics::FloatRect;
use sfml::graphics::IntRect;
use sfml::graphics::Shape;
use sfml::graphics::Sprite;
use sfml::graphics::Texture;
use sfml::graphics::Transformable;
use sfml::graphics::View;
use sfml::graphics::{RenderTarget, RenderWindow};
use sfml::system::SfBox;
use sfml::system::Vector2f;
use sfml::window::Key;

use std::ops::Deref;

use crate::generator::{Corridor, Door, Dungeon, Floor, Monster, StairsDown, StairsUp, Tile, Wall};
use crate::old_engine::{Game, Screen};
use crate::util;
use crate::util::get_gfx_path;
use crate::util::get_sprite_coords;

use crate::collision::CollisionResolver;

use crate::animation::Animation;
use crate::entities::{Creature, Facing};

use crate::graph::Graph;
use crate::solver::Solver;

static SOLVER_THREAD_COUNT: usize = 4;

pub struct GameplayScreen<'a> {
    tile_size: usize,
    tile_sizef: f32,
    dungeon: Dungeon,
    graph: Arc<RwLock<Graph>>,
    tiles: Vec<TileData<'a>>,
    view: SfBox<View>,
    zoom_index: isize,
    zoom_levels: Vec<f32>,
    creatures: Vec<Creature<'a>>,
    debug_graph: bool,
    debug_los: bool,
    debug_node_circle: CircleShape<'a>,
    solvers: Vec<Solver>,
    path_count: usize,
    vis_x: Range<isize>,
    vis_y: Range<isize>,
    player_idx: Option<usize>,
    collide: CollisionResolver,
}

impl<'a> GameplayScreen<'a> {
    pub fn load_texture() -> SfBox<Texture> {
        let tex_path = get_gfx_path("all_tiles.png");
        Texture::from_file(&tex_path).expect("Failed to load all_tiles.png")
    }
    pub fn new(dungeon: &Dungeon, texture: &'a Texture) -> GameplayScreen<'a> {
        let mut dungeon = dungeon.clone();
        dungeon.shrink();
        // load tile texture file
        // let tex_path = get_gfx_path("all_tiles.png");
        // let tex = Texture::from_file(&tex_path).expect("Failed to load all_tiles.png");

        // get refcounted version for rc::Sprite
        // let rc_tex = get_rc_resource(tex);

        let tsz_init = 16;
        let debug_node_radius = tsz_init as f32 / 4.0;

        // init screen
        let mut ret = GameplayScreen {
            tile_size: tsz_init,
            tile_sizef: tsz_init as f32,
            dungeon: dungeon.clone(),
            graph: Arc::new(RwLock::new(Graph::new())),
            zoom_index: 1,
            zoom_levels: vec![1., 2., 3., 4.],
            tiles: Vec::new(),
            view: View::new(Default::default(), Default::default()), // TODO: default??
            creatures: Vec::new(),
            debug_graph: false,
            debug_los: false,
            debug_node_circle: CircleShape::new(debug_node_radius, 8),
            solvers: Vec::new(),
            path_count: 0,
            vis_x: 0..1,
            vis_y: 0..1,
            player_idx: None,
            collide: CollisionResolver::new(),
        };
        ret.debug_node_circle
            .set_origin(Vector2f::new(debug_node_radius, debug_node_radius));
        ret.debug_node_circle.set_fill_color(Color {
            r: 0u8,
            g: 0u8,
            b: 255u8,
            a: 150u8,
        });

        for _ in 0..SOLVER_THREAD_COUNT {
            ret.solvers.push(Solver::new());
        }

        // closure to get tile coordinates from tile x/y index
        // i.e. top left tile in texture atlas is (0,0)
        let t_sz = ret.tile_size;
        let grab_tile_rect = |x: usize, y: usize| -> IntRect {
            let (tx, ty) = get_sprite_coords(x, y, t_sz, t_sz);
            IntRect {
                left: tx as i32,
                top: ty as i32,
                width: t_sz as i32,
                height: t_sz as i32,
            }
        };

        // get coordinates of each tile type
        let coords_floor = grab_tile_rect(8, 6);
        let _coords_door = grab_tile_rect(3, 0);
        let coords_hall = grab_tile_rect(7, 4);
        let coords_up = grab_tile_rect(9, 7);
        let coords_dn = grab_tile_rect(10, 7);

        // get sprite directly from coords
        let get_spr = |x: usize, y: usize| -> Sprite<'a> {
            let coords = grab_tile_rect(x, y);
            let mut spr = Sprite::with_texture(texture);
            spr.set_texture_rect(&coords);
            spr
        };

        let b_n = get_spr(9, 3);
        let b_s = get_spr(9, 5);
        let b_e = get_spr(10, 4);
        let b_w = get_spr(8, 4);

        // for each tile in the dungeon
        for tile in dungeon.get_tile_vector().iter() {
            // convert x/y index to px coordinates
            let x = tile.x * t_sz as isize;
            let y = tile.y * t_sz as isize;

            // load tile coordinates based on tile type
            let tile_coords = match tile.t {
                Floor => vec![coords_floor],
                Door => vec![coords_hall], // TODO draw door ON TOP of walls
                Corridor => vec![coords_hall],
                StairsUp => vec![coords_floor, coords_up],
                StairsDown => vec![coords_floor, coords_dn],
                _ => vec![],
            };

            // load sprite from texture and add to tile list
            let half = t_sz as f32 / 2.0;
            let bounds = FloatRect::new(x as f32 - half, y as f32 - half, t_sz as f32, t_sz as f32);
            let mut tile_data = TileData::new(&bounds, tile);
            for coords in tile_coords.iter() {
                let mut spr = Sprite::with_texture(texture);
                spr.set_texture_rect(coords);
                spr.set_origin(Vector2f::new(t_sz as f32 / 2.0, t_sz as f32 / 2.0));
                spr.set_position(Vector2f::new(x as f32, y as f32));
                tile_data.sprites.push(spr);
            }

            let wall_off = 1.0;
            ret.add_wall_check(&mut tile_data, (0, -1), &b_n, wall_off);
            ret.add_wall_check(&mut tile_data, (0, 1), &b_s, wall_off);
            ret.add_wall_check(&mut tile_data, (1, 0), &b_e, wall_off);
            ret.add_wall_check(&mut tile_data, (-1, 0), &b_w, wall_off);

            ret.tiles.push(tile_data);
        }

        println!("Initializing graph...");
        // initialize graph
        for y in 0..dungeon.height {
            for x in 0..dungeon.width {
                ret.graph
                    .write()
                    .ok()
                    .expect("mt write error")
                    .add_node_at(x, y);
            }
        }

        println!("Starting graph node loop...");
        // loop through the graph and
        // connect accessible nodes
        for y in 0..dungeon.height {
            for x in 0..dungeon.width {
                let idx_opt = ret.tile_idx_from_coords((x, y));
                let idx = idx_opt.expect("Shouldn't be negative");
                match ret.tiles[idx].is_passable() {
                    false => {}
                    true => {
                        // only check R, DR, D, DL
                        // yay undirected graphs!

                        // check R and D
                        ret.connect_direct(x, y, (1, 0));
                        ret.connect_direct(x, y, (0, 1));

                        // diagonal
                        ret.connect_diag(x, y, (1, 1), (x + 1, y), (x, y + 1));
                        ret.connect_diag(x, y, (-1, 1), (x - 1, y), (x, y + 1));
                    }
                }
            }
        }
        println!("Done with graph!");

        let get_walk_cycle_frames = |x: usize, y: usize| -> Vec<IntRect> {
            vec![
                grab_tile_rect(x, y),
                grab_tile_rect(x - 1, y),
                grab_tile_rect(x, y),
                grab_tile_rect(x + 1, y),
            ]
        };

        let get_walk_cycle = |x: usize, y: usize, length: f32| -> Animation {
            let spr_m = get_spr(x, y);
            let cycle_s = get_walk_cycle_frames(x, y);
            let cycle_w = get_walk_cycle_frames(x, y + 1);
            let cycle_e = get_walk_cycle_frames(x, y + 2);
            let cycle_n = get_walk_cycle_frames(x, y + 3);
            let mut anim = Animation::new(&spr_m, &cycle_n, length);
            anim.frame_sets.push(cycle_e);
            anim.frame_sets.push(cycle_s);
            anim.frame_sets.push(cycle_w);
            anim
        };

        // load up player sprite
        let coords_hero = grab_tile_rect(4, 8);
        let mut sprite_hero = Sprite::with_texture(texture);
        sprite_hero.set_texture_rect(&coords_hero);

        // create player creature
        let mut hero = Creature::new(&get_walk_cycle(4, 8, 0.5), 10);
        let (start_x, start_y) = dungeon.start_coords;
        hero.set_position2f(
            (start_x * t_sz as isize) as f32,
            (start_y * t_sz as isize) as f32,
        );
        hero.player = true;

        // a bunch of monsters
        let coords_slime = grab_tile_rect(10, 8);
        let mut sprite_slime = Sprite::with_texture(texture);
        sprite_slime.set_texture_rect(&coords_slime);

        let monster_cycles = [
            get_walk_cycle(10, 8, 1.0),
            get_walk_cycle(10, 12, 1.0),
            get_walk_cycle(7, 12, 1.0),
            get_walk_cycle(4, 12, 1.0),
            // get_walk_cycle(1,12,1.0), // slime
        ];

        // find and create monsters
        for tile in dungeon.tiles.iter() {
            match tile.e {
                Some(Monster(num)) => {
                    let idx = num % monster_cycles.len();
                    let mut slime = Creature::new(&monster_cycles[idx], 5);
                    slime.set_position2f(
                        (tile.x * t_sz as isize) as f32,
                        (tile.y * t_sz as isize) as f32,
                    );
                    slime.anim.timer = (((num % 100) as f32) / 100.0) * slime.anim.length;
                    slime.anim.update(0.0);
                    ret.creatures.push(slime);
                }
                _ => {}
            }
        }

        ret.creatures.push(hero);
        ret
    }

    fn logic(&mut self, _game: &mut Game, window: &mut RenderWindow, delta: f32) {
        // figure out zoom level
        self.zoom_index = util::clamp(self.zoom_index, 0, self.zoom_levels.len() as isize - 1);
        let mag = self.zoom_levels[self.zoom_index as usize];

        // if no player, enable panning? sure.
        let pan_spd = 16. * 16. * delta / mag;

        let go_l = Key::Left.is_pressed();
        let go_r = Key::Right.is_pressed();
        let go_u = Key::Up.is_pressed();
        let go_d = Key::Down.is_pressed();

        let mut pan = Vector2f::new(0., 0.);

        if go_l {
            pan.x -= pan_spd
        };
        if go_r {
            pan.x += pan_spd
        };
        if go_u {
            pan.y -= pan_spd
        };
        if go_d {
            pan.y += pan_spd
        };

        self.view.move_(pan);

        // depth sort
        self.sprite_depth_sort();

        // find player if necessary
        let player = match self.player_idx {
            Some(idx) => Some(idx),
            None => {
                let mut fnd = None;
                for i in 0..self.creatures.len() {
                    if self.creatures[i].player {
                        fnd = Some(i);
                        break;
                    }
                }
                fnd
            }
        };

        // update player!
        match player {
            None => {}
            Some(hero) => {
                let dist = 64. * delta;

                let angle = match (go_u, go_d, go_l, go_r) {
                    (false, false, false, true) => Some(0.),
                    (false, true, false, true) => Some(45.),
                    (false, true, false, false) => Some(90.),
                    (false, true, true, false) => Some(135.),
                    (false, false, true, false) => Some(180.),
                    (true, false, true, false) => Some(225.),
                    (true, false, false, false) => Some(270.),
                    (true, false, false, true) => Some(315.),
                    _ => None,
                };

                // move player
                match angle {
                    None => {}
                    Some(deg) => {
                        let guy = &mut self.creatures[hero];
                        guy.move_polar_deg(dist, deg);
                        guy.set_facing(Facing::from_deg(deg));
                        guy.update_anim(delta);
                    }
                }

                // get solutions
                let mut path_map: HashMap<usize, Vec<(isize, isize)>> = HashMap::new();
                for solver in self.solvers.iter_mut() {
                    loop {
                        match solver.poll() {
                            None => break,
                            Some(soln) => {
                                let id = soln.id;
                                let path = match soln.path {
                                    None => Vec::new(),
                                    Some(path) => path,
                                };
                                path_map.insert(id, path);
                            }
                        }
                    }
                }

                // chase player!
                let hero_pos = self.creatures[hero].get_position();
                for i in 0..self.creatures.len() {
                    if i == hero {
                        continue;
                    }

                    let monster_pos = self.creatures[i].get_position();

                    let sees_player = self.los(&hero_pos, &monster_pos);

                    let path_id = self.creatures[i].path_id;

                    let searching_path = self.creatures[i].path_id.is_some();

                    match path_id {
                        None => {}
                        Some(ref id) => {
                            let path_opt = path_map.remove(id);
                            match path_opt {
                                None => {}
                                Some(ref path) => {
                                    self.creatures[i].path_id = None;
                                    //match path.len() {
                                    //0 => {},
                                    //_ => {
                                    self.creatures[i].set_path(path);
                                    self.creatures[i].pop_path_node();
                                    //}
                                    //}
                                }
                            }
                        }
                    }
                    let has_path = self.creatures[i].has_path();

                    let req_path = sees_player
                        && ((!has_path) || (!searching_path && self.creatures[i].path_age > 0.25));

                    if req_path {
                        self.request_path_update(i, hero);
                    }

                    // TODO reconcile this with collision somehow
                    if has_path {
                        self.creatures[i].path_age += delta;
                        let tsz = self.tile_size as f32;
                        let first_coords = self.creatures[i].get_target_node().expect("UGH");
                        let (tx, ty) = first_coords;
                        let (wx, wy) = (tx as f32 * tsz, ty as f32 * tsz);
                        let wv = Vector2f::new(wx, wy);

                        let chase_dist = 16.0 * delta;
                        let mut dist_remaining = chase_dist;

                        self.creatures[i].update_anim(delta);
                        while dist_remaining > 0.0 && self.creatures[i].has_path() {
                            let pos_dif = wv - monster_pos;
                            let dif_len = (pos_dif.x * pos_dif.x + pos_dif.y * pos_dif.y).sqrt();
                            if dif_len < dist_remaining {
                                let cr = &mut self.creatures[i];
                                cr.set_position(&wv);
                                cr.pop_path_node();
                                dist_remaining -= dif_len;
                            } else {
                                let (dx, dy) = (pos_dif.x, pos_dif.y);
                                let rads = dy.atan2(dx);
                                self.creatures[i].move_polar_rad(chase_dist, rads);
                                self.creatures[i].set_facing_rad(rads);
                                dist_remaining = 0.0;
                            }
                        }
                    }
                }
            }
        }

        // collision
        self.resolve_all_collisions();

        // updates
        // for creature in self.creatures.iter_mut() {
        // 	creature.update_anim(delta);
        // }

        // set up screen view
        let ws = window.size();
        self.view
            .set_size(Vector2f::new(ws.x as f32 / mag, ws.y as f32 / mag));

        match player {
            None => {}
            Some(hero) => self.view.set_center(self.creatures[hero].get_position()),
        }

        // figure out visible tiles

        let view_size = self.view.size();
        let view_center = self.view.center();
        let view_half = view_size / 2.0f32;

        let view_left = view_center.x - view_half.x;
        let view_right = view_center.x + view_half.x;
        let view_top = view_center.y - view_half.y;
        let view_bottom = view_center.y + view_half.y;

        let coord_left = self.tile_coord_from_position(view_left);
        let coord_right = self.tile_coord_from_position(view_right);
        let coord_top = self.tile_coord_from_position(view_top);
        let coord_bottom = self.tile_coord_from_position(view_bottom);

        self.vis_x = coord_left..(coord_right + 1);
        self.vis_y = coord_top..(coord_bottom + 1);

        for y in self.vis_y.clone() {
            for x in self.vis_x.clone() {
                match self.tile_idx_from_coords((x, y)) {
                    None => {}
                    Some(idx) => {
                        // set visibility based on player
                        match player {
                            None => {
                                self.tiles[idx].seen = true;
                                self.tiles[idx].visible = true;
                            }
                            Some(hero) => {
                                let hero_pos = self.creatures[hero].get_position();
                                let hero_coords =
                                    self.tile_coords_from_position((hero_pos.x, hero_pos.y));
                                let (hero_x, hero_y) = hero_coords;
                                let t = self.tiles[idx].tile;

                                if self.los_coords(hero_x, hero_y, t.x, t.y) {
                                    self.tiles[idx].seen = true;
                                    self.tiles[idx].visible = true;
                                } else {
                                    self.tiles[idx].visible = false;
                                }
                            }
                        }
                        // color
                        let color = if self.tiles[idx].visible || self.debug_los {
                            Color::WHITE
                        } else {
                            Color {
                                r: 100,
                                g: 75,
                                b: 75,
                                a: 255,
                            }
                        };
                        for spr in self.tiles[idx].sprites.iter_mut() {
                            spr.set_color(color);
                        }
                    }
                }
            }
        }
    }

    fn request_path_update(&mut self, i: usize, hero: usize) {
        let hero_coords = self.tile_coords_from_creature(&self.creatures[hero]);
        let new_target = Some(hero_coords);
        if self.creatures[i].path_target == new_target {
            return;
        }
        let id = self.path_count;
        self.path_count += 1;
        self.creatures[i].path_id = Some(id);
        self.creatures[i].path_target = new_target;
        self.creatures[i].awake = true;
        let rawr_coords = self.tile_coords_from_creature(&self.creatures[i]);
        let solver_idx = i % self.solvers.len();
        self.solvers[solver_idx].queue_solve(id, self.graph.clone(), rawr_coords, hero_coords);
    }

    fn get_active_tiles(&self, bounds: &FloatRect) -> Vec<(isize, isize)> {
        let mut active_tiles = Vec::new();

        let top_left = (bounds.left, bounds.top);
        let bottom_right = (bounds.left + bounds.width, bounds.top + bounds.height);

        let top_left_tile = self.tile_coords_from_position(top_left);
        let bottom_right_tile = self.tile_coords_from_position(bottom_right);

        let (x1, y1) = top_left_tile;
        let (x2, y2) = bottom_right_tile;

        for y in y1..y2 + 1 {
            for x in x1..x2 + 1 {
                active_tiles.push((x, y));
            }
        }

        active_tiles
    }

    fn tile_coords_from_creature(&self, creature: &Creature) -> (isize, isize) {
        let pos = creature.get_position();
        self.tile_coords_from_position((pos.x, pos.y))
    }

    fn tile_coords_from_position(&self, pos: (f32, f32)) -> (isize, isize) {
        let (x, y) = pos;
        (
            self.tile_coord_from_position(x),
            self.tile_coord_from_position(y),
        )
    }

    fn tile_coord_from_position(&self, coord: f32) -> isize {
        let t_size = self.tile_size as f32;
        let t_half = self.tile_size as f32 / 2.0;
        ((coord + t_half) / t_size).floor() as isize
    }

    fn tile_data_from_coords(&'a self, tile_coords: (isize, isize)) -> Option<&'a TileData> {
        match self.tile_idx_from_coords(tile_coords) {
            None => None,
            Some(idx) => Some(&self.tiles[idx]),
        }
    }

    fn tile_idx_from_coords(&self, tile_coords: (isize, isize)) -> Option<usize> {
        let (x_idx, y_idx) = tile_coords;

        if x_idx < 0 || y_idx < 0 || x_idx >= self.dungeon.width || y_idx >= self.dungeon.height {
            return None;
        }

        let idx = (x_idx + y_idx * (self.dungeon.width)) as usize;

        if idx >= self.tiles.len() {
            return None;
        }

        Some(idx)
    }

    fn sprite_depth_sort(&mut self) {
        // simple bubble depth sort -- acceptable because after init,
        // creatures swap depth values relatively rarely, and bubble
        // sort only uses O(1) memory ;)
        // TODO insertion sort because it's slightly better
        let mut shuffled = true;
        while shuffled {
            shuffled = false;
            for i in 0..self.creatures.len() - 1 {
                let a = i;
                let b = i + 1;
                let apos = self.creatures[a].get_bounds();
                let bpos = self.creatures[b].get_bounds();
                let ay = apos.top + apos.height;
                let by = bpos.top + bpos.height;
                if by < ay {
                    shuffled = true;
                    let a_copy = self.creatures[a].clone();
                    self.creatures[a] = self.creatures[b].clone();
                    self.creatures[b] = a_copy;
                }
            }
        }
    }

    fn resolve_all_collisions(&mut self) {
        for i in 0..self.creatures.len() - 1 {
            let bounds = self.creatures[i].get_bounds();
            let active = self.get_active_tiles(&bounds);

            // creature-creature collision
            for j in i + 1..self.creatures.len() {
                let i_box = &self.creatures[i].get_bounds();
                let j_box = &self.creatures[j].get_bounds();
                let offsets = self.collide.resolve_weighted(i_box, j_box, 0.5);
                match offsets {
                    None => {}
                    Some(vectors) => {
                        let (a, b) = vectors;
                        self.creatures[i].move_by(&a);
                        self.creatures[j].move_by(&b);
                    }
                }
            }

            // creature-wall collision
            for coords in active.iter() {
                let idx = self
                    .tile_idx_from_coords(coords.clone())
                    .expect("Can't collide creature/wall for negative index");
                if !self.tiles[idx].is_passable() {
                    let offset = self.collide.resolve_weighted(
                        &self.tiles[idx].bounds,
                        &self.creatures[i].get_bounds(),
                        1.0,
                    );
                    match offset {
                        None => {}
                        Some(coords) => {
                            let (_, offset) = coords;
                            let (tx, ty) = (self.tiles[idx].tile.x, self.tiles[idx].tile.y);
                            let check = if offset.x > 0.0 {
                                Some((tx + 1, ty))
                            } else if offset.x < 0.0 {
                                Some((tx - 1, ty))
                            } else if offset.y > 0.0 {
                                Some((tx, ty + 1))
                            } else if offset.y < 0.0 {
                                Some((tx, ty - 1))
                            } else {
                                None
                            };

                            match check {
                                None => {}
                                Some(check_coords) => {
                                    if !active.contains(&check_coords)
                                        || self
                                            .tile_data_from_coords(check_coords)
                                            .expect("hunH?!")
                                            .is_passable()
                                    {
                                        self.creatures[i].move_by(&offset);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn draw(&self, game: &mut Game, window: &mut RenderWindow) {
        window.set_view(&self.view);

        window.clear(Color::BLACK);

        let mut hero_pos = None;

        for creature in self.creatures.iter() {
            if creature.player {
                let pos = creature.get_position();
                hero_pos = Some(pos);
                break;
            }
        }

        for y in self.vis_y.clone() {
            for x in self.vis_x.clone() {
                match self.tile_idx_from_coords((x, y)) {
                    None => {}

                    Some(idx) => {
                        // first draw sprites
                        if self.tiles[idx].visible || self.tiles[idx].seen || self.debug_los {
                            for sprite in self.tiles[idx].sprites.iter() {
                                window.draw(sprite.deref());
                            }
                        }
                        // then maybe nodes
                        if self.tiles[idx].is_passable() && self.debug_graph {
                            let mut circle = self.debug_node_circle.clone();
                            circle.set_position(Vector2f::new(
                                self.tiles[idx].tile.x as f32 * self.tile_size as f32,
                                self.tiles[idx].tile.y as f32 * self.tile_size as f32,
                            ));
                            window.draw(&circle);
                        }
                    }
                }
            }
        }

        for creature in self.creatures.iter() {
            match hero_pos {
                None => creature.draw(window),
                Some(ref pos) => {
                    if self.debug_los || self.los(pos, &creature.get_position()) {
                        creature.draw(window);
                        if self.debug_graph {
                            match creature.get_path() {
                                None => {}
                                Some(path) => {
                                    // draw_line(&mut self, window: &mut RenderWindow,
                                    // start: &Vector2f, end: &Vector2f, color: &Color)

                                    // draw red line from creature to current target node
                                    let (start_tx, start_ty) = path[0];
                                    let start_wx = start_tx as f32 * self.tile_sizef;
                                    let start_wy = start_ty as f32 * self.tile_sizef;
                                    let cpos = creature.get_position();
                                    let npos = Vector2f::new(start_wx, start_wy);
                                    game.draw_line(window, &cpos, &npos, &Color::RED);

                                    // if path len > 1, connect all nodes
                                    if path.len() > 1 {
                                        for i in 0..path.len() - 1 {
                                            let (atx, aty) = path[i];
                                            let (btx, bty) = path[i + 1];
                                            let t = self.tile_sizef;
                                            let (awx, awy): (f32, f32) =
                                                (atx as f32 * t, aty as f32 * t);
                                            let (bwx, bwy): (f32, f32) =
                                                (btx as f32 * t, bty as f32 * t);
                                            let apos = Vector2f::new(awx, awy);
                                            let bpos = Vector2f::new(bwx, bwy);
                                            game.draw_line(
                                                window,
                                                &apos,
                                                &bpos,
                                                &Color {
                                                    r: 255,
                                                    g: 255,
                                                    b: 255,
                                                    a: 150,
                                                },
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn los(&self, a: &Vector2f, b: &Vector2f) -> bool {
        let (ax, ay) = self.tile_coords_from_position((a.x, a.y));
        let (bx, by) = self.tile_coords_from_position((b.x, b.y));
        self.los_coords(ax, ay, bx, by)
    }

    fn get_tile_los(&self, coords: (isize, isize)) -> bool {
        let idx = self
            .tile_idx_from_coords(coords)
            .unwrap_or_else(|| panic!("FAILED getting tile LOS {:?}", coords));
        self.tiles[idx].is_clear()
    }

    // TODO better LOS algorithm
    fn los_coords(&self, x1: isize, y1: isize, x2: isize, y2: isize) -> bool {
        let xstep;
        let ystep;
        let mut error;
        let mut error_prev;
        let mut x = x1;
        let mut y = y1;
        let mut dx = x2 - x1;
        let mut dy = y2 - y1;
        let mut points = Vec::new();
        points.push((x1, y1));
        if dx < 0 {
            xstep = -1;
            dx *= -1;
        } else {
            xstep = 1;
        }
        if dy < 0 {
            ystep = -1;
            dy *= -1;
        } else {
            ystep = 1;
        }
        let ddx = 2 * dx;
        let ddy = 2 * dy;
        if ddx >= ddy {
            error = dx;
            error_prev = error;
            for _ in 0..dx {
                x += xstep;
                error += ddy;
                if error > ddx {
                    y += ystep;
                    error -= ddx;
                    if error + error_prev < ddx {
                        points.push((x, y - ystep));
                    } else if error + error_prev > ddx {
                        points.push((x - xstep, y));
                    } else {
                        points.push((x, y - ystep));
                        points.push((x - xstep, y));
                    }
                }
                points.push((x, y));
                error_prev = error;
            }
        } else {
            error = dy;
            error_prev = error;
            for _ in 0..dy {
                y += ystep;
                error += ddx;
                if error > ddy {
                    x += xstep;
                    error -= ddy;
                    if error + error_prev < ddy {
                        points.push((x - xstep, y));
                    } else if error + error_prev > ddy {
                        points.push((x, y - ystep));
                    } else {
                        points.push((x - xstep, y));
                        points.push((x, y - ystep));
                    }
                }
                points.push((x, y));
                error_prev = error;
            }
        }

        for coords in points.iter() {
            if !self.get_tile_los(*coords) {
                return false;
            }
        }

        true
    }
}

impl<'a> Screen for GameplayScreen<'a> {
    fn key_press(&mut self, _game: &mut Game, _window: &mut RenderWindow, key: Key) -> bool {
        match key {
            Key::Comma => {
                self.zoom_index -= 1;
                true
            }
            Key::Period => {
                self.zoom_index += 1;
                true
            }
            Key::G => {
                self.debug_graph = !self.debug_graph;
                true
            }
            Key::L => {
                self.debug_los = !self.debug_los;
                true
            }
            Key::D => {
                let all = self.debug_los && self.debug_graph;
                let target = !all;
                self.debug_los = target;
                self.debug_graph = target;
                true
            }
            _ => false,
        }
    }

    fn update(
        &mut self,
        game: &mut Game,
        window: &mut RenderWindow,
        delta: f32,
    ) -> Option<Box<dyn Screen>> {
        self.logic(game, window, delta);
        self.draw(game, window);
        None
    }
}

/* Tile Sprite */

struct TileData<'a> {
    pub sprites: Vec<Sprite<'a>>,
    pub bounds: FloatRect,
    pub tile: Tile,
    pub seen: bool,
    pub visible: bool,
}

impl<'a> TileData<'a> {
    pub fn new(bounds: &FloatRect, tile: &Tile) -> TileData<'a> {
        TileData {
            sprites: Vec::new(),
            bounds: bounds.clone(),
            tile: tile.clone(),
            seen: false,
            visible: false,
        }
    }
    pub fn is_passable(&self) -> bool {
        self.tile.t != Wall
    }
    pub fn is_clear(&self) -> bool {
        self.is_passable()
    }
}

///////////////// utility stuff
// TODO oh god this is so messy

impl<'a> GameplayScreen<'a> {
    // closure for non-diagonal connections
    fn connect_direct(&mut self, x: isize, y: isize, offset: (isize, isize)) -> bool {
        let (ox, oy) = offset;
        let x2 = x + ox;
        let y2 = y + oy;
        let idx2_opt = self.tile_idx_from_coords((x2, y2));
        match idx2_opt {
            None => false,
            Some(idx2) => match self.tiles[idx2].is_passable() {
                false => false,
                true => self
                    .graph
                    .write()
                    .ok()
                    .unwrap()
                    .connect_nodes_at(x, y, x2, y2),
            },
        }
    }

    // closure for diagonal connections
    fn connect_diag(
        &mut self,
        x: isize,
        y: isize,
        offset: (isize, isize),
        check1: (isize, isize),
        check2: (isize, isize),
    ) -> bool {
        let check1_idx_opt = self.tile_idx_from_coords(check1);
        let check2_idx_opt = self.tile_idx_from_coords(check2);
        match (check1_idx_opt, check2_idx_opt) {
            (Some(check1_idx), Some(check2_idx)) => match (
                self.tiles[check1_idx].is_passable(),
                self.tiles[check2_idx].is_passable(),
            ) {
                (true, true) => self.connect_direct(x, y, offset),
                (_, _) => false,
            },
            (_, _) => false,
        }
    }

    fn add_wall_check(
        &mut self,
        tile_data: &mut TileData<'a>,
        offset: (isize, isize),
        wall: &Sprite<'a>,
        wall_off: f32,
    ) -> bool {
        if offset == (0, 0) || tile_data.tile.t == Wall {
            return false;
        }
        let (ox, oy) = offset;
        let (x, y) = (tile_data.tile.x, tile_data.tile.y);
        match self.dungeon.get_tile_type(x + ox, y + oy) {
            Some(t) => match t {
                Wall => {
                    let mut spr = wall.clone();
                    let half = self.tile_sizef / 2.0;
                    spr.set_origin(Vector2f::new(half, half));
                    spr.set_position(Vector2f::new(
                        x as f32 * self.tile_sizef,
                        y as f32 * self.tile_sizef,
                    ));
                    spr.move_(Vector2f::new(wall_off * ox as f32, wall_off * oy as f32));
                    tile_data.sprites.push(spr);
                    true
                }
                _ => false,
            },
            _ => false,
        }
    }
}
