use bevy::prelude::*;

const GRAVITY: f32 = 9.8;

const PLAYER_SIZE: f32 = 30.0;
const PLAYER_SPEED: f32 = 400.0;
const PLAYER_MASS: f32 = 40.0;
const PLAYER_JUMP_FORCE: f32 = 230.0;
const PLAYER_START_Y: f32 = 100.0;

const WALL_THICKNESS: f32 = 20.0;

// x coordinates
const LEFT_WALL_X: f32 = -400.0;
const RIGHT_WALL_X: f32 = 400.0;

// y coordinate
const BOTTOM_WALL_Y: f32 = -300.0;

const WALL_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                apply_gravity,
                apply_velocity,
                player_movement_system,
                player_bounds_system,
            )
        )
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Mass(f32);

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Component)]
struct Pad;

#[derive(Bundle)]
struct WallBundle {
    collider: Collider,
    sprite_bundle: SpriteBundle,
}

#[derive(Component)]
struct PlayerState {
    grounded: bool,
}

enum WallLocation {
    Bottom,
    Left,
    Right,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Bottom => Vec2::new(0.0, BOTTOM_WALL_Y),
            WallLocation::Left => Vec2::new(LEFT_WALL_X, 0.0), 
            WallLocation::Right => Vec2::new(RIGHT_WALL_X, 0.0),
        }
    }
    fn size(&self) -> Vec2 {
        let level_width = RIGHT_WALL_X - LEFT_WALL_X;
        assert!(level_width > 0.0);
        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, level_width + WALL_THICKNESS)
            }
            WallLocation::Bottom => {
                Vec2::new(level_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            collider: Collider,
            sprite_bundle: SpriteBundle {
                transform: Transform{
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
        }
    }
}

fn setup(
    mut commands: Commands, 
) {
    commands.spawn(Camera2dBundle::default());

    let player_y = PLAYER_START_Y + BOTTOM_WALL_Y;

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, player_y, 0.0),
                scale: Vec3::splat(PLAYER_SIZE),
                ..default()
            },
            sprite: Sprite {
                color: Color::srgb(0.5, 0.5, 1.0),
                ..default()
            },
            ..default()
        },
        Player,
        Collider,
        Velocity(Vec2::ZERO),
        Mass(PLAYER_MASS),
        PlayerState { grounded: false },
    ));
    
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
}

fn player_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut PlayerState), With<Player>>,
) {
    let (mut player_velocity, mut player_state ) = query.single_mut();

    if keyboard_input.pressed(KeyCode::KeyA) {
        player_velocity.x = -PLAYER_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        player_velocity.x = PLAYER_SPEED;
    } else {
        player_velocity.x = 0.0;
    }

    if keyboard_input.pressed(KeyCode::Space) && player_state.grounded {
        player_velocity.y = PLAYER_JUMP_FORCE;
        player_state.grounded = false;
    }
}

fn player_bounds_system(
    mut query: Query<(&mut Transform, &mut Velocity, &mut PlayerState), With<Player>>,
) {
    for (mut transform, mut velocity, mut player_state) in &mut query {
        let left_bound = LEFT_WALL_X + WALL_THICKNESS / 2.0 + PLAYER_SIZE / 2.0;
        let right_bound = RIGHT_WALL_X - WALL_THICKNESS / 2.0 - PLAYER_SIZE / 2.0;
        let bottom_bound = BOTTOM_WALL_Y + WALL_THICKNESS / 2.0 + PLAYER_SIZE / 2.0;

        transform.translation.x = transform.translation.x.clamp(left_bound, right_bound);
        transform.translation.y = transform.translation.y.clamp(bottom_bound, f32::MAX);

        if transform.translation.y == bottom_bound {
            velocity.y = 0.0;
            player_state.grounded = true;
        }
    }
}

fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity)>, 
    time: Res<Time>
) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn apply_gravity(
    mut query: Query<(&mut Velocity, &Mass, &PlayerState), With<Player>>,
    time: Res<Time>
) {
    let (mut velocity, mass, player_state) = query.single_mut();
    if !player_state.grounded {
        velocity.y -= GRAVITY * mass.0 * time.delta_seconds();
    }
}


