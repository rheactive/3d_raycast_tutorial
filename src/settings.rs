// game settings
use raylib::prelude::*;

pub const RES: (i32, i32) = (1600, 900);
pub const MAP_SIZE: (usize, usize) = (16, 9);
pub const TILE_SIZE: i32 = RES.0 / MAP_SIZE.0 as i32;
pub const FSZ: i32 = 40;
pub const TITLE: &'static str = "Ray casting tutorial";
pub const FPS: u32 = 60;
pub const DT: f32 = 1.0 / FPS as f32;
pub const BGR: Color = Color::BLACK;
pub const FCL: Color = Color::WHITE;

pub const PI: f32 = std::f32::consts::PI;
pub const PLAYER_POS: (f32, f32) = (1.5, 5.0);
pub const PLAYER_ANGLE: f32 = 0.0;
pub const PLAYER_SPEED: f32 = 2.4;
pub const PLAYER_ROT_SPEED: f32 = 3.0;

pub const PLAYER_FOV: f32 = PI / 3.0;
pub const HALF_FOV: f32 = PLAYER_FOV / 2.0;
pub const RAY_NUMBER: i32 = RES.0 / 2;
pub const DA: f32 = PLAYER_FOV / (RAY_NUMBER as f32);
pub const MAX_DIST: f32 = 20.0;
pub const TOL: f32 = 0.001;
//pub const DR: f32 = 0.01;
//pub const PLAYER_SIZE: f32 = 0.25 * TILE_SIZE as f32;

