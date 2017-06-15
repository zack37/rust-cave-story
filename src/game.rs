use graphics::{Graphics, SCREEN_HEIGHT, SCREEN_WIDTH};
use input::Input;
use map::Map;
use player::Player;
use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::thread::sleep;
use time::{Duration, PreciseTime};

const FPS: i64 = 60;
pub const TILE_SIZE: u32 = 32;

pub struct Game {}

impl Game {
    pub fn new() -> Game {
        Game {}
    }

    pub fn play(&mut self) {
        // Initialize
        let sdl_context = sdl2::init().expect("Failed to create SDL Context");
        let mut event_pump = sdl_context
            .event_pump()
            .expect("Failed to create event pump");
        let mut graphics: &mut Graphics = &mut Graphics::new(sdl_context)
                                                   .expect("Failed to create graphics");
        let mut input = Input::new();
        let (width, height) = (SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
        let mut player = Player::new(graphics, width / 2, height / 2);
        let mut map = Map::create_test_map(graphics);

        // Prepare
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

            if input.was_key_pressed(Keycode::Escape) {
                break 'running;
            }

            // Player horizontal movement
            if input.is_key_held(Keycode::Left) && input.is_key_held(Keycode::Right) {
                player.stop_moving();
            } else if input.is_key_held(Keycode::Left) {
                player.start_moving_left();
            } else if input.is_key_held(Keycode::Right) {
                player.start_moving_right();
            } else {
                player.stop_moving();
            }

            if input.is_key_held(Keycode::Up) && input.is_key_held(Keycode::Down) {
                player.look_horizontal();
            } else if input.is_key_held(Keycode::Up) {
                player.look_up();
            } else if input.is_key_held(Keycode::Down) {
                player.look_down();
            } else {
                player.look_horizontal();
            }

            // Player jump
            if input.was_key_pressed(Keycode::Z) {
                player.start_jump();
            } else if input.was_key_released(Keycode::Z) {
                player.stop_jump();
            }

            // UPDATE
            let current_time = PreciseTime::now();
            let elapsed_time = last_update_time.to(current_time);
            player.update(elapsed_time, &map);
            map.update(elapsed_time);
            last_update_time = current_time;
            //

            // DRAW
            graphics.clear();
            player.draw(&mut graphics);
            map.draw(graphics);
            graphics.flip();
            //

            // grab new PreciseTime to account for update and draw time
            self.frame_limit(start_ticks.to(PreciseTime::now()));
        }
    }

    fn frame_limit(&self, elapsed_time: Duration) {
        let ms_per_frame = Duration::milliseconds(1000 / FPS);
        let sleep_duration = ms_per_frame - elapsed_time;

        if let Ok(sleep_duration) = sleep_duration.to_std() {
            sleep(sleep_duration);
        }
    }
}
