use game::{SCREEN_HEIGHT, SCREEN_WIDTH};
use graphics::Graphics;
use sdl2::rect::Rect;

const BACKGROUND_SIZE: u32 = 128;

pub trait Backdrop {
    fn draw(&self, graphics: &mut Graphics);
}

pub struct FixedBackdrop {
    surface_id: String,
}

impl FixedBackdrop {
    pub fn new(path: &str, graphics: &mut Graphics) -> FixedBackdrop {
        graphics.load_image(path, None);
        FixedBackdrop { surface_id: String::from(path) }
    }
}

impl Backdrop for FixedBackdrop {
    fn draw(&self, graphics: &mut Graphics) {
        for x in (0..SCREEN_WIDTH).step_by(BACKGROUND_SIZE) {
            for y in (0..SCREEN_HEIGHT).step_by(BACKGROUND_SIZE) {
                let src = Rect::new(0, 0, BACKGROUND_SIZE, BACKGROUND_SIZE);
                let dest: Rect = Rect::new(x as i32, y as i32, BACKGROUND_SIZE, BACKGROUND_SIZE);
                graphics.blit_surface(&self.surface_id, src, dest);
            }
        }
    }
}
