use bevy::prelude::*;

#[derive(Event)]
pub struct Collision {
    pub block_index: usize,
    pub side: CollisionSide,
}

#[derive(Event, Debug, PartialEq, Eq, Copy, Clone)]
pub enum CollisionSide {
    Left,
    Right,
    Top,
    Bottom,
}
