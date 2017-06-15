mod collision_tile;
mod tile;

use game::TILE_SIZE;
use graphics::{Graphics, SCREEN_WIDTH, SCREEN_HEIGHT};
use sdl2::rect::Rect;
use sprite::{AnimatedSprite, Sprite};
use self::tile::*;
use self::collision_tile::*;
use time::Duration;

pub struct Map {
    tiles: Vec<Vec<Tile<AnimatedSprite>>>,
}

impl Map {
    pub fn new() -> Map {
        Map { tiles: vec![vec![]] }
    }

    pub fn create_test_map(graphics: &mut Graphics) -> Map {
        let mut map = Map::new();

        let num_rows = (SCREEN_HEIGHT / TILE_SIZE) as usize;
        let num_cols = (SCREEN_WIDTH / TILE_SIZE) as usize;

        map.tiles = vec![Vec::with_capacity(num_cols); num_rows];

        let row = 11;
        for col in 0..num_cols {
            let sprite = AnimatedSprite::new(graphics,
                                             "content/PrtCave.bmp",
                                             TILE_SIZE as i32,
                                             0,
                                             TILE_SIZE,
                                             TILE_SIZE,
                                             1,
                                             1);
            let tile = Tile::new(TileType::Wall, sprite);
            map.tiles[row].insert(col, tile);
        }

        map
    }

    pub fn get_colliding_tiles(&self, rect: &Rect) -> Vec<CollisionTile> {
        let tile_size = TILE_SIZE as i32;
        let first_row = rect.top() / tile_size;
        let last_row = rect.bottom() / tile_size;
        let first_col = rect.left() / tile_size;
        let last_col = rect.right() / tile_size;

        (first_row..last_row)
            .flat_map(|row| {
                          (first_col..last_col).map(move |col| {
                    CollisionTile::new(row, col, self.tiles[row as usize][col as usize].tile_type())
                })
                      })
            .collect::<Vec<CollisionTile>>()
    }

    pub fn update(&mut self, elapsed_time: Duration) {
        for row in self.tiles.as_mut_slice() {
            for col in row.as_mut_slice() {
                let mut sprite = col.sprite();
                sprite.update(elapsed_time);
            }
        }
    }

    pub fn draw(&mut self, graphics: &mut Graphics) {
        for row in 0..self.tiles.len() {
            for col in 0..self.tiles[row].len() {
                let sprite = self.tiles[row][col].sprite();
                let x = col as u32 * TILE_SIZE;
                let y = row as u32 * TILE_SIZE;
                sprite.draw(graphics, x as i32, y as i32);
            }
        }
    }
}
