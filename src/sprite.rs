use game::TILE_SIZE;
use graphics::Graphics;
use std::path::Path;
use sdl2::surface::Surface;
use sdl2::rect::Rect;
use time::Duration;

pub trait Sprite {
    #[allow(unused)]
    fn update(&mut self, elapsed_time: Duration) {}
    fn draw(&self, graphics: &mut Graphics, x: i32, y: i32);
}

pub struct StaticSprite<'sprite_sheet> {
    sprite_sheet: Surface<'sprite_sheet>,
    source_rect: Rect,
}

impl<'sprite_sheet> StaticSprite<'sprite_sheet> {
    pub fn new(file_path: &str,
               source_x: i32,
               source_y: i32,
               width: u32,
               height: u32)
               -> StaticSprite {
        let bmp = Surface::load_bmp(Path::new(file_path)).expect("Failed to load bitmap");
        StaticSprite {
            sprite_sheet: bmp,
            source_rect: Rect::new(source_x, source_y, width, height),
        }
    }
}

impl<'a> Sprite for StaticSprite<'a> {
    fn draw(&self, graphics: &mut Graphics, x: i32, y: i32) {
        let destination_rect = Rect::new(x, y, self.source_rect.width(), self.source_rect.height());
        graphics.blit_with_defaults(&self.sprite_sheet, self.source_rect, destination_rect);
    }
}

pub struct AnimatedSprite<'sprite_sheet> {
    static_sprite: StaticSprite<'sprite_sheet>,
    frame_time: Duration,
    num_frames: u32,
    current_frame: u32,
    since_last_frame_change: Duration, // Elapsed since last frame change
}

impl<'sprite_sheet> AnimatedSprite<'sprite_sheet> {
    pub fn new(file_path: &str,
               source_x: i32,
               source_y: i32,
               width: u32,
               height: u32,
               fps: u32,
               num_frames: u32)
               -> AnimatedSprite {
        let static_sprite = StaticSprite::new(file_path, source_x, source_y, width, height);
        AnimatedSprite {
            static_sprite: static_sprite,
            frame_time: Duration::milliseconds((1000 / fps) as i64),
            num_frames: num_frames,
            current_frame: 0,
            since_last_frame_change: Duration::zero(),
        }
    }
}

impl<'a> Sprite for AnimatedSprite<'a> {
    fn update(&mut self, elapsed_time: Duration) {
        self.since_last_frame_change = self.since_last_frame_change + elapsed_time;
        if self.since_last_frame_change > self.frame_time {
            self.current_frame += 1;
            self.since_last_frame_change = Duration::zero();
            let current_x = self.static_sprite.source_rect.x();
            if self.current_frame < self.num_frames {
                self.static_sprite
                    .source_rect
                    .set_x(current_x + TILE_SIZE as i32);
            } else {
                self.static_sprite
                    .source_rect
                    .set_x(current_x - TILE_SIZE as i32 * (self.num_frames as i32 - 1));
                self.current_frame = 0;
            }
        }
    }

    fn draw(&self, graphics: &mut Graphics, x: i32, y: i32) {
        self.static_sprite.draw(graphics, x, y);
    }
}
