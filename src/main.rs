#![feature(step_by)]

extern crate sdl2;
extern crate time;

mod backdrop;
mod game;
mod graphics;
mod log;
mod map;
mod input;
mod player;
mod sprite;

fn main() {
    game::Game::new().play();
}
