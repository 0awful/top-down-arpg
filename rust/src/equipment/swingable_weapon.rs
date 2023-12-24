use godot::engine::{AnimationPlayer, Area2D, CollisionShape2D, Sprite2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Area2D)]
pub struct SwingableWeapon {
    #[base]
    base: Base<Area2D>,

    #[init(default = 0.2)]
    #[var]
    timeout: f64,
}

#[godot_api]
impl SwingableWeapon {
    #[func]
    pub fn attack(&mut self) {
        self.show();
        let mut animation_player = self.base.get_node_as::<AnimationPlayer>("AnimationPlayer");
        animation_player.play_ex().name("attack".into()).done();
        let mut timer = self
            .base
            .get_tree()
            .unwrap()
            .create_timer(self.timeout)
            .unwrap();
        timer.connect("timeout".into(), self.base.callable("attack_callback"));
    }

    #[func]
    pub fn attack_callback(&mut self) {
        self.hide();
    }

    #[func]
    pub fn show(&mut self) {
        self.base
            .get_node_as::<CollisionShape2D>("Hitbox")
            .set_disabled(false);
        self.base.get_node_as::<Sprite2D>("Sprite2D").show();
    }
    #[func]
    pub fn hide(&mut self) {
        self.base
            .get_node_as::<CollisionShape2D>("Hitbox")
            .set_disabled(true);
        self.base.get_node_as::<Sprite2D>("Sprite2D").hide();
    }
}
