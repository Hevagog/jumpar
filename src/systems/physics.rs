use crate::components;
use crate::events;
use crate::resources;
use bevy::{math::bounding::Aabb2d, prelude::*};

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

pub fn detect_collision_system(
    player_query: Query<(&Transform, &components::PlayerState), With<components::Player>>,
    block_query: Query<(&components::Collider, &Transform)>,
    config: Res<resources::json_reader::Config>,
    mut collision_events: EventWriter<events::Collision>,
) {
    let (player_transform, player_state) = player_query.single();

    let player_aabb = Aabb2d::new(
        player_transform.translation.truncate(),
        Vec2::new(config.objects.player.size, config.objects.player.size),
    );

    if let Some(collision) = detect_collision(&player_aabb, &block_query, &config) {
        collision_events.send(collision);
    }
}

pub fn handle_collision_system(
    mut player_query: Query<
        (
            &mut Transform,
            &mut components::Velocity,
            &mut components::PlayerState,
        ),
        With<components::Player>,
    >,
    mut collision_events: EventReader<events::Collision>,
) {
    let (mut player_transform, mut player_velocity, mut player_state) = player_query.single_mut();

    for collision in collision_events.read() {
        match collision {
            events::Collision::Left => {
                player_transform.translation.x -= 0.01;
                player_velocity.x = 0.0;
            }
            events::Collision::Right => {
                player_transform.translation.x += 0.01;
                player_velocity.x = 0.0;
            }
            events::Collision::Top => {
                player_transform.translation.y += 0.01;
                player_velocity.y = 0.0;
            }
            events::Collision::Bottom => {
                player_transform.translation.y -= 0.01;
                player_velocity.y = 0.0;
                player_state.grounded = true;
            }
        }
    }
}

fn detect_collision(
    player_aabb: &Aabb2d,
    block_query: &Query<(&components::Collider, &Transform)>,
    config: &resources::json_reader::Config,
) -> Option<events::Collision> {
    for ((_, block_transform), block_config) in block_query.iter().zip(config.objects.blocks.iter())
    {
        let block_size = Vec2::new(block_config.w, block_config.h);
        let block_aabb = Aabb2d::new(block_transform.translation.truncate(), block_size);

        if overlap(&player_aabb, &block_aabb) {
            let collision = get_collision(&player_aabb, &block_aabb);
            return Some(collision);
        }
    }
    None
}

fn overlap(first: &Aabb2d, second: &Aabb2d) -> bool {
    first.min.x < second.max.x
        && first.max.x > second.min.x
        && first.min.y < second.max.y
        && first.max.y > second.min.y
}

fn get_collision(first: &Aabb2d, second: &Aabb2d) -> events::Collision {
    let x_overlap = first.max.x - second.min.x;
    let y_overlap = first.max.y - second.min.y;

    if x_overlap < y_overlap {
        if first.min.x < second.min.x {
            events::Collision::Left
        } else {
            events::Collision::Right
        }
    } else {
        if first.min.y < second.min.y {
            events::Collision::Bottom
        } else {
            events::Collision::Top
        }
    }
}
