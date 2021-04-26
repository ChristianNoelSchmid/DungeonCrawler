use bevy::{prelude::*, render::camera::{ActiveCamera, ActiveCameras, Camera}};

use crate::controls::PlayerControls;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(move_camera.system());
    }
}

fn move_camera(mut camera_transforms: Query<&mut Transform, With<ActiveCamera>>, mut player: Query<&Transform, With<PlayerControls>>) {
    let mut transform = camera_transforms.single_mut().unwrap();
    transform.translation = player.single_mut().unwrap().translation;
}