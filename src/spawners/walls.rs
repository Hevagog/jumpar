use crate::components;
use crate::resources;
use bevy::prelude::*;

pub fn spawn_walls(commands: &mut Commands, config: &Res<resources::json_reader::Config>) {
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
