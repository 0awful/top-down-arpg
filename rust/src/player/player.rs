use godot::engine::{CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = CharacterBody2D)]
pub struct Player {
    #[base]
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl Player {}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<Self::Base>) -> Self {
        Player { base }
    }
}
