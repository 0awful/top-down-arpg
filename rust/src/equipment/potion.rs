use godot::engine::Sprite2D;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Sprite2D)]
pub struct Potion {
    #[base]
    base: Base<Sprite2D>,
}

#[godot_api]
impl Potion {}
