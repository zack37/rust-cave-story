use graphics::Graphics;
use input::Input;
use player::Player;
use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::thread::sleep;
use time::{Duration, PreciseTime};

const K_FPS: i64 = 60;
pub const TILE_SIZE: u32 = 32;

pub struct Game {
    player: Player<'static>
}

impl Game {
    pub fn new() -> Game {
        Game { player: Player::new(320, 240) }
    }

    pub fn play(&mut self) {
        // Initialize
        let sdl_context = sdl2::init().expect("Failed to create SDL Context");
        let video_subsystem = sdl_context
            .video()
            .expect("Failed to create video subsystem");
        let mut event_pump = sdl_context
            .event_pump()
            .expect("Failed to create event pump");
        let mut graphics = Graphics::new(&video_subsystem)
            .expect("Failed to create graphics object");
        let mut input = Input::new();

        // Prepare
        sdl_context.mouse().show_cursor(false);
        let mut last_update_time = PreciseTime::now();

        // while running ~ 60Hz
        //   Handle input, timer callbacks.
        //   update() Move the player, move projectiles, check collisions
        //   draw() draw everything!
        'running: loop {
            //     // This loop lasts 1/60th os a second
            //     //                 1000/60ths of a ms
            input.begin_new_frame();
            let start_ticks = PreciseTime::now();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running, // immediately quit
                    Event::KeyUp { keycode: Some(key), .. } => input.key_up_event(key),
                    Event::KeyDown { keycode: Some(key), .. } => input.key_down_event(key),
                    _ => {}
                }
            }

            if input.was_key_pressed(&Keycode::Escape) {
                break 'running;
            }

            if input.is_key_held(&Keycode::Left) && input.is_key_held(&Keycode::Right) {
                self.player.stop_moving();
            } else if input.is_key_held(&Keycode::Left) {
                self.player.start_moving_left();
            } else if input.is_key_held(&Keycode::Right) {
                self.player.start_moving_right();
            } else {
                self.player.stop_moving();
            }

            let current_time = PreciseTime::now();
            self.update(last_update_time.to(current_time));
            last_update_time = current_time;

            self.draw(&mut graphics);

            // grab new PreciseTime to account for update and draw time
            self.frame_limit(start_ticks.to(PreciseTime::now()));
        }
    }

    fn frame_limit(&self, elapsed_time: Duration) {
        let sleep_duration = Duration::milliseconds(1000 / K_FPS) - elapsed_time;

        if let Ok(sleep_duration) = sleep_duration.to_std() {
            sleep(sleep_duration);
        }
    }

    fn update(&mut self, elapsed_time: Duration) {
        self.player.update(elapsed_time);
    }

    fn draw(&self, graphics: &mut Graphics) {
        graphics.clear();
        self.player.draw(graphics);
        graphics.flip();
    }
}
