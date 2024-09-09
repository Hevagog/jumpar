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
    if player_state.grounded == false {
        velocity.y -= config.physics.gravity * mass.0 * time.delta_seconds();
    }
}

pub fn detect_collision_system(
    mut player_query: Query<(&Transform, &mut components::PlayerState), With<components::Player>>,
    block_query: Query<(&components::Collider, &Transform, &components::Block)>,
    config: Res<resources::json_reader::Config>,
    mut collision_events: EventWriter<events::Collision>,
) {
    let (player_transform, mut player_state) = player_query.single_mut();

    let player_aabb = Aabb2d::new(
        player_transform.translation.truncate(),
        Vec2::new(
            config.objects.player.size / 2.0,
            config.objects.player.size / 2.0,
        ),
    );

    if let Some(collision) = detect_collision(&player_aabb, block_query, &config) {
        if collision.side == events::CollisionSide::Top {
            player_state.grounded = true;
        }
        collision_events.send(collision);
    } else {
        player_state.grounded = false;
    }
}

pub fn handle_collision_system(
    mut query_set: ParamSet<(
        Query<
            (
                &mut Transform,
                &mut components::Velocity,
                &mut components::PlayerState,
            ),
            With<components::Player>,
        >,
        Query<(&components::Block, &Transform)>,
    )>,
    mut collision_events: EventReader<events::Collision>,
    config: Res<resources::json_reader::Config>,
) {
    let mut collisions_to_handle = Vec::new();

    for collision in collision_events.read() {
        let block = config.objects.blocks.get(collision.block_index).unwrap();
        if let Some((_, block_transform)) = query_set
            .p1()
            .iter()
            .find(|(b, _)| b.0 == collision.block_index)
        {
            collisions_to_handle.push((collision.side, block.clone(), block_transform.translation));
        }
    }

    if let Ok((mut player_transform, mut player_velocity, mut player_state)) =
        query_set.p0().get_single_mut()
    {
        for (side, block, block_translation) in collisions_to_handle {
            match side {
                events::CollisionSide::Left => {
                    player_velocity.x = 0.0;
                    player_transform.translation.x =
                        block_translation.x - block.w / 2.0 - config.objects.player.size / 2.0;
                }
                events::CollisionSide::Right => {
                    player_velocity.x = 0.0;
                    player_transform.translation.x =
                        block_translation.x + block.w / 2.0 + config.objects.player.size / 2.0;
                }
                events::CollisionSide::Top => {
                    player_velocity.y = 0.0;
                    player_transform.translation.y =
                        block_translation.y + block.h / 2.0 + config.objects.player.size / 2.0;
                }
                events::CollisionSide::Bottom => {
                    player_velocity.y = 0.0;
                    player_transform.translation.y =
                        block_translation.y - block.h / 2.0 - config.objects.player.size / 2.0;
                }
            }
        }
    }

    collision_events.clear();
}

fn detect_collision(
    player_aabb: &Aabb2d,
    block_query: Query<(&components::Collider, &Transform, &components::Block)>,
    config: &resources::json_reader::Config,
) -> Option<events::Collision> {
    for ((_, block_transform, block_index), block_config) in
        block_query.iter().zip(config.objects.blocks.iter())
    {
        let block_size = Vec2::new(block_config.w / 2.0, block_config.h / 2.0);
        let block_aabb = Aabb2d::new(block_transform.translation.truncate(), block_size);

        if overlap(&player_aabb, &block_aabb) {
            let collision = get_collision(&player_aabb, &block_aabb);
            return Some(events::Collision {
                block_index: block_index.0,
                side: collision,
            });
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

fn get_collision(first: &Aabb2d, second: &Aabb2d) -> events::CollisionSide {
    let x_overlap = (first.max.x - second.min.x)
        .abs()
        .min((second.max.x - first.min.x).abs());
    let y_overlap = (first.max.y - second.min.y)
        .abs()
        .min((second.max.y - first.min.y).abs());

    if x_overlap < y_overlap {
        if first.min.x < second.min.x {
            events::CollisionSide::Left
        } else {
            events::CollisionSide::Right
        }
    } else {
        if first.min.y < second.min.y {
            events::CollisionSide::Bottom
        } else {
            events::CollisionSide::Top
        }
    }
}
