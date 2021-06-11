#[derive(Copy, Clone, PartialEq)]
pub enum Tile {
    Dead,
    Alive,
}

use rand::random;
use Tile::*;

impl Tile {
    pub fn random() -> Self {
        match random::<bool>() {
            true => Alive,
            false => Dead,
        }
    }

    pub fn update(&self, neighbours: usize) -> Self {
        match neighbours {
            3 => Alive,
            2 => *self,
            _ => Dead,
        }
    }
}
