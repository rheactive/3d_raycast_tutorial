use tetra::graphics::{self, Color, Rectangle, DrawParams};
use tetra::{Context, ContextBuilder, State};
use tetra::math::Vec2;

//======================================
//======================================


mod settings;
mod map;
mod player;
mod raycasting;
mod assets;

//======================================
//======================================


struct GameState {
    player: player::Player,
    map: map::Map,
    rays: Vec<raycasting::Ray>,
    assets: assets::Assets
}

//======================================
//======================================

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let player = player::Player::init();
        let map = map::Map::init();
        let rays = raycasting::ray_cast(&player, &map);
        let assets = assets::Assets::load(ctx);

        Ok(GameState {
            player,
            map,
            rays,
            assets,
        })
    }
}

//======================================
//======================================

impl State for GameState {
    // Inside `impl State for GameState`:

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        
        self.player.update(&self.map, ctx);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, 
            Color::rgb(settings::BGR[0], settings::BGR[1], settings::BGR[2]));

        self.rays = raycasting::ray_cast(&self.player, &self.map);

        for k in 0..settings::RAY_NUMBER {

            let offset = self.rays[k].offset;
            
            let rect: Rectangle = Rectangle::new(
                offset * (settings::TEXTURE_SIZE - settings::SCALE),
                0.0,
                settings::SCALE,
                settings::TEXTURE_SIZE
            );

            let position = Vec2::new(
                k as f32 * settings::SCALE,
                settings::HALF_HEIGHT - self.rays[k].height / 2.0
            );

            let scale = Vec2::new(
                1.0,
                self.rays[k].height / settings::TEXTURE_SIZE,
            );

            let params = DrawParams::new()
                .position(position)
                .scale(scale);
            
            self.assets.wall_textures[1].draw_region(ctx,
                rect,
                 params);
        }

        Ok(())
    }
}

//======================================
//======================================


fn main() -> tetra::Result {
    ContextBuilder::new(
        "Tetroom", 
        settings::RES.0 as i32, 
        settings::RES.1 as i32
    )
        .quit_on_escape(true)
        .build()?
        .run( 
            GameState::new
        )
    
}