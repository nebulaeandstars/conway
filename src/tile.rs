#[derive(Copy, Clone)]
pub enum Tile {
    Alive,
    Dead,
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
}
