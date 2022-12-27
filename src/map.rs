use raylib::prelude::*;

use crate::settings;

pub const MINI_MAP: [[u16; settings::MAP_SIZE.0]; settings::MAP_SIZE.1] = 
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
        for x in 0..settings::MAP_SIZE.0 {
            for y in 0..settings::MAP_SIZE.1 {
                if MINI_MAP[y][x] > 0 {
                    tl.push((MINI_MAP[y][x], x as i32, y as i32))
                }
            }
        }
        Map {
            tiles: tl
        }
    }

    pub fn draw (&self, screen: &mut RaylibDrawHandle) {
        for tile in &self.tiles {
            let x = tile.1 * settings::TILE_SIZE;
            let y = tile.2 * settings::TILE_SIZE;
            screen.draw_rectangle_lines(
                x, y, 
                settings::TILE_SIZE, settings::TILE_SIZE,
                Color::WHITE
            )

        }
    }
}
