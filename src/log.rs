use std::fmt::{Debug, Display};

#[allow(unused)]
pub struct Logger {
    module: String,
}

#[allow(unused)]
impl Logger {
    pub fn new(module: &str) -> Logger {
        Logger { module: String::from(module) }
    }

    #[inline]
    pub fn log<T: Display>(&self, msg: T) {
        if cfg!(debug_assertions) {
            println!("{}: {}", self.module, msg);
        }
    }

    #[inline]
    pub fn debug<T: Debug>(&self, msg: T) {
        if cfg!(debug_assertions) {
            println!("{}: {:?}", self.module, msg);
        }
    }
}
