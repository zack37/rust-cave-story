#![feature(step_by)]

extern crate sdl2;
extern crate time;

mod backdrop;
mod game;
mod graphics;
mod input;
mod log;
mod map;
mod player;
mod sprite;
mod units;

fn main() {
    game::Game::new().play();
}
