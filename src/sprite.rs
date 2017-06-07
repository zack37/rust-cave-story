use graphics::Graphics;
use std::path::Path;
use sdl2::surface::Surface;
use sdl2::rect::Rect;

pub struct Sprite<'a> {
    sprite_sheet: Surface<'a>,
    source_rect: Rect
}

impl<'a> Sprite<'a> {
    pub fn new(file_path: &str, source_x: i32, source_y: i32, width: u32, height: u32) -> Sprite {
        let bmp = Surface::load_bmp(Path::new(file_path)).expect("Failed to load bitmap");
        Sprite { sprite_sheet: bmp, source_rect: Rect::new(source_x, source_y, width, height) }
    }

    pub fn draw(&self, graphics: &mut Graphics, x: i32, y: i32) {
        let destination_rect = Rect::new(x, y, self.source_rect.width(), self.source_rect.height());
        graphics.blit_surface(&self.sprite_sheet, self.source_rect, destination_rect, 0.0, None, false, false);
    }
}
