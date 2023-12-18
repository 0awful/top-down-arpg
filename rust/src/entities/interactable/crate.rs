use godot::engine::Area2D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Area2D)]
pub struct Crate {
    #[base]
    base: Base<Area2D>,
}

#[godot_api]
impl Crate {}
