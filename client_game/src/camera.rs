//! CameraPlugin - the movement controls for the camera
//!
//! Christian Schmid - April 2021

use bevy::{prelude::*, render::camera::Camera};

use crate::controls::PlayerControls;

const CAM_SPEED: f32 = 2.0;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(move_camera.system());
    }
}

fn move_camera(
    time: Res<Time>,
    mut tr_q: QuerySet<(
        Query<&Transform, With<PlayerControls>>,
        Query<&mut Transform, With<Camera>>,
    )>,
) {
    let play_pos = tr_q.q0().iter().next().unwrap().translation;
    let mut cam_transform = tr_q.q1_mut().single_mut().unwrap();

    cam_transform.translation = cam_transform
        .translation
        .lerp(play_pos, time.delta().as_secs_f32() * CAM_SPEED);
}
