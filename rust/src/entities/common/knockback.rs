use godot::engine::Node;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct Knockback {
    #[base]
    base: Base<Node>,
    #[export]
    pub value: real,
}

#[godot_api]
impl Knockback {}
