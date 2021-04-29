use crate::{dungeons::inst::Dungeon, mesh::gen_dun_mesh};
use crate::res::{ARENA_HEIGHT, ARENA_WIDTH};
use bevy::prelude::*;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_startup_system(spawn_bounds.system());
    }
}

// The system which spawns a new snake
fn spawn_bounds(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    //let texture_handle = asset_server.load("world_texture.jpg");
    let mesh = meshes.add(gen_dun_mesh(&Dungeon::new(ARENA_WIDTH, ARENA_HEIGHT)));

    commands
        .spawn()
        .insert_bundle(MeshBundle {
            mesh,
            visible: Visible {
                is_visible: true,
                is_transparent: true
            },
            ..Default::default()
        });
}