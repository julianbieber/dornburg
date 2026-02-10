use crate::player::PlayerMarker;
use avian2d::prelude::{LinearVelocity, RigidBody};
use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, Query, Res, Transform, With};

pub fn update_player_position(
    mut query: Query<(&RigidBody, &Transform, &mut LinearVelocity), With<PlayerMarker>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    // let up_key: KeyCode = KeyCode::KeyW; // unused as of now
    let left_key: KeyCode = KeyCode::KeyA;
    let right_key: KeyCode = KeyCode::KeyD;
    let jump_key: KeyCode = KeyCode::Space;
    // let down_key: KeyCode = KeyCode::KeyS; // unused as of now

    let directional_change_base = 5.0;
    let directional_change_threshold = 20.0;
    let directional_change_multiplier = 10.0;

    let max_horizontal_velocity = 300.0;

    for (i, (body, transform, mut linear_velocity)) in query.iter_mut().enumerate() {
        if keys.just_pressed(jump_key) {
            println!(
                "Player({}) POS {} {} {}!",
                i, transform.translation.x, transform.translation.y, transform.translation.z
            );
            linear_velocity.y = 250.0;
        }
        if keys.pressed(left_key) {
            if linear_velocity.x > directional_change_threshold {
                linear_velocity.x -= directional_change_base * directional_change_multiplier;
            } else if linear_velocity.x < -max_horizontal_velocity {
                linear_velocity.x = -max_horizontal_velocity;
            } else {
                linear_velocity.x -= directional_change_base;
            }
        }
        if keys.pressed(right_key) {
            if linear_velocity.x < directional_change_threshold {
                linear_velocity.x += directional_change_base * directional_change_multiplier;
            } else if linear_velocity.x > max_horizontal_velocity {
                linear_velocity.x = max_horizontal_velocity;
            } else {
                linear_velocity.x += directional_change_base;
            }
        }
    }
}
