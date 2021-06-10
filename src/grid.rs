use crate::tile::Tile;
use rayon::prelude::*;

/// A 2D Vec of Tiles, with a 1-element border of dead tiles.
pub struct Grid {
    elements: Vec<Vec<Tile>>,
}

impl Grid {
    pub fn new(x_size: usize, y_size: usize) -> Self {
        Grid {
            elements: vec![vec![Tile::random(); y_size + 2]; x_size + 2],
        }
    }

    pub fn random(x_size: usize, y_size: usize) -> Self {
        let mut grid = Grid::new(x_size, y_size);

        grid.elements = grid
            .elements
            .par_iter_mut()
            .map(|row| row.par_iter_mut().map(|_| Tile::random()).collect())
            .collect();

        grid
    }

    #[allow(dead_code)]
    pub fn get_full_grid_ref(&self) -> &Vec<Vec<Tile>> {
        &self.elements
    }

    pub fn get_rows_as_slice(&self) -> &[Vec<Tile>] {
        &self.elements[1..self.elements.len() - 2]
    }
}
