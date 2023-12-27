use godot::engine::{Area2D, CharacterBody2D};
use godot::prelude::*;

use crate::consts::MOB_SPEED;
use crate::entities::Knockback;
use crate::utils::{check_overlap, damage_handler, get_nearest, get_nearest_groups};

#[derive(GodotClass)]
#[class(init, base = CharacterBody2D)]
pub struct WalkingEnemy {
    #[base]
    base: Base<CharacterBody2D>,

    #[init(default = MOB_SPEED)]
    pub speed: real,
    pub target: Option<Vector2>,
}
#[godot_api]
impl WalkingEnemy {
    #[func]
    pub fn hurtbox_entered(&mut self, area: Gd<Area2D>) {
        // this could be pulled into a function. Potientially it would be able to put it on a hurtbox node.
        let area_groups = get_nearest_groups(area.clone().upcast());
        let self_groups = get_nearest_groups(self.base.clone().upcast());

        if check_overlap(area_groups, self_groups) {
            return;
        }

        godot_print!("Hurtbox entered");
        damage_handler(self.base.clone().upcast(), area.clone().upcast());
        // knockback is a vector. This won't work. Gotta change the API a little eventually.
        if let Some(knockback) = get_nearest::<Knockback>(area.clone().upcast(), "Knockback".into())
        {
            self.knockback(knockback.bind().value);
        } else {
            godot_error!(
                "attacking item initialized without knockback resource, {:?}",
                area
            )
        }
    }

    #[func]
    pub fn move_self(&mut self) {}

    #[func]
    pub fn assign_target(&mut self, position: Vector2) {}

    #[func]
    pub fn knockback(&mut self, amount: real) {
        godot_print!("received knockback {}", amount)
    }

    #[func]
    pub fn death(&mut self) {
        godot_print!("Enemy died");
        self.base.queue_free();
    }
}
