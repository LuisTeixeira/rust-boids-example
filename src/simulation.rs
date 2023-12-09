use crate::math::Vector2D;


pub const SIZE: Vector2D = Vector2D::new(1600.0, 1000.0);

#[derive(Debug)]
pub enum Msg {
    Tick,
}