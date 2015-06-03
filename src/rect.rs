use na;
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
    pub fn from_bounds(min_x: float, min_y: float, max_x: float, max_y: float)
            -> Self
    {
        Rect {
            data: [min_x, min_y, max_x - min_x, max_y - min_y],
        }
    }
    pub fn from_vectors(min: Vec2f, max: Vec2f) -> Self {
        Self::from_bounds(min.x, min.y, max.x, max.y)
    }
    pub fn from_components(pos: &Position, col: &Collision) -> Self {
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
        self.min() + Vec2f::new(self.data[2], self.data[3])
    }
    pub fn center(&self) -> Vec2f {
        ( self.min() + self.max() ) / 2.0
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
    pub fn get_overlap(a: &Self, b: &Self) -> Option<Rect> {
        let (amin, amax) = (a.min(), a.max());
        let (bmin, bmax) = (b.min(), b.max());
        if amax.x <= bmin.x || amin.x >= bmax.x
        || amax.y <= bmin.y || amin.y >= bmax.y {
            None
        } else {
            let max_x = float::min(amax.x, bmax.x);
            let min_x = float::max(amin.x, bmin.x);
            let max_y = float::min(amax.y, bmax.y);
            let min_y = float::max(amin.y, bmin.y);
            let r = Rect::from_bounds(min_x, min_y, max_x, max_y);
            Some(r)
        }
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