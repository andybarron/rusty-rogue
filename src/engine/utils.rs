use std::fmt::Debug;
use sfml::system::Vector2f;

pub trait DebugString {
    fn ds(&self) -> String;
}

impl<T> DebugString for T where T: Debug {
    fn ds(&self) -> String {
        format!("{:?}", self)
    }
}

pub trait VectorExt {
    fn len(&self) -> f32;
}

impl VectorExt for Vector2f {
    fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

// use std::hash::{Hash, Hasher};
// impl Hash for T where T: DebugString + !Hash {
//     fn hash<H>(&self, state: &mut H) where H: Hasher {
//         self.ds().hash(state)
//     }
// }

// #[derive(Debug, Eq, PartialEq)]
// struct Foo;

// use std::collections::HashSet;
// fn test() {
//     let mut m = HashSet::new();
//     m.insert(Foo.ds());
// }