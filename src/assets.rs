use tetra::graphics::{Texture};
use tetra::{Context};
use tetra::graphics::text::{Font, VectorFontBuilder};

pub struct Assets {
    pub wall_textures: Vec<Texture>,
    pub font: Font,
    pub back_textures: Vec<Texture>
}

impl Assets {
    pub fn load (ctx: &mut Context) -> Assets {
        let font = 
        VectorFontBuilder::new("./resources/brohoney.ttf").unwrap()
        .with_size(ctx, 40.0).unwrap();

        let wall_textures = vec![
            Texture::new(ctx, "./resources/textures/1.png").unwrap(),
            Texture::new(ctx, "./resources/textures/2.png").unwrap(),
            Texture::new(ctx, "./resources/textures/3.png").unwrap(),
        ];

        let back_textures = vec![
            Texture::new(ctx, "./resources/textures/sky.png").unwrap(),
            Texture::new(ctx, "./resources/textures/floor_huge.png").unwrap(),
        ];

        Assets {
            wall_textures,
            font,
            back_textures
        }
    }
}