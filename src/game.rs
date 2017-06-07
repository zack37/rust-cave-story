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
    player: Option<Player<'static>>,
}

impl Game {
    pub fn new() -> Game {
        Game { player: None }
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
        self.player =
            Some(Player::new(320, 240));

        // while running ~ 60Hz
        //   Handle input, timer callbacks.
        //   update() Move the player, move projectiles, check collisions
        //   draw() draw everything!
        'running: loop {
            //     // This loop lasts 1/60th os a second
            //     //                 1000/60ths of a ms
            input.beginNewFrame();
            let start_ticks = PreciseTime::now();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running, // immediately quit
                    Event::KeyUp { keycode: Some(key), .. } => input.keyUpEvent(key),
                    Event::KeyDown { keycode: Some(key), .. } => input.keyDownEvent(key),
                    _ => {}
                }
            }

            if input.wasKeyPressed(&Keycode::Escape) {
                break 'running;
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
        if let Some(ref mut player) = self.player {
            player.update(elapsed_time);
        }
    }

    fn draw(&self, graphics: &mut Graphics) {
        if let Some(ref player) = self.player {
            player.draw(graphics);
        }
    }
}
