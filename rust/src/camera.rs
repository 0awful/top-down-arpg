use godot::engine::{Camera2D, ICamera2D};
use godot::prelude::*;

// TODO: Tilemap ref to lock to scene

#[derive(GodotClass)]
#[class(base = Camera2D)]
pub struct Camera {
    #[base]
    base: Base<Camera2D>,
}

#[godot_api]
impl Camera {}

#[godot_api]
impl ICamera2D for Camera {
    fn init(base: Base<Self::Base>) -> Self {
        Camera { base }
    }
}
