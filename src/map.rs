use crate::settings as s;

pub const MINI_MAP: [[u16; s::MAP_SIZE.0]; s::MAP_SIZE.1] = 
    [
        [3, 4, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 4, 3],
        [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4],
        [3, 0, 0, 2, 5, 2, 2, 0, 0, 0, 2, 5, 2, 0, 0, 3],
        [1, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 5, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 2, 0, 0, 1],
        [1, 0, 0, 2, 5, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
        [4, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 4],
        [3, 4, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 4, 3]
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
    
}
