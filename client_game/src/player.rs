use crate::{
    controls::PlayerControls,
    res::{Materials, Position, Size},
};
use bevy::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_startup_stage("game_setup", SystemStage::single(spawn_player.system()));
    }
}

// The system which spawns the player
fn spawn_player(mut commands: Commands, materials: Res<Materials>) {
    // Create a new Entity, composed of a Sprite component, Size,
    // Position, and PlayerControls component
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.head_material.clone(),
            sprite: Sprite::new(Vec2::new(1.0, 1.0)),
            ..Default::default()
        })
        .insert(PlayerControls::new())
        .insert(Position { x: 3, y: 3 })
        .insert(Size::square(1.0));
}
