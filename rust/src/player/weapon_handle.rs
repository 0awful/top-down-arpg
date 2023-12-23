use std::f32::consts::PI;

use godot::engine::Node2D;
use godot::prelude::*;

use crate::consts::FacingDirection;
use crate::SwingableWeapon;

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct WeaponHandle {
    #[base]
    base: Base<Node2D>,
}

#[godot_api]
impl WeaponHandle {
    pub fn attack(&mut self, direction: FacingDirection) {
        match direction {
            FacingDirection::Right => self.base.set_global_rotation(0.25 * 2.0 * PI),
            FacingDirection::Down => self.base.set_global_rotation(0.5 * 2.0 * PI),
            FacingDirection::Left => self.base.set_global_rotation(0.75 * 2.0 * PI),
            FacingDirection::Up => self.base.set_global_rotation(0.0),
        }

        if let Some(mut swingable) = self.base.try_get_node_as::<SwingableWeapon>("Equiped") {
            return swingable.bind_mut().attack();
        }
        godot_error!("Error in attack call no node named 'Equiped' of expected type")
    }

    #[func]
    pub fn hide(&mut self) {
        if let Some(mut swingable) = self.base.try_get_node_as::<SwingableWeapon>("Equiped") {
            swingable.bind_mut().hide();
        }
    }
}
