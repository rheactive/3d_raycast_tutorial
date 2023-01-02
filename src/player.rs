use tetra::{Context};
use tetra::input::{self, Key};

use crate::settings::{self as s};
use crate::map;

fn round_anlge (a: f32) -> f32 {
    let mut b: f32 = a;
    if a >  2.0 * s::PI {
        b = a - 2.0 * s::PI;
    }
    if a < 0.0 {
        b = a + 2.0 * s::PI;
    }
    b
}

pub fn get_cell (x: f32, y: f32) -> (i32, i32) {
    let cell_x = f32::trunc(x) as i32;
    let cell_y = f32::trunc(y) as i32;
    (cell_x, cell_y)
}

pub fn check_wall (x: f32, y: f32, map: &map::Map) -> bool {
    let mut check = false;
    let (x_cell, y_cell) = 
        get_cell(x + s::PLAYER_RADIUS, y + s::PLAYER_RADIUS);
    for tile in &map.tiles {
        if tile.1 == x_cell 
        && tile.2 == y_cell {
            check = true
        }
    }
    let (x_cell, y_cell) = 
        get_cell(x + s::PLAYER_RADIUS, y - s::PLAYER_RADIUS);
    for tile in &map.tiles {
        if tile.1 == x_cell 
        && tile.2 == y_cell {
            check = true
        }
    }
    let (x_cell, y_cell) = 
        get_cell(x - s::PLAYER_RADIUS, y + s::PLAYER_RADIUS);
    for tile in &map.tiles {
        if tile.1 == x_cell 
        && tile.2 == y_cell {
            check = true
        }
    }
    let (x_cell, y_cell) = 
        get_cell(x - s::PLAYER_RADIUS, y - s::PLAYER_RADIUS);
    for tile in &map.tiles {
        if tile.1 == x_cell 
        && tile.2 == y_cell {
            check = true
        }
    }
    check
}

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32
}

impl Player {
    pub fn init () -> Player {
        Player {
            x: s::PLAYER_POS.0,
            y: s::PLAYER_POS.1,
            angle: s::PLAYER_ANGLE
        }
    }

    pub fn movement (&mut self, map: &map::Map, ctx: &Context) -> bool {
        let mut moved = false;

        let sin_a = self.angle.sin();
        let cos_a = self.angle.cos();
        let mut dx: f32 = 0.0;
        let mut dy: f32 = 0.0;
        let speed = s::PLAYER_SPEED;
        let speed_sin = speed * sin_a;
        let speed_cos = speed * cos_a;

        let keys: [bool; 4] = [
            input::is_key_down(ctx, Key::W),
            input::is_key_down(ctx, Key::S),
            input::is_key_down(ctx, Key::A),
            input::is_key_down(ctx, Key::D)
        ];

        if keys[0] {
            dx += speed_cos;
            dy += speed_sin
        }

        if keys[1] {
            dx += -speed_cos;
            dy += -speed_sin
        }

        if keys[2] {
            self.angle += - s::PLAYER_ROT_SPEED
        }

        if keys[3] {
            self.angle += s::PLAYER_ROT_SPEED
        }

        if keys != [false; 4] {
            moved = true;
            if !check_wall(self.x + dx, self.y + dy, &map) {
                self.x += dx;
                self.y += dy;
            } else {
                if !check_wall(self.x + dx, self.y, &map) {
                    self.x += dx;
                } else {
                    if !check_wall(self.x, self.y + dy, &map) {
                        self.y += dy;
                    }
                }
            }
        }


       self.angle = round_anlge(self.angle);

        moved

    }

    pub fn update (&mut self, map: &map::Map, ctx: &Context) -> bool {
        self.movement(&map, &ctx)
        
    }

}