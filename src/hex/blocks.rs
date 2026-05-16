use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct ColoredBlock {
    pub start: usize,
    pub end: usize,
    pub bg_color: u32,
    pub fg_color: u32,
}

fn get_random_color() -> u32 {
    let mut rng = rand::rng();

    rng.random::<u32>()
}

impl ColoredBlock {
    pub fn new(start: usize, end: usize) -> Self {
        ColoredBlock {
            start,
            end,
            bg_color: get_random_color(),
            fg_color: get_random_color(),
        }
    }

    pub fn set_random_color(&mut self) {
        self.bg_color = get_random_color();
        self.fg_color = get_random_color();
    }
}
