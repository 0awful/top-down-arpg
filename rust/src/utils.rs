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

// Returns the groups of self or nearest parent with groups.
// If there is no parent with groups returns an empty array.
pub fn get_nearest_groups(node: Gd<Node>) -> Array<StringName> {
    if node.get_groups().len() > 0 {
        return node.get_groups();
    }
    if let Some(parent) = node.get_parent() {
        return get_nearest_groups(parent);
    } else {
        return Array::new();
    }
}
