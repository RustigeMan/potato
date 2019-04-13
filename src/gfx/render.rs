use crate::threads::gfx::GfxMsg;

use sfml::graphics::{Color, RenderTarget, RenderWindow, Transformable};
use sfml::graphics::{Sprite, Texture};

use std::collections::HashMap;

pub struct Renderer {
    win: RenderWindow,
    textures: HashMap<u32, Texture>,
}

impl Renderer {
    pub fn new(win: RenderWindow) -> Self {
        Renderer {
            win: win,
            textures: HashMap::new(),
        }
    }

    pub fn process_message(&mut self, message: GfxMsg) {
        use GfxMsg::*;
        match message {
            Clear => self.win.clear(&Color::BLACK),

            Display => self.win.display(),

            LoadImg(path, id) => {
                let texture = Texture::from_file(&path)
                    .expect(&format!("Could not load texture at: {}", &path));
                self.textures.insert(id.id(), texture);
            }

            DrawImg(img, x, y) => {
                let texture = self
                    .textures
                    .get(&img.id())
                    .expect(&format!("Could not get texture with ID: {}", &img.id()));

                let mut sprite = Sprite::with_texture(&texture);
                sprite.set_position((x, y));

                self.win.draw(&sprite);
            }

            Exit => return,
        }
    }

    pub fn win(&mut self) -> &mut RenderWindow {
        &mut self.win
    }
}
