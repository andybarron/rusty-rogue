use super::tiles::*;

#[derive(Clone)]
pub struct Dungeon {
    tiles: Vec<TileInfo>,
    params: DungeonParams,
    map: TileMap,
}

#[derive(Clone, RustcEncodable, RustcDecodable, Debug)]
pub struct DungeonParams {
    pub seed: u32,
    pub size: (usize, usize),
    pub rooms: (usize, usize),
    pub floor: String,
    pub wall: String,
    pub room_size: (usize, usize),
}

#[derive(Clone, Copy, Debug)]
pub struct Room {
    pub x: isize,
    pub y: isize,
    pub w: usize,
    pub h: usize,
}

impl Room {
    pub fn center(&self) -> (isize, isize) {
        (
            self.x + (self.w as isize/2),
            self.y + (self.h as isize/2)
        )
    }
}

impl Dungeon {
    pub fn empty(params: DungeonParams, map: TileMap) -> Self {
        let (w, h) = params.size;
        let wall_tile = (&map)[&params.wall].clone();
        Dungeon {
            tiles: vec![wall_tile; w*h],
            params: params,
            map: map,
        }
    }
    pub fn get(&self, x: isize, y: isize) -> Option<&TileInfo> {
        self.tile_idx(x, y).and_then(|idx| self.tiles.get(idx))
    }
    pub fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut TileInfo> {
        match self.tile_idx(x, y) {
            None => None,
            Some(i) => self.tiles.get_mut(i)
        }
    }
    pub fn set(&mut self, x: isize, y: isize, t: &TileInfo) -> bool {
        self.get_mut(x, y).map(|old| *old = t.clone()).is_some()
    }
    pub fn valid_room(&self, r: Room, t: &TileInfo) -> bool {
        for y in (r.y-1)..(r.y+r.h as isize+1) {
            for x in (r.x-1)..(r.x+r.w as isize+1) {
                match self.get(x, y) {
                    None => return false,
                    Some(ty) if ty == t => return false,
                    _ => {},
                }
            }
        }
        true
    }
    pub fn fill_room(&mut self, r: Room, t: &TileInfo) -> bool {
        if !self.valid_room(r, t) { return false; }
        assert!(r.x > 0);
        assert!(r.x + (r.w as isize) < self.get_width() as isize);
        assert!(r.y > 0);
        assert!(r.y + (r.h as isize) < self.get_height() as isize);
        for y in (r.y)..(r.y+r.h as isize) {
            for x in (r.x)..(r.x+r.w as isize) {
                self.set(x, y, t);
            }
        }
        true
    }
    pub fn print(&self) {
        for y in 0..(self.get_height() as isize) {
            for x in 0..(self.get_width() as isize) {
                let t = self.get(x, y).unwrap();
                let c = if t.wall { '*' } else { ' ' };
                print!("{}", c);
            }
            println!("");
        }
        println!("Size: {}x{}", self.get_width(), self.get_height());
    }
    /* getters */
    pub fn get_width(&self) -> usize {
        self.params.size.0
    }
    pub fn get_height(&self) -> usize {
        self.params.size.1
    }
    pub fn get_params(&self) -> &DungeonParams {
        &self.params
    }
    pub fn get_map(&self) -> &TileMap {
        &self.map
    }
    /* private functions */
    fn tile_idx(&self, x: isize, y: isize) -> Option<usize> {
        if x < 0 || y < 0 {
            None
        } else {
            let uy = y as usize;
            let ux = x as usize;
            Some( ux + uy * self.get_width() )
        }
    }
}