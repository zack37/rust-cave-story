use game::TILE_SIZE;
use graphics::Graphics;
use sprite::{Sprite, AnimatedSprite};
use time::Duration;

const WALKING_ACCELERATION: f32 = 0.0012; // pixels/ms/ms
const MAX_SPEED_X: f32 = 0.325; // pixels/ms
const SLOWDOWN_FACTOR: f32 = 0.8;

pub struct Player<'sprite> {
    sprite: AnimatedSprite<'sprite>,
    x: i32,
    y: i32,
    acceleration_x: f32,
    velocity_x: f32,
}

impl<'sprite> Player<'sprite> {
    pub fn new(x: i32, y: i32) -> Player<'sprite> {

        Player {
            sprite: AnimatedSprite::new("content/MyChar.bmp", 0, 0, TILE_SIZE, TILE_SIZE, 15, 3),
            x: x,
            y: y,
            acceleration_x: 0.0,
            velocity_x: 0.0,
        }
    }

    pub fn start_moving_left(&mut self) {
        self.acceleration_x = -WALKING_ACCELERATION;
    }

    pub fn start_moving_right(&mut self) {
        self.acceleration_x = WALKING_ACCELERATION;
    }

    pub fn stop_moving(&mut self) {
        self.acceleration_x = 0.0;
    }

    pub fn update(&mut self, elapsed_time: Duration) {
        self.sprite.update(elapsed_time);
        let milliseconds = elapsed_time.num_milliseconds() as f32;
        self.x += (self.velocity_x * milliseconds).round() as i32;
        self.velocity_x += self.acceleration_x * milliseconds;
        if self.acceleration_x < 0.0 {
            self.velocity_x = self.velocity_x.max(-MAX_SPEED_X);
        } else if self.acceleration_x > 0.0 {
            self.velocity_x = self.velocity_x.min(MAX_SPEED_X);
        } else {
            self.velocity_x *= SLOWDOWN_FACTOR;
        }
    }

    pub fn draw(&self, graphics: &mut Graphics) {
        self.sprite.draw(graphics, self.x, self.y);
    }
}
