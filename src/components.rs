use utils::{float, Vec2f};

#[derive(Clone, Copy)]
pub struct Position(pub Vec2f);
#[derive(Clone, Copy)]
pub struct Velocity(pub Vec2f);
#[derive(Clone, Copy)]
pub struct Collision{pub w: float, pub h: float}

// pub struct Position(Vector2f);
// pub struct Velocity(Vector2f);
// pub struct Collision{width: f32, height: f32}
// pub struct RenderSprite(Sprite);
// pub struct Health{max: u32, current: u32}