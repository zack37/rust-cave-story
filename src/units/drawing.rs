use std::ops::{Add, Sub, Mul, Div};

const TILE_SIZE: i32 = 32;
const SCALE: f64 = 1.0;

pub trait AsGame {
    fn to_game(&self) -> Game;
}
pub trait AsTile {
    fn to_tile(&self) -> Tile;
}
pub trait AsPixel {
    fn to_pixel(&self) -> Pixel;
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Game(pub f64);

impl AsGame for Game {
    #[inline]
    fn to_game(&self) -> Game {
        *self
    }
}

impl AsTile for Game {
    #[inline]
    fn to_tile(&self) -> Tile {
        let Game(a) = *self;
        Tile((a / TILE_SIZE as f64) as usize)
    }
}

impl AsPixel for Game {
    #[inline]
    fn to_pixel(&self) -> Pixel {
        let Game(a) = *self;
        Pixel((a / SCALE).round() as i32)
    }
}

impl<T: AsGame> Add<T> for Game {
    type Output = Game;

    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        let (Game(a), Game(b)) = (self, rhs.to_game());
        Game(a + b)
    }
}

impl<T: AsGame> Sub<T> for Game {
    type Output = Game;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        let (Game(a), Game(b)) = (self, rhs.to_game());
        Game(a - b)
    }
}

impl<T: AsGame> Mul<T> for Game {
    type Output = Game;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        let (Game(a), Game(b)) = (self, rhs.to_game());
        Game(a * b)
    }
}

impl<T: AsGame> Div<T> for Game {
    type Output = Game;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        let (Game(a), Game(b)) = (self, rhs.to_game());
        Game(a / b)
    }
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct Pixel(pub i32);

impl AsPixel for Pixel {
    #[inline]
    fn to_pixel(&self) -> Pixel {
        *self
    }
}

impl<T: AsPixel> Add<T> for Pixel {
    type Output = Pixel;

    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        let (Pixel(a), Pixel(b)) = (self, rhs.to_pixel());
        Pixel(a + b)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HalfTile(pub u64);

impl AsGame for HalfTile {
    #[inline]
    fn to_game(&self) -> Game {
        let HalfTile(a) = *self;
        Game((a * (TILE_SIZE as u64 / 2)) as f64)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tile(pub usize);

impl AsGame for Tile {
    #[inline]
    fn to_game(&self) -> Game {
        let Tile(a) = *self;
        Game((a * TILE_SIZE as usize) as f64)
    }
}

impl AsTile for Tile {
    #[inline]
    fn to_tile(&self) -> Tile {
        *self
    }
}

impl AsPixel for Tile {
    #[inline]
    fn to_pixel(&self) -> Pixel {
        self.to_game().to_pixel()
    }
}

impl<T: AsTile> Add<T> for Tile {
    type Output = Tile;

    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        let (Tile(a), Tile(b)) = (self, rhs.to_tile());
        Tile(a + b)
    }
}

impl<T: AsTile> Sub<T> for Tile {
    type Output = Tile;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        let (Tile(a), Tile(b)) = (self, rhs.to_tile());
        Tile(a - b)
    }
}

impl<T: AsTile> Mul<T> for Tile {
    type Output = Tile;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        let (Tile(a), Tile(b)) = (self, rhs.to_tile());
        Tile(a * b)
    }
}

impl<T: AsTile> Div<T> for Tile {
    type Output = Tile;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        let (Tile(a), Tile(b)) = (self, rhs.to_tile());
        Tile(a / b)
    }
}
