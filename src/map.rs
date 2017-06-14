use game::TILE_SIZE;
use graphics::{Graphics, SCREEN_WIDTH, SCREEN_HEIGHT};
use sprite::{AnimatedSprite, Sprite};
use time::Duration;

pub struct Map {
    foreground_sprites: Vec<Vec<AnimatedSprite>>
}

impl Map {
    pub fn new() -> Map {
        Map {
            foreground_sprites: vec![vec![]]
        }
    }

    pub fn create_test_map(graphics: &mut Graphics) -> Map {
        let mut map = Map::new();

        let num_rows = (SCREEN_HEIGHT / TILE_SIZE) as usize;
        let num_cols = (SCREEN_WIDTH / TILE_SIZE) as usize;

        map.foreground_sprites = vec![Vec::with_capacity(num_cols); num_rows];

        let row = 11;
        for col in 0..num_cols {
            let sprite = AnimatedSprite::new(graphics, "content/PrtCave.bmp", TILE_SIZE as i32, 0, TILE_SIZE, TILE_SIZE, 1, 1);
            map.foreground_sprites[row].insert(col, sprite);
        }

        map
    }

    pub fn update(&mut self, elapsed_time: Duration) {
        for row in self.foreground_sprites.as_mut_slice() {
            for col in row.as_mut_slice() {
                col.update(elapsed_time);
            }
        }
    }

    pub fn draw(&mut self, graphics: &mut Graphics) {
        for row in 0..self.foreground_sprites.len() {
            for col in 0..self.foreground_sprites[row].len() {
                let x = col as u32 * TILE_SIZE;
                let y = row as u32 * TILE_SIZE;
                self.foreground_sprites[row][col].draw(graphics, x as i32, y as i32);
            }
        }
    }
}
