use crate::components;
use crate::resources;
use bevy::prelude::*;

pub fn spawn_player(commands: &mut Commands, config: &Res<resources::json_reader::Config>) {
    let player_y = config.objects.player.y + config.wall_params.bottom_y;

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, player_y, 0.0),
                scale: Vec3::splat(config.objects.player.size),
                ..default()
            },
            sprite: Sprite {
                color: Color::srgb(0.5, 0.5, 1.0),
                ..default()
            },
            ..default()
        },
        components::Player,
        components::Collider,
        components::Velocity(Vec2::ZERO),
        components::Mass(config.objects.player.mass),
        components::PlayerState { grounded: false },
    ));
}
