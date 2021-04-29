//! CameraPlugin - the movement controls for the camera
//!
//! Christian Schmid - April 2021

use bevy::{
    prelude::*,
    render::camera::{Camera, OrthographicProjection},
};

use crate::controls::PlayerControls;

const CAM_SPEED: f32 = 4.0;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(move_camera.system())
            .add_startup_stage("camera_setup", SystemStage::single(setup_camera.system()));
    }
}

fn setup_camera(mut cam_ortho: Query<(&mut OrthographicProjection, &Transform), With<Camera>>) {
    let mut cam_ortho = cam_ortho.iter_mut().next().unwrap();
    cam_ortho.0.scale = 10.0;
    cam_ortho.0.near = -10000.0;
    cam_ortho.0.far = 100000.0;

    println!("{}", cam_ortho.1.translation.z);
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

    cam_transform.translation.z = 1000.0;
}
