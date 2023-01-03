// game settings

pub const WIDTH: f32 = 2400.0;
pub const HEIGHT: f32 = 1350.0;
pub const HALF_WIDTH: f32 = WIDTH / 2.0;
pub const HALF_HEIGHT: f32 = HEIGHT / 2.0;
pub const MAP_WIDTH: usize = 16;
pub const MAP_HEIGHT: usize = 16;
//pub const BGR: [f32; 3] = [131.0 / 255.0, 84.0 / 255.0, 58.0 / 255.0];
pub const BGR: [f32; 3] = [0.6, 0.6, 0.6];

pub const PI: f32 = std::f32::consts::PI;
pub const PLAYER_POS: (f32, f32) = (9.5, 5.5);
pub const PLAYER_ANGLE: f32 = 0.0;
pub const PLAYER_SPEED: f32 = 0.08;
pub const PLAYER_ROT_SPEED: f32 = 0.05;
pub const PLAYER_RADIUS: f32 = 0.25;

pub const PLAYER_FOV: f32 = PI / 2.5;
pub const HALF_FOV: f32 = PLAYER_FOV / 2.0;

pub const SCALE: f32 = 1.0;
pub const RAY_NUMBER: usize = (WIDTH / SCALE) as usize;
// pub const DA: f32 = PLAYER_FOV / (RAY_NUMBER as f32);
pub const MAX_DIST: f32 = 20.0;
pub const TOL: f32 = 0.0001;
pub const TILE_TEXTURE_SIZE: f32 = 64.0;

pub const FLOOR_SIZE: f32 = MAP_WIDTH as f32;
pub const PLAYER_Z: f32 = 0.5;






