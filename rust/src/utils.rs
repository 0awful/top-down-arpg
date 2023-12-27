use crate::consts::FacingDirection;
use crate::entities::{Damage, Health, Knockback};
use crate::meta::GodotType;
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

pub fn get_nearest<T>(node: Gd<Node>, name: GString) -> Option<Gd<T>>
where
    T: Inherits<Node>,
{
    if let Some(target) = node.try_get_node_as::<T>(name.clone()) {
        return Some(target);
    }
    if let Some(parent) = node.get_parent() {
        return get_nearest::<T>(parent, name.clone());
    }
    return None;
}

// Takes two nodes. The first should have a subnode that exposes a health type named health the second should expose a damage type named damage.
// It extracts the damage value and then applies it as damage against the health node.
pub fn damage_handler(health: Gd<Node>, damage: Gd<Node>) {
    if let Some(damage) = get_nearest::<Damage>(damage, "Damage".into()) {
        if let Some(mut health) = get_nearest::<Health>(health, "Health".into()) {
            health
                .bind_mut()
                .on_damage_received(damage.bind().get_value())
        } else {
            godot_error!("Damage Handler encountered node without health. This suggests something is missing health nodes")
        }
    } else {
        godot_error!("Damage Handler encountered nodes without damage. This suggests something is missing damage nodes");
    }
}

pub fn check_overlap<T>(first: Array<T>, second: Array<T>) -> bool
where
    T: GodotType,
{
    for item in first.iter_shared() {
        if second.contains(&item) {
            return true;
        }
    }
    return false;
}
