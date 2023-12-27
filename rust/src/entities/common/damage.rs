use godot::engine::Node;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node
)]
pub struct Damage {
    #[base]
    base: Base<Node>,
    #[export]
    #[var]
    pub value: i32,
}

#[godot_api]
impl Damage {}
