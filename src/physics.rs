use utils::*;
use rect::*;

const EPSILON: float = 0.1;

pub struct CollisionResult {
    pub a: Vec2f,
    pub b: Vec2f,
    pub vertical: bool,
}

pub fn collide_rect_weighted(a: &Rectf, b: &Rectf, w: float)
        -> Option<CollisionResult>
{
    Rectf::get_overlap(a, b).map(|overlap| {
        assert!(0.0 <= w && w <= 1.0);
        let vertical = overlap.h() < overlap.w();
        let dist = if vertical { overlap.h() } else { overlap.w() } + EPSILON;
        let swap = if vertical {
            b.center().y < a.center().y
        } else {
            b.center().x < a.center().x
        };
        let offset = if vertical {
            Vec2f::new(0.0, dist)
        } else {
            Vec2f::new(dist, 0.0)
        } * if swap { -1.0 } else { 1.0 };
        CollisionResult {
            a: offset * (w - 1.0),
            b: offset * w,
            vertical: vertical,
        }
    })
}