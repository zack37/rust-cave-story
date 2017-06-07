use sdl2::VideoSubsystem;
use sdl2::render::WindowCanvas;
use sdl2::rect::{Point, Rect};
use sdl2::surface::Surface;

const K_SCREEN_WIDTH: u32 = 640;
const K_SCREEN_HEIGHT: u32 = 480;

pub struct Graphics {
    screen: WindowCanvas,
}

impl Graphics {
    pub fn new(video_subsystem: &VideoSubsystem) -> Result<Graphics, &str> {
        match video_subsystem
                  .window("Cave Story: Rust", K_SCREEN_WIDTH, K_SCREEN_HEIGHT)
                  .position_centered()
                  .opengl()
                  .build() {
            Ok(window) => {
                match window.into_canvas().software().build() {
                    Ok(mut canvas) => {
                        canvas.clear();
                        canvas.present();
                        Ok(Graphics { screen: canvas })
                    }
                    Err(_) => Err("Failed to create canvas"),
                }
            }
            Err(_) => Err("Failed to create window"),
        }
    }

    pub fn blit_with_defaults(&mut self, surface: &Surface, source_rect: Rect, dest_rect: Rect) {
        self.blit_surface(surface, source_rect, dest_rect, 0.0, None, false, false);
    }

    pub fn blit_surface(&mut self,
                        surface: &Surface,
                        source_rect: Rect,
                        dest_rect: Rect,
                        angle: f64,
                        center: Option<Point>,
                        flip_horizontal: bool,
                        flip_vertical: bool) {
        let texture_creator = self.screen.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(surface)
            .expect("Failed to create surface from texture");
        self.screen
            .copy_ex(&texture,
                     Some(source_rect),
                     Some(dest_rect),
                     angle,
                     center,
                     flip_horizontal,
                     flip_vertical)
            .expect("Failed to copy texture");
        self.screen.present();
    }
}
