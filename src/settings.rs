// game settings

pub const RES: (f32, f32) = (1600.0, 900.0);
pub const HALF_WIDTH: f32 = RES.0 / 2.0;
pub const HALF_HEIGHT: f32 = RES.1 / 2.0;
pub const MAP_SIZE: (usize, usize) = (16, 9);
//pub const TILE_SIZE: f32 = RES.0 / MAP_SIZE.0 as f32;
//pub const FSZ: i32 = 40;
//pub const TITLE: &'static str = "Ray casting tutorial";
//pub const FPS: u32 = 60;
//pub const DT: f32 = 1.0 / FPS as f32;
pub const BGR: [f32; 3] = [0.0, 0.0, 0.0];
//pub const FCL: [f32; 3] = [1.0, 1.0, 1.0];

pub const PI: f32 = std::f32::consts::PI;
pub const PLAYER_POS: (f32, f32) = (9.5, 5.5);
pub const PLAYER_ANGLE: f32 = 0.0;
pub const PLAYER_SPEED: f32 = 0.04;
pub const PLAYER_ROT_SPEED: f32 = 0.05;
pub const PLAYER_RADIUS: f32 = 0.1;

pub const PLAYER_FOV: f32 = PI / 3.0;
pub const HALF_FOV: f32 = PLAYER_FOV / 2.0;

pub const SCALE: f32 = 2.0;
pub const RAY_NUMBER: usize = (RES.0 / SCALE) as usize;
pub const DA: f32 = PLAYER_FOV / (RAY_NUMBER as f32);
pub const MAX_DIST: f32 = 20.0;
pub const TOL: f32 = 0.0001;
pub const TEXTURE_SIZE: f32 = 256.0;
//pub const HALF_TEXTURE: f32 = TEXTURE_SIZE / 2.0;




