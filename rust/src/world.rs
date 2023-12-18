use godot::engine::{INode2D, Node2D};
use godot::prelude::*;

/*
 * Should have:
 * 1. Gui
 * 2. Player
 * 3. Tilemap
 */

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct World {
    #[base]
    base: Base<Node2D>,
}

#[godot_api]
impl World {
    #[func]
    fn should_pause(&mut self) {
        if let Some(mut tree) = self.base.get_tree() {
            tree.set_pause(true);
        } else {
            godot_error!("Could not get tree to pause");
        }
    }
    #[func]
    fn should_resume(&mut self) {
        if let Some(mut tree) = self.base.get_tree() {
            tree.set_pause(false);
        } else {
            godot_error!("Could not get tree to resume");
        }
    }
}

#[godot_api]
impl INode2D for World {}
