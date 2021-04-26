use std::time::Duration;

use bevy::prelude::*;

use crate::res::{Position, ARENA_HEIGHT, ARENA_WIDTH};

const MOVE_KEYS: [(KeyCode, Position); 4] = [
    (KeyCode::A, Position { x: -1, y: 0 }),
    (KeyCode::S, Position { x: 0, y: -1 }),
    (KeyCode::W, Position { x: 0, y: 1 }),
    (KeyCode::D, Position { x: 1, y: 0 }),
];

pub struct PlayerControls {
    move_repeat_timer: Timer,
}

impl PlayerControls {
    pub fn new() -> Self {
        Self {
            move_repeat_timer: Timer::new(Duration::from_millis(100), true),
        }
    }
}

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(player_movement.system());
    }
}

fn player_movement(
    time: Res<Time>,
    key_input: Res<Input<KeyCode>>,
    mut head_positions: Query<(&mut Position, &mut PlayerControls)>,
) {
    for (mut position, mut controls) in head_positions.iter_mut() {
        let mut tick = false;
        for (code, pos) in MOVE_KEYS.iter() {
            if key_input.just_pressed(*code) {
                *position += *pos;
            } else if key_input.pressed(*code) {
                tick = true;
                if controls.move_repeat_timer.just_finished() {
                    *position += *pos;
                }
            }
            if position.x > (ARENA_WIDTH - 1) as i32 {
                position.x = (ARENA_WIDTH - 1) as i32;
            } else if position.x < 0 {
                position.x = 0;
            }
            if position.y > (ARENA_HEIGHT - 1) as i32 {
                position.y = (ARENA_HEIGHT - 1) as i32;
            } else if position.y < 0 {
                position.y = 0;
            }
        }
        if tick {
            controls.move_repeat_timer.tick(time.delta());
        } else {
            controls.move_repeat_timer.reset();
        }
    }
}
