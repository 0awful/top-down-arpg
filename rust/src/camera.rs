use godot::engine::global::Side;
use godot::engine::{Camera2D, ICamera2D, TileMap};
use godot::prelude::*;

// TODO: Tilemap ref to lock to scene

#[derive(GodotClass)]
#[class(init, base = Camera2D)]
pub struct Camera {
    #[base]
    base: Base<Camera2D>,
    #[export]
    tilemap: Option<Gd<TileMap>>,
}

#[godot_api]
impl Camera {}

#[godot_api]
impl ICamera2D for Camera {
    fn ready(&mut self) {
        if let Some(tilemap) = self.tilemap.clone() {
            let map_rect = tilemap.get_used_rect();
            let tile_size = tilemap.get_rendering_quadrant_size();
            let world_size_in_px = map_rect.size * tile_size;
            let offset = 0;
            let buffer = 2 * tile_size;
            self.base
                .set_limit(Side::SIDE_RIGHT, world_size_in_px.x + offset + buffer);
            self.base
                .set_limit(Side::SIDE_BOTTOM, world_size_in_px.y + offset + buffer);
            self.base.set_limit(Side::SIDE_LEFT, offset - buffer);
            self.base.set_limit(Side::SIDE_TOP, offset - buffer);
        }
    }
}
