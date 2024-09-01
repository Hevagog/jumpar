use bevy::prelude::{Commands, Res, Resource};
use relative_path::RelativePath;
use serde::Deserialize;
use serde_json;
use std::env::current_dir;
use std::fs;

#[derive(Resource)]
pub struct JsonFilePath(pub String);

#[derive(Deserialize, Debug, Resource)]
pub struct Config {
    pub window: Window,
    pub canvas: Canvas,
    pub objects: Objects,
    pub wall_params: WallParams,
    pub physics: Physics,
}

#[derive(Deserialize, Debug)]
pub struct Physics {
    pub gravity: f32,
}

#[derive(Deserialize, Debug)]
pub struct WallParams {
    pub thickness: f32,
    pub color: [f32; 3],
    pub left_x: f32,
    pub right_x: f32,
    pub bottom_y: f32,
    pub pad_size: [f32; 2],
    pub pad_color: [f32; 3],
}

#[derive(Deserialize, Debug)]
pub struct Window {
    pub width: u32,
    pub height: u32,
}

#[derive(Deserialize, Debug)]
pub struct Canvas {
    pub width: u32,
    pub height: u32,
}

#[derive(Deserialize, Debug)]
pub struct Objects {
    pub blocks: Vec<Block>,
    pub player: Player,
    pub goal: Goal,
}

#[derive(Deserialize, Debug)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub speed: f32,
    pub mass: f32,
    pub jump_force: f32,
    pub start_y: f32,
}

#[derive(Deserialize, Debug)]
pub struct Block {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Deserialize, Debug)]
pub struct Goal {
    pub x: f32,
    pub y: f32,
}

pub fn read_json(path: Res<JsonFilePath>, mut commands: Commands) {
    let root = current_dir().unwrap();
    let path: std::path::PathBuf = RelativePath::new(&path.0).to_path(root);
    let data = fs::read_to_string(path.into_os_string()).expect("Unable to read file");
    let config: Config = serde_json::from_str(&data).expect("Unable to parse json");
    commands.insert_resource(config);
}
