use crate::tile::Tile;
use rayon::prelude::*;

/// A 2D Vec of Tiles, with a 1-element border of dead tiles.
pub struct Grid {
    elements: Vec<Vec<Tile>>,
    x_size: usize,
    y_size: usize,
}

impl Grid {
    pub fn new(x_size: usize, y_size: usize) -> Self {
        Grid {
            elements: vec![vec![Tile::random(); y_size + 2]; x_size + 2],
            x_size,
            y_size,
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

    pub fn get_neighbours(&self, x: usize, y: usize) -> usize {
        if !self.index_is_in_bounds(x, y) {
            panic!("Bad index in get_neighbours! ({}, {})", x, y)
        }

        self.elements[x - 1][y - 1] as usize
            + self.elements[x - 1][y] as usize
            + self.elements[x - 1][y + 1] as usize
            + self.elements[x][y + 1] as usize
            + self.elements[x][y - 1] as usize
            + self.elements[x + 1][y + 1] as usize
            + self.elements[x + 1][y] as usize
            + self.elements[x + 1][y - 1] as usize
    }

    pub fn next(&self) -> Self {
        let mut new = Grid::new(self.x_size, self.y_size);

        new.elements = new
            .elements
            .par_iter_mut()
            .enumerate()
            .map(|(x, row)| {
                row.par_iter_mut()
                    .enumerate()
                    .map(|(y, _)| match self.index_is_in_bounds(x, y) {
                        true => self.elements[x][y].update(self.get_neighbours(x, y)),
                        false => Tile::Dead,
                    })
                    .collect()
            })
            .collect();

        new
    }


    fn index_is_in_bounds(&self, x: usize, y: usize) -> bool {
        x > 0 && y > 0 && x < self.x_size && y < self.y_size
    }
}
