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

    for _k in 0..s::RAY_NUMBER {
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

        a += s::DA;
        
    }
    rays

}

