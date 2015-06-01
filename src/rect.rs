use na::{Vec2};
use utils::*;
use components::*;

#[derive(Clone, Copy, PartialEq)]
pub struct Rect {
    data: [float; 4],
}

impl Rect {
    pub fn new(x: float, y: float, w: float, h: float) -> Self {
        Rect {
            data: [x, y, w, h],
        }
    }
    pub fn from_vectors(min: Vec2f, max: Vec2f) -> Self {
        Rect {
            data: [min.x, min.y, max.x - min.x, max.y - min.y],
        }
    }
    pub fn from_components(pos: Position, col: Collision) -> Self {
        Self::new(pos.0.x, pos.0.y, col.w, col.h)
    }
    pub fn rounded(&self) -> Self {
        Self::new(
            self.x().round(),
            self.y().round(),
            self.w().round(),
            self.h().round())
    }
    pub fn translated(&self, amt: Vec2f) -> Self {
        Self::from_vectors(self.min() + amt, self.max() + amt)
    }
    pub fn translated2f(&self, x: float, y: float) -> Self {
        self.translated(Vec2f::new(x, y))
    }
    pub fn min(&self) -> Vec2f {
        Vec2f::new(self.data[0], self.data[1])
    }
    pub fn max(&self) -> Vec2f {
        Vec2f::new(self.data[2], self.data[3])
    }
    pub fn x(&self) -> float {
        self.data[0]
    }
    pub fn y(&self) -> float {
        self.data[1]
    }
    pub fn w(&self) -> float {
        self.data[2]
    }
    pub fn h(&self) -> float {
        self.data[3]
    }
}

impl Into<[float; 4]> for Rect {
    fn into(self) -> [float; 4] {
        self.data
    }
}

impl From<[float; 4]> for Rect {
    fn from(other: [float; 4]) -> Self {
        Rect { data: other }
    }
}