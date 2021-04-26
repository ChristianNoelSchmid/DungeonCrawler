use crate::res::{Materials, Position, Size, ARENA_HEIGHT, ARENA_WIDTH};
use bevy::prelude::*;
pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_startup_stage("wall_setup", SystemStage::single(spawn_bounds.system()));
    }
}

// The system which spawns a new snake
fn spawn_bounds(mut commands: Commands, materials: Res<Materials>) {
    let mut spawn_wall = |x, y| {
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.wall_material.clone(),
                sprite: Sprite::new(Vec2::new(20.0, 20.0)),
                ..Default::default()
            })
            .insert(Position { x, y })
            .insert(Size::square(0.8));
    };

    for x in 0..ARENA_WIDTH as i32 {
        spawn_wall(x, 0);
        spawn_wall(x, (ARENA_HEIGHT - 1) as i32);
    }
    for y in 1..(ARENA_HEIGHT - 1) as i32 {
        spawn_wall(0, y);
        spawn_wall((ARENA_WIDTH - 1) as i32, y);
    }
}
