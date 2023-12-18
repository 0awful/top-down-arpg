use godot::engine::Resource;
use godot::prelude::*;

use crate::resources::Item;

#[derive(GodotClass)]
#[class(init, base = Resource)]
pub struct InventorySlot {
    #[base]
    base: Base<Resource>,
    #[export]
    pub item: Option<Gd<Item>>,
}

#[godot_api]
impl InventorySlot {}
