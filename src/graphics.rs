use game::{SCREEN_HEIGHT, SCREEN_WIDTH};
use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use std::collections::HashMap;

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

    pub fn load_image<T>(&mut self, file_path: &str, black_is_transparent: T) -> &Surface<'static>
        where T: Into<Option<bool>>
    {
        self.sprite_sheets
            .entry(String::from(file_path))
            .or_insert_with(|| {
                let mut surface = Surface::load_bmp(file_path).expect("Failed to load image");
                if black_is_transparent.into().is_some() {
                    surface
                        .set_color_key(true, Color::RGB(0, 0, 0))
                        .expect("Failed to key sprite");
                }
                surface
            })
    }

    pub fn blit_surface<S, D>(&mut self, src_id: &str, source_rect: S, dest_rect: D)
        where S: Into<Option<Rect>>,
              D: Into<Option<Rect>>
    {
        let surface = self.sprite_sheets.get_mut(src_id).unwrap();
        let texture_creator = self.screen.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(surface)
            .expect("Failed to create texture");
        self.screen
            .copy(&texture, source_rect.into(), dest_rect.into())
            .expect("Failed to copy texture");
    }

    pub fn clear(&mut self) {
        self.screen.clear();
    }

    pub fn flip(&mut self) {
        self.screen.present();
    }
}
