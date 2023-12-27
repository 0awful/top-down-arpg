use godot::engine::Node;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct Health {
    #[base]
    base: Base<Node>,
    #[export]
    health: i32,
    #[export]
    max_health: i32,
}

#[godot_api]
impl Health {
    #[signal]
    pub fn health_hit_zero() {}

    #[func]
    pub fn on_damage_received(&mut self, amount: i32) {
        self.health -= amount;
        if self.health <= 0 {
            self.base.emit_signal("health_hit_zero".into(), &[]);
        }
    }

    #[func]
    pub fn on_healing_recieved(&mut self, amount: i32) {
        self.health += amount;
        if self.health > self.max_health {
            self.health = self.max_health;
        }
    }
}
