#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum MotionType {
    Standing,
    Walking,
    Jumping,
    Falling,
}

pub const MOTION_TYPES: [MotionType; 4] = [MotionType::Standing,
                                           MotionType::Walking,
                                           MotionType::Jumping,
                                           MotionType::Falling];

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum HorizontalFacing {
    Left,
    Right,
}

pub const HORIZONTAL_FACING: [HorizontalFacing; 2] = [HorizontalFacing::Left,
                                                      HorizontalFacing::Right];

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum VerticalFacing {
    Up,
    Down,
    Horizontal,
}

pub const VERTICAL_FACING: [VerticalFacing; 3] = [VerticalFacing::Up,
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

    pub fn motion_type(&self) -> MotionType {
        self.motion_type
    }

    pub fn horizontal_facing(&self) -> HorizontalFacing {
        self.horizontal_facing
    }

    pub fn vertical_facing(&self) -> VerticalFacing {
        self.vertical_facing
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
