mod collision_tile;
pub mod tile;

use backdrop::{Backdrop, FixedBackdrop};
use game::{SCREEN_WIDTH, SCREEN_HEIGHT, TILE_SIZE};
use graphics::Graphics;
use sdl2::rect::Rect;
use sprite::{Sprite, StaticSprite};
use self::tile::*;
use self::collision_tile::*;
use std::cell::RefCell;
use std::iter::repeat;
use std::rc::Rc;
use time::Duration;

pub struct Map {
    tiles: Vec<Vec<Tile>>,
    backdrop: Option<Box<Backdrop>>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            tiles: vec![vec![]],
            backdrop: None,
        }
    }

    pub fn create_test_map(graphics: &mut Graphics) -> Map {
        let mut map = Map::new();

        map.backdrop = Some(Box::new(FixedBackdrop::new("content/bkBlue.bmp", graphics)));

        let num_rows = (SCREEN_HEIGHT / TILE_SIZE) as usize;
        let num_cols = (SCREEN_WIDTH / TILE_SIZE) as usize;

        let blank_tile = Tile::new();
        let blank_row: Vec<Tile> = repeat(blank_tile).take(num_cols).collect();
        map.tiles = repeat(blank_row.clone()).take(num_rows).collect();

        let file_path = "content/PrtCave.bmp";
        let sprite = Rc::new(RefCell::new(Box::new(StaticSprite::new(graphics,
                                                                     file_path,
                                                                     TILE_SIZE as i32,
                                                                     0,
                                                                     TILE_SIZE,
                                                                     TILE_SIZE)) as
                                          Box<Sprite>));
        let wall_tile = Tile::from_sprite(sprite, TileType::Wall);

        // floor
        for col in 0..num_cols {
            map.tiles[num_rows - 1][col] = wall_tile.clone();
        }

        // walls
        for row in 0..num_rows {
            map.tiles[row][0] = wall_tile.clone();
            map.tiles[row][num_cols - 1] = wall_tile.clone();
        }

        map.tiles[num_rows - 2][3] = wall_tile.clone();
        map.tiles[num_rows - 2][5] = wall_tile.clone();

        map.tiles[num_rows - 3][4] = wall_tile.clone();
        map.tiles[num_rows - 4][3] = wall_tile.clone();
        map.tiles[num_rows - 5][2] = wall_tile.clone();

        map
    }

    pub fn get_colliding_tiles(&self, rect: &Rect) -> Vec<CollisionTile> {
        let tile_size = TILE_SIZE as i32;
        let first_row = rect.top() / tile_size;
        let last_row = rect.bottom() / tile_size;
        let first_col = rect.left() / tile_size;
        let last_col = rect.right() / tile_size;

        let total_tiles = ((last_row - first_row + 1) * (last_col - first_col + 1)) as usize;
        let mut collision_tiles = Vec::with_capacity(total_tiles);
        for row in first_row..(last_row + 1) {
            for col in first_col..(last_col + 1) {
                collision_tiles.push(CollisionTile::new(row,
                                                        col,
                                                        self.tiles[row as usize][col as usize]
                                                            .tile_type()));
            }
        }

        collision_tiles
    }

    pub fn update(&mut self, elapsed_time: Duration) {
        for row in self.tiles.as_mut_slice() {
            for col in row.as_mut_slice() {
                if let Some(sprite) = col.sprite() {
                    sprite.borrow_mut().update(elapsed_time);
                }
            }
        }
    }

    pub fn draw(&mut self, graphics: &mut Graphics) {
        for row in 0..self.tiles.len() {
            for col in 0..self.tiles[row].len() {
                if let Some(sprite) = self.tiles[row][col].sprite() {
                    let x = col as u32 * TILE_SIZE;
                    let y = row as u32 * TILE_SIZE;
                    sprite.borrow_mut().draw(graphics, x as i32, y as i32);
                }
            }
        }
    }

    pub fn draw_background(&mut self, graphics: &mut Graphics) {
        if let Some(ref mut backdrop) = self.backdrop {
            backdrop.draw(graphics);
        }
    }
}
