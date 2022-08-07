extern crate sdl2;

mod constants;
mod resources;
mod audio;
mod texture;
mod game;
mod systems;
mod events;
mod map;
mod components;
mod entities;

use crate::game::Loop;
use crate::texture::Image;
use crate::audio::Sound;

pub fn main() {
    let main_loop = Loop {};
    let image = Image {};
    let sound = Sound {};
    gameshell_base::run(main_loop, image, sound);
}