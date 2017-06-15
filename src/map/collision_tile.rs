use map::tile::TileType;

#[derive(Debug)]
pub struct CollisionTile {
    row: i32,
    col: i32,
    tile_type: TileType,
}

impl CollisionTile {
    pub fn new(row: i32, col: i32, tile_type: TileType) -> CollisionTile {
        CollisionTile {
            row,
            col,
            tile_type,
        }
    }

    pub fn row(&self) -> i32 {
        self.row
    }

    pub fn col(&self) -> i32 {
        self.col
    }

    pub fn tile_type(&self) -> TileType {
        self.tile_type.clone()
    }
}
