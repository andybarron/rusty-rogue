use std::cmp::PartialOrd;

pub use nalgebra::Vector2;
// pub type int = isize;
// pub type uint = usize;
pub type float = f64;
pub type int = i64;
pub type Vector2f = Vector2<float>;

pub trait Vector2Ext<N> {
    fn tuple(&self) -> (N, N);
}

impl<N> Vector2Ext<N> for Vector2<N>
where
    N: Clone,
{
    fn tuple(&self) -> (N, N) {
        (self.x.clone(), self.y.clone())
    }
}

pub trait Tuple2Ext<N> {
    fn vector(&self) -> Vector2<N>;
}

impl<N> Tuple2Ext<N> for (N, N)
where
    N: Clone,
{
    fn vector(&self) -> Vector2<N> {
        Vector2::new(self.0.clone(), self.1.clone())
    }
}

pub fn min<T>(a: T, b: T) -> T
where
    T: PartialOrd,
{
    if b < a {
        b
    } else {
        a
    }
}

pub fn max<T>(a: T, b: T) -> T
where
    T: PartialOrd,
{
    if b > a {
        b
    } else {
        a
    }
}
