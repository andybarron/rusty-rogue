use std::collections::HashMap;
use std::rc::Rc;
// use petgraph::?;
use utils::*;
use poglgame::Texture;
use poglgame::Sprite;

#[derive(RustcDecodable, RustcEncodable, Debug, Clone, PartialEq, Eq)]
pub struct TileInfo {
    pub name: String,
    pub wall: bool,
    pub opaque: bool,
}

pub type TileMap = HashMap<String, TileInfo>;

#[derive(Clone)]
pub struct Tile {
    pub info: TileInfo,
    pub sprite: Rc<Sprite<Texture>>,
}

impl Default for TileInfo {
    fn default() -> Self {
        TileInfo {
            name: "".into(),
            wall: true,
            opaque: true,
        }
    }
}