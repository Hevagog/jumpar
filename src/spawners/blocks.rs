use crate::components;
use crate::resources;
use bevy::prelude::*;

pub fn spawn_blocks(commands: &mut Commands, config: &Res<resources::json_reader::Config>) {
    for (index, block) in config.objects.blocks.iter().enumerate() {
        let vertical_speed = block.y + 300.0 + (index as f32 * 25.0);
        let b_index = match index % 2 == 0 {
            true => 1.0,
            false => -1.0,
        };
        commands.spawn((
            components::BlockBundle::new(block),
            components::Block(index),
            components::Direction(b_index as f32),
            components::Velocity(Vec2::new(vertical_speed * b_index, 0.0)),
        ));
    }
}
