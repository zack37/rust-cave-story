use game::TILE_SIZE;
use graphics::Graphics;
use sprite::{Sprite,AnimatedSprite};
use time::Duration;

pub struct Player<'a> {
    sprite: AnimatedSprite<'a>,
    x: i32,
    y: i32
}

impl<'a> Player<'a> {
    pub fn new(x: i32, y: i32) -> Player<'a> {

        Player {
            sprite: AnimatedSprite::new("content/MyChar.bmp", 0, 0, TILE_SIZE, TILE_SIZE, 15, 3),
            x: x,
            y: y
        }
    }

    pub fn startMovingLeft(&mut self) {}

    pub fn startMovingRight(&mut self) {}

    pub fn stopMoving(&mut self) {}

    pub fn update(&mut self, elapsed_time: Duration) {
        self.sprite.update(elapsed_time);
    }

    pub fn draw(&self, graphics: &mut Graphics) {
        self.sprite.draw(graphics, self.x, self.y);
    }
}
