use godot::engine::{Area2D, CharacterBody2D};
use godot::prelude::*;

use crate::consts::{KNOCKBACK_POWER, MOB_SPEED};
use crate::utils::get_nearest_groups;

#[derive(GodotClass)]
#[class(init, base = CharacterBody2D)]
pub struct WalkingEnemy {
    #[base]
    base: Base<CharacterBody2D>,

    #[init(default = MOB_SPEED)]
    pub speed: real,
    #[init(default = KNOCKBACK_POWER)]
    pub knockback: real,
    pub target: Option<Vector2>,
}
#[godot_api]
impl WalkingEnemy {
    #[func]
    pub fn hit_by_weapon(&mut self, area: Gd<Area2D>) {
        let area_groups = get_nearest_groups(area.clone().upcast());
        let self_groups = get_nearest_groups(self.base.clone().upcast());
        for item in area_groups.iter_shared() {
            if self_groups.contains(&item) {
                return;
            }
        }
        godot_print!("hit by {:?} not in its group", area)
    }

    #[func]
    pub fn hit_by_player(&mut self, area: Gd<Area2D>) {
        godot_print!("hit by {:?}", area)
    }

    #[func]
    pub fn move_self(&mut self) {}

    #[func]
    pub fn assign_target(&mut self, position: Vector2) {}

    #[func]
    pub fn knockback(&mut self) {}

    #[func]
    pub fn death(&mut self) {}
}
