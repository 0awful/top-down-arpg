use crate::consts::FacingDirection;
use godot::prelude::*;

pub fn update_facing(movement_vec: Vector2) -> Option<FacingDirection> {
    if movement_vec.length_squared() > 0.0 {
        if movement_vec.x > 0.0 {
            return Some(FacingDirection::Right);
        } else if movement_vec.x < 0.0 {
            return Some(FacingDirection::Left);
        } else if movement_vec.y > 0.0 {
            return Some(FacingDirection::Down);
        } else if movement_vec.y < 0.0 {
            return Some(FacingDirection::Up);
        }
    }
    return None;
}
