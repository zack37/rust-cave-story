use graphics::Graphics;
use sprite::Sprite;
use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::thread::sleep;
use time::{Duration, PreciseTime};

const K_FPS: i64 = 60;

pub struct Game {
    sprite: Option<Sprite<'static>>
}

impl Game {
    pub fn new() -> Game {
        Game { sprite: None }
    }

    pub fn event_loop(&mut self) {
        // Initialize
        let sdl_context = sdl2::init().expect("Failed to create SDL Context");
        let video_subsystem = sdl_context.video().expect("Failed to create video subsystem");
        let mut event_pump = sdl_context.event_pump().expect("Failed to create event pump");
        let mut graphics = Graphics::new(&video_subsystem).expect("Failed to create graphics object");

        self.sprite = Some(Sprite::new("content/MyChar.bmp", 0, 0, 32, 32));

        // Prepare
        sdl_context.mouse().show_cursor(false);

        // while running ~ 60Hz
        //   Handle input, timer callbacks.
        //   update() Move the player, move projectiles, check collisions
        //   draw() draw everything!
        'running: loop {
            //     // This loop lasts 1/60th os a second
            //     //                 1000/60ths of a ms
            let start_ticks = PreciseTime::now();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                    _ => {}
                }
            }

            self.update();
            self.draw(&mut graphics);

            let elapsed_ms = start_ticks.to(PreciseTime::now());
            let fps_duration = Duration::milliseconds(1000 / K_FPS);
            let sleep_duration = fps_duration - elapsed_ms;

            if let Ok(sleep_duration) = sleep_duration.to_std() {
                sleep(sleep_duration);
            }
        }
    }

    fn update(&self) {}

    fn draw(&self, graphics: &mut Graphics) {
        if let Some(ref sprite) = self.sprite {
            sprite.draw(graphics, 320, 240);
        }
    }
}
