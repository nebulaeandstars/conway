#[derive(Copy, Clone, Debug)]
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
        match self {
            _ if neighbours == 3 => Alive,
            Alive if neighbours == 2 => Alive,
            _ => Dead,
        }
    }
}
