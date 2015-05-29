extern crate sfml;
use sfml::system::Vector2f;
use sfml::graphics::FloatRect;
use sfml::graphics::rc::Sprite;

pub struct Position(Vector2f);
pub struct Velocity(Vector2f);
pub struct Collision{width: f32, height: f32}
pub struct RenderSprite(Sprite);
// pub struct Health{max: u32, current: u32}