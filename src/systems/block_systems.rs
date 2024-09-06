use crate::components;
use crate::resources;
use bevy::prelude::*;

pub fn move_block_system(
    mut block_query: Query<(
        &components::Collider,
        &mut Transform,
        &mut components::Direction,
        &mut components::Velocity,
        &components::Block,
    )>,
    config: Res<resources::json_reader::Config>,
) {
    for ((_, mut block_transform, mut direction, mut block_velocity, block_index), block_config) in
        block_query.iter_mut().zip(config.objects.blocks.iter())
    {
        match direction.0 > 0.0 {
            true => {
                let bound = config.wall_params.right_x
                    - config.wall_params.thickness / 2.0
                    - block_config.w / 2.0;
                if block_transform.translation.x >= bound {
                    direction.0 = -1.0;
                    block_velocity.x = -block_velocity.x;
                } else {
                    block_velocity.x = block_velocity.x;
                }
            }
            false => {
                let bound = config.wall_params.left_x
                    + config.wall_params.thickness / 2.0
                    + block_config.w / 2.0;
                if block_transform.translation.x <= bound {
                    direction.0 = 1.0;
                    block_velocity.x = -block_velocity.x;
                } else {
                    block_velocity.x = block_velocity.x;
                }
            }
        };
    }
}
