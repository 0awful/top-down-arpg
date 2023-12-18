use godot::engine::Resource;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Resource)]
pub struct Item {
    #[base]
    base: Base<Resource>,
    #[export]
    pub name: GString,
}

#[godot_api]
impl Item {}
