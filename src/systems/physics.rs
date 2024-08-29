use crate::components;
use crate::resources;
use bevy::prelude::*;

pub fn apply_velocity(mut query: Query<(&mut Transform, &components::Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

pub fn apply_gravity(
    mut query: Query<
        (
            &mut components::Velocity,
            &components::Mass,
            &components::PlayerState,
        ),
        With<components::Player>,
    >,
    time: Res<Time>,
    config: Res<resources::json_reader::Config>,
) {
    let (mut velocity, mass, player_state) = query.single_mut();
    if !player_state.grounded {
        velocity.y -= config.physics.gravity * mass.0 * time.delta_seconds();
    }
}
