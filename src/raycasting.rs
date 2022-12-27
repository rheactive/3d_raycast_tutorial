use crate::player::get_cell;
use crate::settings as s;
use crate::map as m;
use crate::player as p;

pub struct Ray {
    pub angle: f32,
    pub distance: f32,
    pub tile: u16
}

impl Ray {
    pub fn new (angle: f32, distance: f32, tile: u16) -> Ray {
        Ray {
            angle: angle,
            distance: distance,
            tile: tile
        }
    }
}

pub fn ray_cast (player: &p::Player, map: &m::Map) -> Vec<Ray> {
    let x = player.x;
    let y = player.y;
    let (cell_x, cell_y) = get_cell(x, y);
    let mut a = player.angle - s::HALF_FOV + s::TOL;
    let mut rays: Vec<Ray> = Vec::new();

    while a < player.angle + s::HALF_FOV {
        let cos_a = a.cos();
        let sin_a = a.sin();

        let mut ray = Ray::new(a, 0.0, 0);

        // horizontals
        let (mut y_hor, dy) = if sin_a > 0.0 {
            (cell_y as f32 + 1.0, 1.0)
        } else {
            (cell_y as f32 - s::TOL, -1.0)
        };

        let mut dist_hor = ((y_hor - y + s::TOL) / sin_a).abs();
        let mut x_hor = x + dist_hor * cos_a;

        let d_dist = (dy / sin_a).abs();
        let dx = d_dist * cos_a;

        let mut check = false;

        while dist_hor < s::MAX_DIST && !check {
            let (x_cell, y_cell) = get_cell(x_hor, y_hor);

            for tile in &map.tiles {
                if tile.1 == x_cell 
                && tile.2 == y_cell {
                    ray.tile = tile.0;
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

        let mut dist_vert = ((x_vert - x) / cos_a).abs();
        let mut y_vert = y + dist_vert * sin_a;

        let d_dist = (dx / cos_a).abs();
        let dy = d_dist * sin_a;

        let mut check = false;

        while dist_vert < s::MAX_DIST && !check {
            let (x_cell, y_cell) = get_cell(x_vert, y_vert);

            for tile in &map.tiles {
                if tile.1 == x_cell 
                && tile.2 == y_cell {
                    ray.tile = tile.0;
                    check = true
                }
            }

            if !check {
                x_vert += dx;
                y_vert += dy;
                dist_vert += d_dist
            }
            
        }

        // compare distances
        let dist: f32 = if dist_vert > dist_hor {
            dist_hor
        } else {
            dist_vert
        };

        ray.distance = dist;
        rays.push(ray);

        a = a + s::DA;
    }
    rays

}

