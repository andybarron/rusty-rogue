use std::rc::Rc;
// use petgraph::?;
use utils::*;
use poglgame::opengl_graphics::Texture;
use poglgame::sprite::Sprite;

#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct TileInfo {
    pub wall: bool,
    pub opaque: bool,
}

#[derive(Clone)]
pub struct Tile {
    pub info: TileInfo,
    pub sprite: Rc<Sprite<Texture>>,
}