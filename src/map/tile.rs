use sprite::Sprite;
use std::clone::Clone;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TileType {
    Air,
    Wall,
}

type TileSprite = Rc<RefCell<Box<Sprite>>>;

#[derive(Clone)]
pub struct Tile {
    tile_type: TileType,
    sprite: Option<TileSprite>,
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            tile_type: TileType::Air,
            sprite: None,
        }
    }

    pub fn from_sprite(sprite: TileSprite, tile_type: TileType) -> Tile {
        Tile {
            tile_type,
            sprite: Some(sprite.clone()),
        }
    }

    pub fn sprite(&self) -> Option<TileSprite> {
        self.sprite.clone()
    }

    pub fn tile_type(&self) -> TileType {
        self.tile_type.clone()
    }
}
