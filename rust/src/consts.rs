use godot::prelude::*;

pub const PLAYER_SPEED: real = 300.0;
#[derive(Clone)]
pub enum FacingDirection {
    Up,
    Down,
    Left,
    Right,
}
