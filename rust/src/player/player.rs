use godot::engine::{AnimationPlayer, Area2D, CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

use crate::consts::{FacingDirection, PLAYER_SPEED};
use crate::player::WeaponHandle;
use crate::utils::{check_overlap, damage_handler, get_nearest, get_nearest_groups, update_facing};
use crate::Knockback;

#[derive(GodotClass)]
#[class(init, base = CharacterBody2D)]
pub struct Player {
    #[base]
    base: Base<CharacterBody2D>,
    #[init(default = PLAYER_SPEED)]
    speed: real,
    #[init(default = FacingDirection::Down)]
    facing_direction: FacingDirection,
}

#[godot_api]
impl Player {
    #[func]
    pub fn hide_weapon(&mut self) {
        self.base
            .get_node_as::<WeaponHandle>("WeaponHandle")
            .bind_mut()
            .hide();
    }

    #[func]
    pub fn handle_input(&mut self) {
        let input = Input::singleton();
        let movement_vec = input.get_vector(
            "walk_left".into(),
            "walk_right".into(),
            "walk_up".into(),
            "walk_down".into(),
        );

        if input.is_action_just_pressed("attack".into()) {
            self.attack();
        }
        self.base.set_velocity(movement_vec * self.speed)
    }

    pub fn attack(&mut self) {
        self.base
            .get_node_as::<WeaponHandle>("WeaponHandle")
            .bind_mut()
            .attack(self.facing_direction.clone());
    }

    #[func]
    pub fn update_animation(&mut self) {
        let mut animation_player = self.base.get_node_as::<AnimationPlayer>("AnimationPlayer");
        if self.base.get_velocity().length_squared() == 0.0 {
            animation_player.stop();
            return;
        }

        let animation_name = match self.facing_direction {
            FacingDirection::Down => "walk_down",
            FacingDirection::Left => "walk_left",
            FacingDirection::Right => "walk_right",
            FacingDirection::Up => "walk_up",
        };

        animation_player
            .play_ex()
            .name(animation_name.into())
            .done();
    }

    #[func]
    pub fn hurtbox_entered(&mut self, area: Gd<Area2D>) {
        let area_groups = get_nearest_groups(area.clone().upcast());
        let self_groups = get_nearest_groups(self.base.clone().upcast());
        if check_overlap(area_groups, self_groups) {
            return;
        }

        // get nearest damage
        // get damage at it's level. If not there, check parent and see if it has damage.
        // Eventually giving up and returning none
        damage_handler(self.base.clone().upcast(), area.clone().upcast());
        if let Some(knockback) = get_nearest::<Knockback>(area.clone().upcast(), "Knockback".into())
        {
            self.knockback(knockback.bind().value)
        } else {
            godot_error!(
                "attacking item initialize without knockback resource, {:?}",
                area
            )
        }
    }

    pub fn knockback(&mut self, knockback: real) {
        godot_print!("player recieved {} knockback", knockback);
    }

    #[func]
    pub fn death(&mut self) {
        self.base.queue_free();
    }
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn ready(&mut self) {
        self.hide_weapon();
    }

    fn physics_process(&mut self, _delta: f64) {
        self.handle_input();
        if let Some(facing_direction) = update_facing(self.base.get_velocity()) {
            self.facing_direction = facing_direction;
        }
        self.update_animation();
        self.base.move_and_slide();
    }
}
