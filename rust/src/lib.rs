use godot::prelude::*;

pub mod camera;
pub mod consts;
pub mod utils;
pub mod world;
pub mod zone;

pub mod resources;
pub use resources::*;

pub mod equipment;
pub use equipment::*;

pub mod player;
pub use player::*;

struct Template;

#[gdextension]
unsafe impl ExtensionLibrary for Template {}
