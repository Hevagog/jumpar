use crate::components;
use crate::events;
use bevy::prelude::*;

pub fn player_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (&mut components::Velocity, &mut components::PlayerState),
        With<components::Player>,
    >,
    config: Res<crate::resources::json_reader::Config>,
) {
    let (mut player_velocity, mut player_state) = query.single_mut();

    if keyboard_input.pressed(KeyCode::KeyA) {
        player_velocity.x = -config.objects.player.speed;
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        player_velocity.x = config.objects.player.speed;
    } else {
        player_velocity.x = 0.0;
    }

    if keyboard_input.pressed(KeyCode::Space) && player_state.grounded {
        player_velocity.y = config.objects.player.jump_force;
        player_state.grounded = false;
    }
}

pub fn player_on_block_moving_system(
    mut param_set: ParamSet<(
        Query<&mut components::Velocity, With<components::Player>>,
        Query<(&components::Velocity, &components::Block)>,
    )>,
    mut collision_events: EventReader<events::Collision>,
) {
    let mut collisions_to_handle = Vec::new();
    for collision in collision_events.read() {
        if let Some((block_velocity, _)) = param_set
            .p1()
            .iter()
            .find(|(_, b)| b.0 == collision.block_index)
        {
            collisions_to_handle.push((collision.side, *block_velocity));
        }
    }

    if let Ok(mut player_velocity) = param_set.p0().get_single_mut() {
        for (side, block_velocity) in collisions_to_handle {
            match side {
                events::CollisionSide::Top => {
                    player_velocity.x += 2.0 * block_velocity.x;
                }
                _ => {}
            }
        }
    }
}

pub fn player_bounds_system(
    mut query: Query<
        (
            &mut Transform,
            &mut components::Velocity,
            &mut components::PlayerState,
        ),
        With<components::Player>,
    >,
    config: Res<crate::resources::json_reader::Config>,
) {
    for (mut transform, mut velocity, mut player_state) in &mut query {
        let left_bound = config.wall_params.left_x
            + config.wall_params.thickness / 2.0
            + config.objects.player.size / 2.0;
        let right_bound = config.wall_params.right_x
            - config.wall_params.thickness / 2.0
            - config.objects.player.size / 2.0;
        let bottom_bound = config.wall_params.bottom_y
            + config.wall_params.thickness / 2.0
            + config.objects.player.size / 2.0;

        transform.translation.x = transform.translation.x.clamp(left_bound, right_bound);
        transform.translation.y = transform.translation.y.clamp(bottom_bound, f32::MAX);

        if transform.translation.y == bottom_bound {
            velocity.y = 0.0;
            player_state.grounded = true;
        }
    }
}
