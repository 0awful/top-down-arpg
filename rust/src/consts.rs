use godot::prelude::*;

pub const PLAYER_SPEED: real = 300.0;
pub const MOB_SPEED: real = 200.0;
pub const KNOCKBACK_POWER: real = 20.0;

#[derive(Clone)]
pub enum FacingDirection {
    Up,
    Down,
    Left,
    Right,
}
