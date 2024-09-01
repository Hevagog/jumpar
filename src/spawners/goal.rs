use crate::components;
use crate::resources;
use bevy::prelude::*;

pub fn spawn_goal(commands: &mut Commands, config: &Res<resources::json_reader::Config>) {
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
}
