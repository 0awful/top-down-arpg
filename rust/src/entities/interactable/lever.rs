use godot::engine::Area2D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = Area2D)]
pub struct Lever {
    #[base]
    base: Base<Area2D>,
}

#[godot_api]
impl Lever {}
