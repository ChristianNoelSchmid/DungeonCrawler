use std::ops::AddAssign;

use bevy::prelude::*;

pub const SPRITE_SCALE: f32 = 2.0;
pub const UNIT_SIZE: u32 = 32;
pub const ARENA_WIDTH: u32 = 50;
pub const ARENA_HEIGHT: u32 = 50;

pub struct ResourcesPlugin;
impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

pub struct Materials {
    pub head_material: Handle<ColorMaterial>,
    pub wall_material: Handle<ColorMaterial>,
}
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Size {
            width: x,
            height: x,
        }
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        head_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        wall_material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
    });

    let texture_handle = asset_server.load("world_texture.jpg");

    commands.insert_resource(texture_atlas.add(TextureAtlas::from_grid_with_padding(
        texture_handle,
        Vec2::new(49.0, 49.0),
        9,
        7,
        Vec2::new(3.0, 3.0),
    )));
}
