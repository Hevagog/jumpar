use crate::resources;
use bevy::prelude::*;
pub mod player;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct Pad;

#[derive(Component)]
pub struct Goal;

#[derive(Bundle)]
pub struct WallBundle {
    pub collider: Collider,
    pub sprite_bundle: SpriteBundle,
}

#[derive(Component)]
pub struct PlayerState {
    pub grounded: bool,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Mass(pub f32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct Direction(pub f32);

#[derive(Component)]
pub struct Block(pub usize);

#[derive(Bundle)]
pub struct BlockBundle {
    pub collider: Collider,
    pub sprite_bundle: SpriteBundle,
}

pub enum WallLocation {
    Bottom,
    Left,
    Right,
}

impl WallBundle {
    pub fn new(location: WallLocation, config: &Res<resources::json_reader::Config>) -> WallBundle {
        WallBundle {
            collider: Collider,
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position(&config).extend(0.0),
                    scale: location.size(&config).extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::srgb_from_array(config.wall_params.color),
                    ..default()
                },
                ..default()
            },
        }
    }
}

impl WallLocation {
    fn position(&self, config: &Res<resources::json_reader::Config>) -> Vec2 {
        match self {
            WallLocation::Bottom => Vec2::new(0.0, config.wall_params.bottom_y),
            WallLocation::Left => Vec2::new(config.wall_params.left_x, 0.0),
            WallLocation::Right => Vec2::new(config.wall_params.right_x, 0.0),
        }
    }
    fn size(&self, config: &Res<resources::json_reader::Config>) -> Vec2 {
        let level_width = config.wall_params.right_x - config.wall_params.left_x;
        assert!(level_width > 0.0);
        match self {
            WallLocation::Left | WallLocation::Right => Vec2::new(
                config.wall_params.thickness,
                level_width + config.wall_params.thickness,
            ),
            WallLocation::Bottom => Vec2::new(
                level_width + config.wall_params.thickness,
                config.wall_params.thickness,
            ),
        }
    }
}

impl BlockBundle {
    pub fn new(block: &resources::json_reader::Block) -> BlockBundle {
        BlockBundle {
            collider: Collider,
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(block.x, block.y, 0.0),
                    scale: Vec3::new(block.w, block.h, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::srgb(0.5, 0.5, 0.5),
                    ..default()
                },
                ..default()
            },
        }
    }
}
