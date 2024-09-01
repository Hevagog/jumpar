use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

mod components;
mod events;
mod plugins;
mod resources;
mod spawners;
mod systems;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin))
        .insert_resource(resources::json_reader::JsonFilePath(
            "assets/config.json".to_string(),
        ))
        .add_event::<events::Collision>()
        .add_systems(Startup, (resources::json_reader::read_json, setup).chain())
        .add_plugins(plugins::PhysicsPlugin)
        .add_plugins(plugins::PlayerPlugin)
        .add_systems(
            FixedUpdate,
            (
                systems::physics::apply_gravity,
                systems::physics::apply_velocity,
                systems::physics::detect_collision_system,
                systems::physics::handle_collision_system,
                systems::player_systems::player_bounds_system,
                systems::player_systems::player_movement_system,
                systems::goal_systems::goal_system,
            )
                .chain(),
        )
        .run();
}

<<<<<<< HEAD
fn setup(
    mut commands: Commands,
    config: Res<resources::json_reader::Config>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    spawners::player::spawn_player(&mut commands, &config);
    spawners::goal::spawn_goal(&mut commands, &config);
    spawners::walls::spawn_walls(&mut commands, &config);
    spawners::blocks::spawn_blocks(&mut commands, &config);
    spawners::ui::spawn_ui(&mut commands, &config, &asset_server);
=======
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
    commands.spawn((
        SpriteBundle {
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
        },
        components::Goal,
    ));

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
    for (index, block) in config.objects.blocks.iter().enumerate() {
        commands.spawn((
            components::BlockBundle::new(block),
            components::Block(index),
        ));
    }
>>>>>>> 005e999f427998970003eff01ae6955ca64deddd
}
