use godot::engine::{INode2D, Node2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct Weapon {
    #[base]
    base: Base<Node2D>,
}

#[godot_api]
impl Weapon {}
