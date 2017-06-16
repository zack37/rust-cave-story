use game::TILE_SIZE;
use graphics::Graphics;
use sdl2::rect::Rect;
use time::Duration;

const BLACK_IS_TRANSPARENT: bool = true;

pub trait Sprite {
    fn draw(&self, graphics: &mut Graphics, x: i32, y: i32);
    fn update(&mut self, elapsed_time: Duration) {}
}

#[derive(Clone, Debug)]
pub struct StaticSprite {
    sprite_sheet_path: String,
    source_rect: Rect,
}

impl StaticSprite {
    pub fn new(graphics: &mut Graphics,
               file_path: &str,
               source_x: i32,
               source_y: i32,
               width: u32,
               height: u32)
               -> StaticSprite {

        graphics.load_image(file_path, BLACK_IS_TRANSPARENT);

        StaticSprite {
            sprite_sheet_path: String::from(file_path),
            source_rect: Rect::new(source_x, source_y, width, height),
        }
    }
}

impl Sprite for StaticSprite {
    fn draw(&self, graphics: &mut Graphics, x: i32, y: i32) {
        let destination_rect = Rect::new(x, y, self.source_rect.width(), self.source_rect.height());
        graphics.blit_surface(&self.sprite_sheet_path, self.source_rect, destination_rect);
    }
}

#[derive(Clone, Debug)]
pub struct AnimatedSprite {
    static_sprite: StaticSprite,
    frame_time: Duration,
    num_frames: u32,
    current_frame: u32,
    since_last_frame_change: Duration, // Elapsed since last frame change
}

impl AnimatedSprite {
    pub fn new(graphics: &mut Graphics,
               file_path: &'static str,
               source_x: i32,
               source_y: i32,
               width: u32,
               height: u32,
               fps: u32,
               num_frames: u32)
               -> AnimatedSprite {
        let static_sprite: StaticSprite =
            StaticSprite::new(graphics, file_path, source_x, source_y, width, height);
        AnimatedSprite {
            static_sprite,
            frame_time: Duration::milliseconds((1000 / fps) as i64),
            num_frames,
            current_frame: 0,
            since_last_frame_change: Duration::zero(),
        }
    }
}

impl Sprite for AnimatedSprite {
    fn update(&mut self, elapsed_time: Duration) {
        if self.num_frames == 1 {
            return;
        }
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
