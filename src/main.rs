use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

mod components;
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
        .add_systems(Startup, (resources::json_reader::read_json, setup).chain())
        .add_plugins(plugins::PhysicsPlugin)
        .add_plugins(plugins::PlayerPlugin)
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
}
