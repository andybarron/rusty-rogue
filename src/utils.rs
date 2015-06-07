pub use na::{Vec2};

// pub type int = isize;
// pub type uint = usize;
pub type float = f64;
pub type Vec2f = Vec2<float>;

pub trait Vec2Ext<N> {
    fn tuple(&self) -> (N, N);
}

impl<N> Vec2Ext<N> for Vec2<N> where N: Clone {
    fn tuple(&self) -> (N, N) {
        (self.x.clone(), self.y.clone())
    }
}

pub trait Tuple2Ext<N> {
    fn vector(&self) -> Vec2<N>;
}

impl<N> Tuple2Ext<N> for (N, N) where N: Clone {
    fn vector(&self) -> Vec2<N> {
        Vec2::new(self.0.clone(), self.1.clone())
    }
}