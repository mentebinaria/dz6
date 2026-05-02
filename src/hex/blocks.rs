use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ColoredBlock {
    pub start: usize,
    pub end: usize,
    pub color: u32,
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
            color: get_random_color(),
        }
    }

    pub fn set_random_color(&mut self) {
        self.color = get_random_color();
    }
}
