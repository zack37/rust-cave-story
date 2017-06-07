extern crate sdl2;
extern crate time;

mod game;
mod graphics;
mod input;
mod player;
mod sprite;

fn main() {
    game::Game::new().play();
}
