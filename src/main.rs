extern crate sdl2;
extern crate time;

mod game;
mod graphics;
mod sprite;

fn main() {
    game::Game::new().play();
}
