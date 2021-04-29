use bevy::{
    core::Time,
    math::Vec3,
    prelude::{CoreStage, IntoSystem, Plugin, Query, Res, SystemSet, Transform},
};

use crate::res::{Position, Size, UNIT_SIZE};

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

// Scales the game objects to the appropriate unit size (in res.rs)
fn size_scaling(mut q: Query<(&Size, &mut Transform)>) {
    for (size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            size.width * UNIT_SIZE as f32,
            size.height * UNIT_SIZE as f32,
            1.0,
        );
    }
}

fn position_translation(time: Res<Time>, mut q: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            (pos.x * UNIT_SIZE as i32) as f32,
            (pos.y * UNIT_SIZE as i32) as f32,
            0.0,
        );
    }
}
