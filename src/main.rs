use raylib::prelude::*;
//use raylib::core::texture;

mod settings;
mod map;
mod player;
mod raycasting;

// fn get_image (filename: &str) -> Image {
//     Image::load_image(filename).expect("Couldn't load image from file")
// }

// fn get_texture (
//     rl: &mut RaylibHandle,
//     thread: &RaylibThread,
//     image: &Image) -> Texture2D {
//     rl.load_texture_from_image(&thread, image)
//         .expect("Couldn't load texture into GPU memory")
// }

// fn load_wall_textures () -> [Image; 6] {
//         [
//             get_image("resources/textures/1.png"),
//             get_image("resources/textures/2.png"),
//             get_image("resources/textures/3.png"),
//             get_image("resources/textures/4.png"),
//             get_image("resources/textures/5.png"),
//             get_image("resources/textures/6.png"),
//         ]
//     }

   

struct Game {
    rl_handle: RaylibHandle,
    thread: RaylibThread,
    dt: f32,
    map: map::Map,
}

impl Game {
    fn init() -> Game {
        let res = settings::RES;
        let title = settings::TITLE;

        let (mut rl, thread) = raylib::init()
            .size(res.0, res.1)
            .title(format!("{}", title).as_str())
            .vsync()
            .build();

        rl.set_target_fps(settings::FPS);

        Game {
            rl_handle: rl,
            thread: thread,
            dt: settings::DT,
            map: map::Map::init()
        }
    }

    fn update(
        &mut self, 
        player: &mut player::Player
    ) {
        let mut screen = self
                                            .rl_handle
                                            .begin_drawing(&self.thread);
        screen.clear_background(settings::BGR);

        self.map.draw_3d(&player, &mut screen);
        screen.draw_rectangle(0, 0, 
            2 * settings::TILE_SIZE, settings::TILE_SIZE, 
            settings::BGR);
        let curr_fps = screen.get_fps();
        self.dt = screen.get_frame_time();
        //player.draw(&mut screen, &self.map);
        screen.draw_text(
            format!("FPS: {}", curr_fps).as_str(),
            10,
            10,
            settings::FSZ,
            settings::FCL,
        )
    }

    fn run(
        &mut self, 
        player: &mut player::Player
    ) {
        loop {
            if self.rl_handle.is_key_pressed(KeyboardKey::KEY_ESCAPE)
                || self.rl_handle.window_should_close()
            {
                break;
            }
            player.update(&self.map, self.dt, &self.rl_handle);
            self.update(player);
        }
    }

}

fn main() {
    //let images = load_wall_textures();
    let mut game = Game::init();
    let mut player = player::Player::init();

    game.run(&mut player)
}
