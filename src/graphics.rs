use sdl2::Sdl;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use std::collections::HashMap;

pub const SCREEN_WIDTH: u32 = 640;
pub const SCREEN_HEIGHT: u32 = 480;

pub struct Graphics {
    screen: WindowCanvas,
    sprite_sheets: HashMap<String, Surface<'static>>,
}

impl Graphics {
    pub fn new(sdl_context: Sdl) -> Result<Graphics, String> {
        sdl_context.mouse().show_cursor(false);
        sdl_context
            .video()
            .map_err(|e| e.to_string())
            .and_then(|video_subsystem| {
                video_subsystem
                    .window("Cave Story: Rust", SCREEN_WIDTH, SCREEN_HEIGHT)
                    .position_centered()
                    .opengl()
                    .build()
                    .map_err(|e| e.to_string())
                    .and_then(|window| {
                        window
                            .into_canvas()
                            .software()
                            .build()
                            .map_err(|e| e.to_string())
                            .and_then(|mut canvas| {
                                          canvas.clear();
                                          canvas.present();
                                          Ok(Graphics {
                                                 screen: canvas,
                                                 sprite_sheets: HashMap::new(),
                                             })
                                      })
                    })
            })
    }

    pub fn load_image(&mut self, file_path: &str) -> &Surface<'static> {
        self.sprite_sheets
            .entry(String::from(file_path))
            .or_insert_with(|| Surface::load_bmp(file_path).expect("Failed to load image"))
    }

    pub fn blit_surface(&mut self, src_id: &str, source_rect: Rect, dest_rect: Rect) {
        let surface = self.sprite_sheets.get_mut(src_id).unwrap();
        let texture_creator = self.screen.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(surface)
            .expect("Failed to create texture");
        self.screen
            .copy(&texture, Some(source_rect), Some(dest_rect))
            .expect("Failed to copy texture");
    }

    pub fn clear(&mut self) {
        self.screen.clear();
    }

    pub fn flip(&mut self) {
        self.screen.present();
    }
}
