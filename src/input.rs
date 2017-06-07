use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashMap;

pub struct Input {
    held_keys: HashMap<Keycode, bool>,
    pressed_keys: HashMap<Keycode, bool>,
    released_keys: HashMap<Keycode, bool>,
}

impl Input {
    pub fn new() -> Input {
        Input {
            held_keys: HashMap::new(),
            pressed_keys: HashMap::new(),
            released_keys: HashMap::new(),
        }
    }

    pub fn beginNewFrame(&mut self) {
        self.pressed_keys.clear();
        self.released_keys.clear();
    }

    pub fn keyDownEvent(&mut self, key: Keycode) {
        self.pressed_keys.insert(key, true);
        self.held_keys.insert(key, true);
    }

    pub fn keyUpEvent(&mut self, key: Keycode) {
        self.released_keys.insert(key, true);
        self.held_keys.insert(key, false);
    }

    pub fn wasKeyPressed(&mut self, key: &Keycode) -> bool {
        *self.pressed_keys.get(key).unwrap_or(&false)
    }

    pub fn wasKeyReleased(&mut self, key: &Keycode) -> bool {
        *self.released_keys.get(key).unwrap_or(&false)
    }

    pub fn isKeyHeld(&mut self, key: &Keycode) -> bool {
        *self.held_keys.get(key).unwrap_or(&false)
    }
}
