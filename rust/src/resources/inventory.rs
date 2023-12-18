use godot::engine::Resource;
use godot::prelude::*;

use crate::resources::InventorySlot;

#[derive(GodotClass)]
#[class(init, base = Resource)]
pub struct Inventory {
    #[base]
    base: Base<Resource>,
    #[export]
    pub items: Array<Gd<InventorySlot>>,
}

#[godot_api]
impl Inventory {}
