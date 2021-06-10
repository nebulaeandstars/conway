use crate::tile::Tile;

/// A 2D Vec of Tiles, with a 1-element border of dead tiles.
pub struct Grid {
    elements: Vec<Vec<Tile>>,
}

impl Grid {
    pub fn new(x_size: usize, y_size: usize) -> Self {
        Grid {
            elements: vec![vec![Tile::Dead; y_size + 2]; x_size + 2],
        }
    }

    pub fn get_full_grid_ref(&self) -> &Vec<Vec<Tile>> {
        &self.elements
    }

    pub fn get_full_grid_mut(&mut self) -> &mut Vec<Vec<Tile>> {
        &mut self.elements
    }

    pub fn get_rows_as_slice(&self) -> &[Vec<Tile>] {
        &self.elements[1..self.elements.len() - 2]
    }
}
