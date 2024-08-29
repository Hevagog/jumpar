use bevy::prelude::*;

mod components;
mod plugins;
mod resources;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(resources::json_reader::JsonFilePath(
            "assets/config.json".to_string(),
        ))
        .add_systems(Startup, (resources::json_reader::read_json, setup).chain())
        .add_systems(
            FixedUpdate,
            (
                systems::physics::apply_gravity,
                systems::physics::apply_velocity,
                systems::player_systems::player_bounds_system,
                systems::player_systems::player_movement_system,
            )
                .chain(),
        )
        .run();
}

#[derive(Event, Default)]
struct CollisionEvent;

fn setup(mut commands: Commands, config: Res<resources::json_reader::Config>) {
    commands.spawn(Camera2dBundle::default());

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

    let goal_y = config.objects.goal.y + config.wall_params.bottom_y;
    commands.spawn((SpriteBundle {
        transform: Transform {
            translation: Vec3::new(config.objects.goal.x, goal_y, 0.0),
            scale: Vec3::splat(config.objects.player.size),
            ..default()
        },
        sprite: Sprite {
            color: Color::srgb(1.0, 0.5, 0.5),
            ..default()
        },
        ..default()
    },));

    commands.spawn(components::WallBundle::new(
        components::WallLocation::Bottom,
        &config,
    ));
    commands.spawn(components::WallBundle::new(
        components::WallLocation::Left,
        &config,
    ));
    commands.spawn(components::WallBundle::new(
        components::WallLocation::Right,
        &config,
    ));
}
