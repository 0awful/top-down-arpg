use godot::engine::{Area2D, IArea2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = Area2D)]
pub struct ProjectileWeapon {
    #[base]
    base: Base<Area2D>,
}

#[godot_api]
impl ProjectileWeapon {
    #[func]
    pub fn attack(&mut self) {
        // There's an animation that plays on the weapon and then a projectile spawns
        // Thrown are different with a looping animation until impact
    }
}

#[godot_api]
impl IArea2D for ProjectileWeapon {
    fn init(base: Base<Self::Base>) -> Self {
        ProjectileWeapon { base }
    }
}
