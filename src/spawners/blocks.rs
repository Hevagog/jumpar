use crate::components;
use crate::resources;
use bevy::prelude::*;

pub fn spawn_blocks(commands: &mut Commands, config: &Res<resources::json_reader::Config>) {
    for (index, block) in config.objects.blocks.iter().enumerate() {
        commands.spawn((
            components::BlockBundle::new(block),
            components::Block(index),
        ));
    }
}
