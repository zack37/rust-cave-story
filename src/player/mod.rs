mod collision_info;
mod jump;
mod sprite_state;

use game::TILE_SIZE;
use graphics::Graphics;
use map::Map;
use map::tile::TileType;
use sdl2::rect::Rect;
use self::collision_info::CollisionInfo;
use self::jump::Jump;
use self::sprite_state::*;
use sprite::{Sprite, AnimatedSprite};
use std::collections::HashMap;
use time::Duration;

const WALKING_ACCELERATION: f32 = 0.0012; // pixels/ms/ms
const MAX_SPEED_X: f32 = 0.325; // pixels/ms
const SLOWDOWN_FACTOR: f32 = 0.8;

// Fall Motion
const GRAVITY: f32 = 0.0012;
const MAX_SPEED_Y: f32 = 0.325; // pixels/ms

// Jump motion
const JUMP_SPEED: f32 = 0.325; // pixels/ms

// Sprite Frames
const CHARACTER_FRAME: i32 = 0;

const WALK_FRAME: i32 = 0;
const STAND_FRAME: i32 = 0;
const JUMP_FRAME: i32 = 1;
const FALL_FRAME: i32 = 2;
const UP_FRAME_OFFSET: i32 = 3;
const DOWN_FRAME: i32 = 6;
const BACK_FRAME: i32 = 7;

// Sprite
const FILE_PATH: &str = "content/MyChar.bmp";

// Walk frames
const WALK_FPS: u32 = 15;
const NUM_WALK_FRAME: u32 = 3;

pub struct Player {
    sprites: HashMap<SpriteState, Box<Sprite>>,
    x: i32,
    y: i32,
    acceleration_x: f32,
    velocity_x: f32,
    velocity_y: f32,
    horizontal_facing: HorizontalFacing,
    vertical_facing: VerticalFacing,
    on_ground: bool,
    jump: Jump,
    collision_x: Rect,
    collision_y: Rect,
}

impl Player {
    pub fn new(graphics: &mut Graphics, x: i32, y: i32) -> Player {
        let player = Player {
            sprites: HashMap::new(),
            x,
            y,
            acceleration_x: 0.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            horizontal_facing: HorizontalFacing::Left,
            vertical_facing: VerticalFacing::Horizontal,
            on_ground: false,
            jump: Jump::new(),
            collision_x: Rect::new(6, 10, 20, 12),
            collision_y: Rect::new(10, 2, 12, 30),
        };
        player.initialize_sprites(graphics)
    }

    pub fn start_moving_left(&mut self) {
        self.acceleration_x = -WALKING_ACCELERATION;
        self.horizontal_facing = HorizontalFacing::Left;
    }

    pub fn start_moving_right(&mut self) {
        self.acceleration_x = WALKING_ACCELERATION;
        self.horizontal_facing = HorizontalFacing::Right;
    }

    pub fn stop_moving(&mut self) {
        self.acceleration_x = 0.0;
    }

    pub fn look_up(&mut self) {
        self.vertical_facing = VerticalFacing::Up;
    }

    pub fn look_down(&mut self) {
        self.vertical_facing = VerticalFacing::Down;
    }

    pub fn look_horizontal(&mut self) {
        self.vertical_facing = VerticalFacing::Horizontal;
    }

    pub fn start_jump(&mut self) {
        if self.on_ground() {
            self.jump.reset();
            self.velocity_y = -JUMP_SPEED;
        } else if self.velocity_y < 0.0 {
            self.jump.reactivate();
        }
    }

    fn on_ground(&self) -> bool {
        self.on_ground
    }

    pub fn stop_jump(&mut self) {
        self.jump.deactivate();
    }

    pub fn draw(&self, graphics: &mut Graphics) {
        self.sprites[&self.get_sprite_state()].draw(graphics, self.x, self.y);
    }

    pub fn update(&mut self, elapsed_time: Duration, map: &Map) {
        let ss = self.get_sprite_state();
        self.sprites.get_mut(&ss).unwrap().update(elapsed_time);
        self.jump.update(elapsed_time);
        let elapsed_time_ms = elapsed_time.num_milliseconds() as f32;

        self.update_x(elapsed_time_ms, map);
        self.update_y(elapsed_time_ms, map);
    }

    fn update_x(&mut self, elapsed_time_ms: f32, map: &Map) {
        // update velocity
        self.velocity_x += self.acceleration_x * elapsed_time_ms;
        if self.acceleration_x < 0.0 {
            self.velocity_x = self.velocity_x.max(-MAX_SPEED_X);
        } else if self.acceleration_x > 0.0 {
            self.velocity_x = self.velocity_x.min(MAX_SPEED_X);
        } else if self.on_ground() {
            self.velocity_x *= SLOWDOWN_FACTOR;
        }

        // calculate delta
        let delta = (self.velocity_x * elapsed_time_ms).round() as i32;

        // check collision in direction of delta
        if delta > 0 {
            // moving right
            // right side collisions
            let info = self.get_collision_info(self.right_collision(delta), map);
            if info.collided {
                println!("moving right col {}", info.col);
                self.x = info.col * TILE_SIZE as i32 - self.collision_x.right();
                self.velocity_x = 0.0;
            } else {
                self.x += delta;
            }

            // left side collisions
            let info = self.get_collision_info(self.left_collision(0), map);
            if info.collided {
                self.x = info.col * TILE_SIZE as i32 + self.collision_x.right() as i32;
            }
        } else {
            // moving left
            // left side collisions
            let info = self.get_collision_info(self.left_collision(delta), map);
            if info.collided {
                self.x = info.col * TILE_SIZE as i32 + self.collision_x.right() as i32;
                self.velocity_x = 0.0;
            } else {
                self.x += delta;
            }

            // right side collisions
            let info = self.get_collision_info(self.right_collision(0), map);
            if info.collided {
                self.x = info.col * TILE_SIZE as i32 - self.collision_x.right();
            }
        }
    }

    fn update_y(&mut self, elapsed_time_ms: f32, map: &Map) {
        // Update velocity
        if !self.jump.active() {
            self.velocity_y = (self.velocity_y + GRAVITY * elapsed_time_ms).min(MAX_SPEED_Y);
        }

        //calculate_delta
        let delta = (self.velocity_y * elapsed_time_ms).round() as i32;

        // check collision in direction of delta
        if delta > 0 {
            let info = self.get_collision_info(self.bottom_collision(delta), map);

            self.on_ground = info.collided;
            if info.collided {
                self.y = info.row * (TILE_SIZE as i32) - self.collision_y.bottom();
                self.velocity_y = 0.0;
            } else {
                self.y += delta;
            }

            let info = self.get_collision_info(self.top_collision(0), map);

            if info.collided {
                self.y = info.row * TILE_SIZE as i32 + self.collision_y.height() as i32;
            }
        } else {
            let info = self.get_collision_info(self.top_collision(delta), map);

            if info.collided {
                self.y = info.row * TILE_SIZE as i32 + self.collision_y.height() as i32;
                self.velocity_y = 0.0;
            } else {
                self.y += delta;
                self.on_ground = false;
            }

            let info = self.get_collision_info(self.bottom_collision(0), map);

            self.on_ground = info.collided;
            if info.collided {
                self.y = info.row * TILE_SIZE as i32 - self.collision_y.bottom();
            }
        }
    }

    fn get_collision_info(&self, rect: Rect, map: &Map) -> CollisionInfo {
        match map.get_colliding_tiles(&rect)
                  .iter()
                  .find(|tile| tile.tile_type() == TileType::Wall) {
            Some(ct) => CollisionInfo::new(true, ct.row(), ct.col()),
            None => CollisionInfo::new(false, 0, 0),
        }
    }

    fn get_sprite_state(&self) -> SpriteState {
        let motion_type = if self.on_ground() {
            if self.acceleration_x == 0.0 {
                MotionType::Standing
            } else {
                MotionType::Walking
            }
        } else {
            if self.velocity_y < 0.0 {
                MotionType::Jumping
            } else {
                MotionType::Falling
            }
        };
        SpriteState::new(motion_type, self.horizontal_facing, self.vertical_facing)
    }

    fn left_collision(&self, delta: i32) -> Rect {
        assert!(delta <= 0);
        Rect::new(self.x + self.collision_x.left() + delta,
                  self.y + self.collision_x.top(),
                  self.collision_x.width() / 2 + delta.abs() as u32,
                  self.collision_x.height())
    }

    fn right_collision(&self, delta: i32) -> Rect {
        assert!(delta >= 0);
        Rect::new(self.x + self.collision_x.left() + (self.collision_x.width() / 2) as i32,
                  self.y + self.collision_x.top(),
                  self.collision_x.width() / 2 + delta as u32,
                  self.collision_x.height())
    }

    fn top_collision(&self, delta: i32) -> Rect {
        assert!(delta <= 0);
        Rect::new(self.x + self.collision_y.left(),
                  self.y + self.collision_y.top() + delta,
                  self.collision_y.width(),
                  self.collision_y.height() / 2 + delta.abs() as u32)
    }

    fn bottom_collision(&self, delta: i32) -> Rect {
        assert!(delta >= 0);
        Rect::new(self.x + self.collision_y.left(),
                  self.y + self.collision_y.top() + (self.collision_y.height() / 2) as i32,
                  self.collision_y.width(),
                  self.collision_y.height() / 2 + delta as u32)
    }

    fn initialize_sprites(mut self, graphics: &mut Graphics) -> Player {
        for &motion_type in MOTION_TYPES.iter() {
            for &horizontal_facing in HORIZONTAL_FACING.iter() {
                for &vertical_facing in VERTICAL_FACING.iter() {
                    self.initialize_sprite(graphics,
                                           SpriteState::new(motion_type,
                                                            horizontal_facing,
                                                            vertical_facing));
                }
            }
        }
        self
    }

    fn initialize_sprite(&mut self, graphics: &mut Graphics, sprite_state: SpriteState) {
        let tile_size = TILE_SIZE as i32;

        let frame = match sprite_state.motion_type() {
            MotionType::Walking => WALK_FRAME,
            MotionType::Standing => STAND_FRAME,
            MotionType::Jumping => JUMP_FRAME,
            MotionType::Falling => FALL_FRAME,
        };
        let vertical_offset = match sprite_state.vertical_facing() {
            VerticalFacing::Up => UP_FRAME_OFFSET * tile_size,
            _ => 0,
        };
        let source_x = frame * tile_size + vertical_offset;

        let horizontal_offset = match sprite_state.horizontal_facing() {
            HorizontalFacing::Left => 0,
            HorizontalFacing::Right => 1,
        };
        let source_y = (CHARACTER_FRAME + horizontal_offset) * tile_size;

        let sprite = match sprite_state.motion_type() {
            MotionType::Walking => {
                AnimatedSprite::new(graphics,
                                    FILE_PATH,
                                    source_x,
                                    source_y,
                                    TILE_SIZE,
                                    TILE_SIZE,
                                    WALK_FPS,
                                    NUM_WALK_FRAME)
            }
            _ => {
                let source_x = if sprite_state.vertical_facing() == VerticalFacing::Down {
                    if sprite_state.motion_type() == MotionType::Standing {
                        BACK_FRAME * tile_size
                    } else {
                        DOWN_FRAME * tile_size
                    }
                } else {
                    source_x
                };
                // "Static" sprite
                AnimatedSprite::new(graphics,
                                    FILE_PATH,
                                    source_x,
                                    source_y,
                                    TILE_SIZE,
                                    TILE_SIZE,
                                    1,
                                    1)
            }
        };

        self.sprites.insert(sprite_state, Box::new(sprite));
    }
}
