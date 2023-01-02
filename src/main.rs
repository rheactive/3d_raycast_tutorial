use tetra::graphics::{self, Color, Rectangle, DrawParams};
use tetra::graphics::mesh::{Vertex, VertexBuffer};
use tetra::{Context, ContextBuilder, State};
use tetra::math::Vec2;
use tetra::time::{get_fps};
use tetra::graphics::text::{Text};

//======================================
//======================================


mod settings;
use settings as s;
mod map;
use map as m;
mod player;
use player as p;
mod assets;
use assets as a;
mod raycasting;
use raycasting as r;

//======================================
//======================================


struct GameState {
    player: p::Player,
    map: m::Map,
    rays: Vec<raycasting::Ray>,
    rays_floor: Vec<r::RayFloor>,
    assets: a::Assets,
}

//======================================
//======================================

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let player = p::Player::init();
        let map = m::Map::init();
        let rays = r::ray_cast(&player, &map);
        let assets = a::Assets::load(ctx);
        let rays_floor = r::floor_cast(&player);

        Ok(GameState {
            player,
            map,
            rays,
            rays_floor,
            assets,
        })
    }
}

//======================================
//======================================

impl State for GameState {
    // Inside `impl State for GameState`:

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {

        let moved = self.player.update(&self.map, ctx);

        if moved {
            self.rays = r::ray_cast(&self.player, &self.map);
            self.rays_floor = r::floor_cast(&self.player);
        }
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, 
            Color::rgb(s::BGR[0], s::BGR[1], s::BGR[2]));

        // DRAW FLOOR

        let mut height_offset;

        let ray_num = self.rays_floor.len();

        let scale = s::SCALE;

        for k in 0..ray_num {
            height_offset = scale * k as f32;

            let vertex11 = Vertex::new(
                Vec2::new(0.0, s::HEIGHT - (height_offset + scale)),
                self.rays_floor[k].uv11,
                Color::WHITE
            );

            let vertex12 = Vertex::new(
                Vec2::new(s::WIDTH, s::HEIGHT - (height_offset + scale)),
                self.rays_floor[k].uv12,
                Color::WHITE
            );

            let vertex21 = Vertex::new(
                Vec2::new(0.0, s::HEIGHT - (height_offset)),
                self.rays_floor[k].uv21,
                Color::WHITE
            );

            let vertex22 = Vertex::new(
                Vec2::new(s::WIDTH, s::HEIGHT - (height_offset)),
                self.rays_floor[k].uv22,
                Color::WHITE
            );

            let vertices = VertexBuffer::new(ctx,
                &[
                    vertex11, vertex21,
                    vertex22, 
                    
                    vertex22, vertex12,
                    vertex11
                ]   
            )?;

            let mut mesh = vertices.into_mesh();

            mesh.set_texture(self.assets.back_textures[1].clone());

            mesh.draw(ctx, DrawParams::new());
        }

        //DRAW SKY
        let sky_scale = Vec2::new(
            s::WIDTH / self.assets.back_textures[0].width() as f32,
            s::HEIGHT / self.assets.back_textures[0].height() as f32 / 2.0,
        );

        let offset = - s::WIDTH * self.player.angle * 0.5 / s::PI;

        let position = Vec2::new(
            offset,
            0.0
        );

        let tint = Color::rgb(0.9, 0.9, 0.9);

        let params = DrawParams::new()
            .position(position)
            .scale(sky_scale)
            .color(tint);

        self.assets.back_textures[0].draw(ctx, params);

        let position = Vec2::new(
            s::WIDTH + offset,
            0.0
        );

        let params = DrawParams::new()
            .position(position)
            .scale(sky_scale)
            .color(tint);

        self.assets.back_textures[0].draw(ctx, params);
        
        
        // DRAW WALLS
        for k in 0..s::RAY_NUMBER {

            let id = self.rays[k].tile;

            let offset = self.rays[k].offset;
            
            let rect: Rectangle = Rectangle::new(
                offset * (s::TILE_TEXTURE_SIZE - scale),
                0.0,
                s::SCALE * 2.0,
                s::TILE_TEXTURE_SIZE
            );

            let position = Vec2::new(
                k as f32 * s::SCALE,
                s::HALF_HEIGHT - self.rays[k].height / 2.0
            );

            let scale = Vec2::new(
                1.0,
                self.rays[k].height / s::TILE_TEXTURE_SIZE,
            );

            let params = DrawParams::new()
                .position(position)
                .scale(scale);
            
            self.assets.wall_textures[id as usize - 1].draw_region(ctx,
                rect,
                 params);
        }

        // DRAW TEXT

        let mut fps = Text::new(
            format!("FPS: {:.1}", get_fps(ctx)), 
            self.assets.font.clone()
        );

        let mut pos = Text::new(
            format!("x: {:.1}; y: {:.1}", self.player.x, self.player.y), 
            self.assets.font.clone()
        );

        let mut angle = Text::new(
            format!("angle: {:.1}", self.player.angle * 180.0 / s::PI), 
            self.assets.font.clone()
        );

        fps.draw(ctx, Vec2::new(60.0, 35.0));
        pos.draw(ctx, Vec2::new(60.0, 75.0));
        angle.draw(ctx, Vec2::new(60.0, 115.0));

        Ok(())
    }
}

//======================================
//======================================


fn main() -> tetra::Result {
    ContextBuilder::new(
        "Ray casting experiment", 
        s::WIDTH as i32, 
        s::HEIGHT as i32
    )
        .quit_on_escape(true)
        .build()?
        .run( 
            GameState::new
        )
    
}