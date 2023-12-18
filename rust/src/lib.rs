use godot::prelude::*;

pub mod resources;
pub use resources::*;

pub mod equipment;
pub use equipment::*;

pub mod player;
pub use player::*;

struct Template;

#[gdextension]
unsafe impl ExtensionLibrary for Template {}
