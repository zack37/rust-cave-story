use game::TILE_SIZE;
use graphics::Graphics;
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
const JUMP_TIME: i64 = 275; // ms

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

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum MotionType {
    Standing,
    Walking,
    Jumping,
    Falling,
}

const MOTION_TYPES: [MotionType; 4] = [MotionType::Standing,
                                       MotionType::Walking,
                                       MotionType::Jumping,
                                       MotionType::Falling];

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum HorizontalFacing {
    Left,
    Right,
}

const HORIZONTAL_FACING: [HorizontalFacing; 2] = [HorizontalFacing::Left, HorizontalFacing::Right];

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum VerticalFacing {
    Up,
    Down,
    Horizontal,
}

const VERTICAL_FACING: [VerticalFacing; 3] = [VerticalFacing::Up,
                                              VerticalFacing::Down,
                                              VerticalFacing::Horizontal];

#[derive(Eq, Hash, PartialEq)]
pub struct SpriteState {
    motion_type: MotionType,
    horizontal_facing: HorizontalFacing,
    vertical_facing: VerticalFacing,
}

impl SpriteState {
    pub fn new<M, D, V>(motion_type: M, horizontal_facing: D, vertical_facing: V) -> SpriteState
        where M: Into<Option<MotionType>>,
              D: Into<Option<HorizontalFacing>>,
              V: Into<Option<VerticalFacing>>
    {
        SpriteState {
            motion_type: motion_type.into().unwrap_or(MotionType::Standing),
            horizontal_facing: horizontal_facing.into().unwrap_or(HorizontalFacing::Left),
            vertical_facing: vertical_facing.into().unwrap_or(VerticalFacing::Horizontal),
        }
    }

    pub fn lt(&self, rhs: &SpriteState) -> bool {
        if self.motion_type != rhs.motion_type {
            return self.motion_type < rhs.motion_type;
        }
        if self.horizontal_facing != rhs.horizontal_facing {
            return self.horizontal_facing < rhs.horizontal_facing;
        }
        if self.vertical_facing != rhs.vertical_facing {
            return self.vertical_facing < rhs.vertical_facing;
        }
        false
    }
}

struct Jump {
    time_remaining: Duration,
    active: bool,
}

impl Jump {
    pub fn new() -> Jump {
        Jump {
            time_remaining: Duration::zero(),
            active: false,
        }
    }

    pub fn reset(&mut self) {
        self.time_remaining = Duration::milliseconds(JUMP_TIME);
        self.reactivate();
    }

    pub fn reactivate(&mut self) {
        self.active = self.time_remaining > Duration::zero();
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn update(&mut self, elapsed_time: Duration) {
        if self.active {
            self.time_remaining = self.time_remaining - elapsed_time;
            if self.time_remaining <= Duration::zero() {
                self.active = false;
            }
        }
    }
}

pub struct Player {
    sprites: HashMap<SpriteState, AnimatedSprite>,
    x: i32,
    y: i32,
    acceleration_x: f32,
    velocity_x: f32,
    velocity_y: f32,
    horizontal_facing: HorizontalFacing,
    vertical_facing: VerticalFacing,
    on_ground: bool,
    jump: Jump,
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

    pub fn update(&mut self, elapsed_time: Duration) {
        let ss = self.get_sprite_state();
        self.sprites
            .get_mut(&ss)
            .expect("Failed getting default sprite")
            .update(elapsed_time);
        self.jump.update(elapsed_time);
        let elapsed_time_ms = elapsed_time.num_milliseconds() as f32;
        self.x += (self.velocity_x * elapsed_time_ms).round() as i32;
        self.velocity_x += self.acceleration_x * elapsed_time_ms;
        if self.acceleration_x < 0.0 {
            self.velocity_x = self.velocity_x.max(-MAX_SPEED_X);
        } else if self.acceleration_x > 0.0 {
            self.velocity_x = self.velocity_x.min(MAX_SPEED_X);
        } else if self.on_ground() {
            self.velocity_x *= SLOWDOWN_FACTOR;
        }

        self.y += (self.velocity_y * elapsed_time_ms).round() as i32;
        if !self.jump.active {
            self.velocity_y = (self.velocity_y + GRAVITY * elapsed_time_ms).min(MAX_SPEED_Y);
        }


        //TODO: remove this hack
        if self.y > 320 {
            self.y = 320;
            self.velocity_y = 0.0;
        }
        self.on_ground = self.y == 320;
        //TODO: remove this hack
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

        let frame = match sprite_state.motion_type {
            MotionType::Walking => WALK_FRAME,
            MotionType::Standing => STAND_FRAME,
            MotionType::Jumping => JUMP_FRAME,
            MotionType::Falling => FALL_FRAME,
        };
        let vertical_offset = match sprite_state.vertical_facing {
            VerticalFacing::Up => UP_FRAME_OFFSET * tile_size,
            _ => 0
        };
        let source_x = frame * tile_size + vertical_offset;
        
        let horizontal_offset = match sprite_state.horizontal_facing {
            HorizontalFacing::Left => 0,
            HorizontalFacing::Right => 1
        }; 
        let source_y = (CHARACTER_FRAME + horizontal_offset) * tile_size;

        let sprite = match sprite_state.motion_type {
            MotionType::Walking => AnimatedSprite::new(graphics, FILE_PATH, source_x, source_y, TILE_SIZE, TILE_SIZE, WALK_FPS, NUM_WALK_FRAME),
            _ => {
                let source_x = if sprite_state.vertical_facing == VerticalFacing::Down {
                    if sprite_state.motion_type == MotionType::Standing {
                        BACK_FRAME * tile_size
                    } else { DOWN_FRAME * tile_size }
                } else { source_x };
                // "Static" sprite
                AnimatedSprite::new(graphics, FILE_PATH, source_x, source_y, TILE_SIZE, TILE_SIZE, 1, 1)
            }
        };

        self.sprites.insert(sprite_state, sprite);
    }

    pub fn draw(&self, graphics: &mut Graphics) {
        self.sprites[&self.get_sprite_state()].draw(graphics, self.x, self.y);
    }
}
