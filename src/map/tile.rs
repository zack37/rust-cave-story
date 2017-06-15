use sprite::Sprite;
use std::clone::Clone;

#[derive(Clone)]
pub enum TileType {
    Air,
    Wall,
}

#[derive(Clone)]
pub struct Tile<S: Sprite + Clone> {
    tile_type: TileType,
    sprite: S,
}

impl<S: Sprite + Clone> Tile<S> {
    pub fn new<T>(tile_type: T, sprite: S) -> Tile<S>
        where T: Into<Option<TileType>>
    {
        Tile {
            tile_type: tile_type.into().unwrap_or(TileType::Air),
            sprite: sprite,
        }
    }

    pub fn sprite(&self) -> S {
        self.sprite.clone()
    }

    pub fn tile_type(&self) -> TileType {
        self.tile_type.clone()
    }
}
