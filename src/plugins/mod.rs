use crate::systems::physics::{
    apply_gravity, apply_velocity, detect_collision_system, handle_collision_system,
};
use crate::systems::player_systems::{
    player_bounds_system, player_movement_system, player_on_block_moving_system,
};
use bevy::prelude::*;

pub struct PhysicsPlugin;
pub struct PlayerPlugin;
pub struct GeneralPlugin;

impl Plugin for GeneralPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                apply_gravity,
                player_bounds_system,
                player_movement_system,
                player_on_block_moving_system,
                apply_velocity,
                detect_collision_system,
                handle_collision_system,
            )
                .chain(),
        );
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (player_bounds_system, player_movement_system).chain(),
        );
    }
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                apply_gravity,
                apply_velocity,
                detect_collision_system,
                handle_collision_system,
            )
                .chain(),
        );
    }
}
