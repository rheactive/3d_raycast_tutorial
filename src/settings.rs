// game settings
use raylib::prelude::*;

pub const RES: (i32, i32) = (1600, 900);
pub const HALF_WIDTH: i32 = RES.0 / 2;
pub const HALF_HEIGHT: i32 = RES.1 / 2;
pub const MAP_SIZE: (usize, usize) = (16, 9);
pub const TILE_SIZE: i32 = RES.0 / MAP_SIZE.0 as i32;
pub const FSZ: i32 = 40;
pub const TITLE: &'static str = "Ray casting tutorial";
pub const FPS: u32 = 60;
pub const DT: f32 = 1.0 / FPS as f32;
pub const BGR: Color = Color::BLACK;
pub const FCL: Color = Color::WHITE;

pub const PI: f32 = std::f32::consts::PI;
pub const PLAYER_POS: (f32, f32) = (9.5, 5.5);
pub const PLAYER_ANGLE: f32 = 0.0;
pub const PLAYER_SPEED: f32 = 2.4;
pub const PLAYER_ROT_SPEED: f32 = 1.5;
pub const PLAYER_RADIUS: f32 = 0.1;

pub const PLAYER_FOV: f32 = PI / 3.0;
pub const HALF_FOV: f32 = PLAYER_FOV / 2.0;
pub const RAY_NUMBER: usize = (RES.0 / 2) as usize;
pub const DA: f32 = PLAYER_FOV / (RAY_NUMBER as f32);
pub const MAX_DIST: f32 = 20.0;
pub const TOL: f32 = 0.001;

pub const SCALE: i32 = RES.0 / RAY_NUMBER as i32;
//pub const TEXTURE_SIZE: i32 = 256;
//pub const HALF_TEXTUE: i32 = TEXTURE_SIZE / 2;




