use std::io::{stdin,Read,BufReader,BufRead};
use rustc_serialize::json;
use std::collections::HashMap;
use utils::*;

#[derive(RustcDecodable, RustcEncodable, Debug, Clone, Default)]
pub struct SpriteSetOptions {
    pub texture: String,
    pub size: (u32, u32),
    pub position: Vec<u32>,
    pub tile: Option<(u32, u32)>,
    pub variants: Option< HashMap<String, (u32, u32)> >,
}

impl SpriteSetOptions {
    pub fn validate(self) -> Result<Self, &'static str> {
        self.validate_internal()
    }
    fn validate_internal(&self) -> Result<Self, &'static str> {
        let num_coords = self.position.len();
        if num_coords < 2 || num_coords % 2 != 0 {
            return Err("Sprite `position` should be one or more x/y pairs");
        }
        Ok(self.clone())
    }
}