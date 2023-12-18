use godot::engine::{Area2D, IArea2D};
use godot::prelude::*;

use crate::resources::Inventory;

#[derive(GodotClass)]
#[class(init, base = Area2D)]
pub struct Chest {
    #[base]
    base: Base<Area2D>,
    inventory: Gd<Inventory>,
}

#[godot_api]
impl Chest {}
