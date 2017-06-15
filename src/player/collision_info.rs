pub struct CollisionInfo {
    pub collided: bool,
    pub row: i32,
    pub col: i32,
}

impl CollisionInfo {
    pub fn new(collided: bool, row: i32, col: i32) -> CollisionInfo {
        CollisionInfo { collided, row, col }
    }
}
