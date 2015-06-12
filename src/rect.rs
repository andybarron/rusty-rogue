use na;
use utils::*;
use components::*;
use num::traits::Num;

pub type Rectf = Rectangle<float>;
pub type Recti = Rectangle<int>;

#[derive(Clone, Copy, PartialEq)]
pub struct Rectangle<T> where T: Num + Copy {
    data: [T; 4],
}

impl<T> Rectangle<T> where T: Num + Copy {
    pub fn new(x: T, y: T, w: T, h: T) -> Self {
        Rectangle {
            data: [x, y, w, h],
        }
    }
    pub fn from_bounds(min_x: T, min_y: T, max_x: T, max_y: T)
            -> Self
    {
        Rectangle {
            data: [min_x, min_y, max_x - min_x, max_y - min_y],
        }
    }
    pub fn from_vectors(min: Vec2<T>, max: Vec2<T>) -> Self {
        Self::from_bounds(min.x, min.y, max.x, max.y)
    }
    pub fn translated(&self, amt: Vec2<T>) -> Self {
        Self::from_vectors(self.min() + amt, self.max() + amt)
    }
    pub fn translated2f(&self, x: T, y: T) -> Self {
        self.translated(Vec2::new(x, y))
    }
    pub fn min(&self) -> Vec2<T> {
        Vec2::new(self.data[0], self.data[1])
    }
    pub fn max(&self) -> Vec2<T> {
        self.min() + Vec2::new(self.data[2], self.data[3])
    }
    pub fn x(&self) -> T {
        self.data[0]
    }
    pub fn y(&self) -> T {
        self.data[1]
    }
    pub fn w(&self) -> T {
        self.data[2]
    }
    pub fn h(&self) -> T {
        self.data[3]
    }
}

impl Rectangle<float> {
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
    pub fn center(&self) -> Vec2<float> {
        ( self.min() + self.max() ) / 2.0
    }
}

impl<T> Rectangle<T> where T: Num + Copy + PartialOrd {
    pub fn get_overlap(a: &Self, b: &Self) -> Option<Self> {
        let (amin, amax) = (a.min(), a.max());
        let (bmin, bmax) = (b.min(), b.max());
        if amax.x <= bmin.x || amin.x >= bmax.x
        || amax.y <= bmin.y || amin.y >= bmax.y {
            None
        } else {
            let max_x = min(amax.x, bmax.x);
            let min_x = max(amin.x, bmin.x);
            let max_y = min(amax.y, bmax.y);
            let min_y = max(amin.y, bmin.y);
            let r = Rectangle::from_bounds(min_x, min_y, max_x, max_y);
            Some(r)
        }
    }

}

impl<T> Into<[T; 4]> for Rectangle<T> where T: Num + Copy {
    fn into(self) -> [T; 4] {
        self.data
    }
}

impl<T> From<[T; 4]> for Rectangle<T> where T: Num + Copy {
    fn from(other: [T; 4]) -> Self {
        Rectangle { data: other }
    }
}