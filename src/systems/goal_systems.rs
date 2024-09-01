use crate::components;
use bevy::prelude::*;

pub fn goal_system(
    mut player_query: Query<(&Transform, &components::PlayerState), With<components::Player>>,
    goal_query: Query<(&Transform, &components::Goal)>,
    // mut game_state: ResMut<crate::resources::GameState>,
) {
    let (player_transform, _) = player_query.single_mut();
    let (goal_transform, _) = goal_query.single();

    if player_transform
        .translation
        .distance(goal_transform.translation)
        < 10.0
    {
        println!("Goal reached!");
    }
}
