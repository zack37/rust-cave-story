use map::tile::TileType;

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
}
