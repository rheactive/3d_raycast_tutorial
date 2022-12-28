use raylib::prelude::*;

use crate::settings as s;
use crate::player as p;
use crate::raycasting as r;

pub const MINI_MAP: [[u16; s::MAP_SIZE.0]; s::MAP_SIZE.1] = 
    [
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1],
        [1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
    ];

pub struct Map {
    pub tiles: Vec<(u16, i32, i32)>
}

impl Map {
    pub fn init () -> Map {
        let mut tl: Vec<(u16, i32, i32)> = Vec::new();
        for x in 0..s::MAP_SIZE.0 {
            for y in 0..s::MAP_SIZE.1 {
                if MINI_MAP[y][x] > 0 {
                    tl.push((MINI_MAP[y][x], x as i32, y as i32))
                }
            }
        }
        Map {
            tiles: tl
        }
    }

    // pub fn draw_2d (&self, screen: &mut RaylibDrawHandle) {
    //     for tile in &self.tiles {
    //         let x = tile.1 * s::TILE_SIZE;
    //         let y = tile.2 * s::TILE_SIZE;
    //         screen.draw_rectangle_lines(
    //             x, y, 
    //             s::TILE_SIZE, s::TILE_SIZE,
    //             Color::WHITE
    //         )

    //     }
    // }

    pub fn draw_3d (&self, player: &p::Player, screen: &mut RaylibDrawHandle) {
        let rays = r::ray_cast(player, &self);
        for k in 0..s::RAY_NUMBER {
            let col_val = 1.0 / 
                (1.0 + 0.00002 * rays[k].distance.powf(5.0)).powf(3.0);
            screen.draw_rectangle(k as i32 * s::SCALE, 
                s::HALF_HEIGHT - rays[k].height as i32 / 2, 
                s::SCALE, 
                rays[k].height as i32, 
                Color::color_from_hsv(0.7, 0.0, col_val))
        }

    }
    
}
