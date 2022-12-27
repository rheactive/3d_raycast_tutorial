use raylib::prelude::*;

use crate::raycasting::ray_cast;
use crate::settings as s;
use crate::map;
use crate::settings::TILE_SIZE;

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
    let cell_x = x as i32;
    let cell_y = y as i32;
    (cell_x, cell_y)
}

pub fn check_wall (x: f32, y: f32, map: &map::Map) -> bool {
    let mut check = false;
    let (x_cell, y_cell) = get_cell(x, y);
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

    pub fn movement (&mut self, map: &map::Map, dt: f32, rl: &RaylibHandle) {
        let sin_a = self.angle.sin();
        let cos_a = self.angle.cos();
        let mut dx: f32 = 0.0;
        let mut dy: f32 = 0.0;
        let speed = s::PLAYER_SPEED * dt;
        let speed_sin = speed * sin_a;
        let speed_cos = speed * cos_a;

        let keys: [bool; 6] = [
            rl.is_key_down(KeyboardKey::KEY_W),
            rl.is_key_down(KeyboardKey::KEY_S),
            rl.is_key_down(KeyboardKey::KEY_A),
            rl.is_key_down(KeyboardKey::KEY_D),
            rl.is_key_down(KeyboardKey::KEY_LEFT),
            rl.is_key_down(KeyboardKey::KEY_RIGHT)
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
            dx += speed_sin;
            dy += -speed_cos
        }

        if keys[3] {
            dx += -speed_sin;
            dy += speed_cos
        }

        if !check_wall(self.x + dx, self.y + dy, &map) {
            self.x += dx;
            self.y += dy;
        }

        if keys[4] {
            self.angle += - s::PLAYER_ROT_SPEED * dt
        }

        if keys[5] {
            self.angle += s::PLAYER_ROT_SPEED * dt
        }

        self.angle = round_anlge(self.angle)

    }

    pub fn update (&mut self, map: &map::Map, dt: f32, rl: &RaylibHandle) {
        self.movement(&map, dt, &rl)
        
    }

    pub fn draw (&self, screen: &mut RaylibDrawHandle, map: &map::Map) {
        let x = (self.x * TILE_SIZE as f32) as i32;
        let y = (self.y * TILE_SIZE as f32) as i32;
        let rays = ray_cast(&self, &map);
        for ray in rays {
            let dist = ray.distance;
            let a = ray.angle;
            let xe = x + (dist * a.cos() * TILE_SIZE as f32) as i32;
            let ye = y + (dist * a.sin() * TILE_SIZE as f32) as i32;
            screen.draw_line(x, y, 
                xe, ye, 
                Color::YELLOW);
            screen.draw_circle(xe, ye, 
                2.0, Color::RED)
        }
        let radius = (s::TILE_SIZE / 5) as f32;
        //let xe = x + (self.angle.cos() * s::RES.0 as f32) as i32;
        //let ye = y + (self.angle.sin() * s::RES.0 as f32) as i32;
        // screen.draw_line(x, y, 
        //     xe, ye, 
        //     Color::BLUE);
        screen.draw_circle(x, y, 
            radius, 
            Color::RED)
    }
}