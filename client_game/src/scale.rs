use bevy::{core::Time, math::{Vec2, Vec3}, prelude::{CoreStage, IntoSystem, Plugin, Query, Res, SystemSet, Transform}, sprite::Sprite, window::Windows};

use crate::res::{Position, Size, ARENA_HEIGHT, ARENA_WIDTH};

pub struct ScalePlugin;

const TR_SPEED: f32 = 15.0;

impl Plugin for ScalePlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        // This is similar to LateUpdate in Unity
        app.add_system_set_to_stage(
            CoreStage::PostUpdate, // There's a number of these values
            // in CoreStage (such as Update, PreUpdate, etc.)
            SystemSet::new()
                .with_system(size_scaling.system())
                .with_system(position_translation.system()),
        );
    }
}

// Scales the game objects to their appropriate size
fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut sprite) in q.iter_mut() {
        sprite.size = Vec2::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.height() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
        );
    }
}

fn position_translation(time: Res<Time>, windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = transform.translation.lerp(Vec3::new(
            convert(pos.x as f32, window.height() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0),
        time.delta().as_secs_f32() * TR_SPEED);
    }
}
