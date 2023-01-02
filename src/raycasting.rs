use tetra::math::Vec2;

use crate::player::get_cell;
use crate::settings as s;
use crate::map as m;
use crate::player as p;

pub struct Ray {
    pub angle: f32,
    pub distance: f32,
    pub height: f32,
    pub tile: u16,
    pub offset: f32
}

impl Ray {
    pub fn new (angle: f32) -> Ray {
        Ray {
            angle: angle,
            distance: 0.0,
            height: 0.0,
            tile: 0,
            offset: 0.0
        }
    }
}

pub fn ray_cast (player: &p::Player, map: &m::Map) -> Vec<Ray> {
    let screen_dist: f32 = s::HALF_WIDTH / s::HALF_FOV.tan();
    let x = player.x;
    let y = player.y;
    let (cell_x, cell_y) = get_cell(x, y);
    let mut a = player.angle - s::HALF_FOV + s::TOL;
    let mut rays: Vec<Ray> = Vec::new();

    for k in 0..s::RAY_NUMBER {
        let cos_a = a.cos();
        let sin_a = a.sin();

        let mut ray = Ray::new(a);

        let mut tile_hor: u16 = 0;
        let mut tile_vert: u16 = 0;

        // horizontals
        let (mut y_hor, dy) = if sin_a > 0.0 {
            (cell_y as f32 + 1.0, 1.0)
        } else {
            (cell_y as f32 - s::TOL, -1.0)
        };

        let mut dist_hor = (y_hor - y) / sin_a;
        let mut x_hor = x + dist_hor * cos_a;

        let d_dist = dy / sin_a;
        let dx = d_dist * cos_a;

        let mut check = false;

        while dist_hor < s::MAX_DIST && !check {
            let (x_cell, y_cell) = get_cell(x_hor, y_hor);

            for tile in &map.tiles {
                if tile.1 == x_cell 
                && tile.2 == y_cell {
                    tile_hor = tile.0;
                    check = true
                }
            }

            if !check {
                x_hor += dx;
                y_hor += dy;
                dist_hor += d_dist
            }
        }

        // verticals
        let (mut x_vert, dx) = if cos_a > 0.0 {
            (cell_x as f32 + 1.0, 1.0)
        } else {
            (cell_x as f32 - s::TOL, -1.0)
        };

        let mut dist_vert = (x_vert - x) / cos_a;
        let mut y_vert = y + dist_vert * sin_a;

        let d_dist = dx / cos_a;
        let dy = d_dist * sin_a;

        let mut check = false;

        while dist_vert < s::MAX_DIST && !check {
            let (x_cell, y_cell) = get_cell(x_vert, y_vert);

            for tile in &map.tiles {
                if tile.1 == x_cell 
                && tile.2 == y_cell {
                    tile_vert = tile.0;
                    check = true
                }
            }

            if !check {
                x_vert += dx;
                y_vert += dy;
                dist_vert += d_dist
            }
            
        }

        let delta_a: f32 = player.angle - a;

        // compare distances
        if dist_vert > dist_hor {
            ray.distance = dist_hor * delta_a.cos();
            ray.tile = tile_hor;
            ray.height = screen_dist / ray.distance;
            if sin_a > 0.0 {
                ray.offset = 1.0 - f32::fract(x_hor)
            } else {
                ray.offset = f32::fract(x_hor)
            }
        } else {
            ray.distance = dist_vert * delta_a.cos();
            ray.tile = tile_vert;
            ray.height = screen_dist / ray.distance;
            if cos_a > 0.0 {
                ray.offset = f32::fract(y_vert)
            } else {
                ray.offset = 1.0 - f32::fract(y_vert)
            }
            
        };

        rays.push(ray);

        let q = s::SCALE / screen_dist;

        let da1 = q * delta_a.cos().powf(2.0);
        let da2 = da1 / (1.0 + q * (2.0 * delta_a).sin().abs());

        if k < s::RAY_NUMBER / 2 {
            a += da1
        } else {
            a += da2
        }
        
    }

    rays

}

// FLOOR

pub struct RayFloor {
    pub uv11: Vec2<f32>,
    pub uv12: Vec2<f32>,
    pub uv21: Vec2<f32>,
    pub uv22: Vec2<f32>,
    pub width: f32,
    pub height: f32
}

impl RayFloor {
    pub fn new () -> RayFloor {
        RayFloor {
            uv11: Vec2::zero(),
            uv12: Vec2::zero(),
            uv21: Vec2::zero(),
            uv22: Vec2::zero(),
            width: 1.0,
            height: 1.0
        }
    }
}


pub fn floor_cast (player: &p::Player) -> Vec<RayFloor> {
    let mut rays_floor = Vec::new();

    let screen_dist: f32 = 1.0;
    let half_width: f32 = screen_dist * s::HALF_FOV.tan();
    let half_height: f32 = half_width / s::WIDTH * s::HEIGHT;
    let scale: f32 = s::SCALE * half_height / s::HALF_HEIGHT;

    let player_pos = Vec2::new(player.x, player.y);
    let a = player.angle;
    let cos_a = a.cos();
    let sin_a = a.sin();

    let min_ray_depth = screen_dist / half_height * s::PLAYER_Z;

    let mut ray_depth = min_ray_depth;

    let mut ray_screen_height = 0f32;

    while ray_depth < s::MAX_DIST {
        let mut ray = RayFloor::new();

        ray_screen_height = ray_screen_height + scale;

        let texture_height = screen_dist 
            / (half_height - ray_screen_height) * s::PLAYER_Z - ray_depth;

        let texture_half_width: f32 = half_width / screen_dist 
            * (ray_depth + texture_height / 2.0);

        let ray_dir1 = Vec2::new(
                ray_depth * cos_a, 
                ray_depth * sin_a
            );
        let ray_dir2 = Vec2::new(
            (ray_depth + texture_height) * cos_a, 
            (ray_depth + texture_height) * sin_a
            );

        let perp_dir = Vec2::new(
            texture_half_width * sin_a, 
            - texture_half_width * cos_a
        );

        ray.uv11 = (player_pos + ray_dir2 + perp_dir) / s::FLOOR_SIZE;
        ray.uv12 = (player_pos + ray_dir2 - perp_dir) / s::FLOOR_SIZE;
        ray.uv21 = (player_pos + ray_dir1 + perp_dir) / s::FLOOR_SIZE;
        ray.uv22 = (player_pos + ray_dir1 - perp_dir) / s::FLOOR_SIZE;
        ray.width = 2.0 * texture_half_width;
        ray.height = texture_height;

        rays_floor.push(ray);

        ray_depth = ray_depth + texture_height;
    }



    rays_floor
}