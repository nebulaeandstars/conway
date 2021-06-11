use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

use crate::colors::GameColors;
use crate::grid::Grid;
use crate::tile::Tile::{Alive, Dead};

pub struct Game {
    gl: GlGraphics,
    grid: Grid,
    colors: GameColors,
    tile_size: usize,
}

impl Game {
    pub fn new(gl: GlGraphics, grid: Grid, colors: GameColors, tile_size: usize) -> Self {
        Self {
            gl,
            grid,
            colors,
            tile_size,
        }
    }


    pub fn render(&mut self, args: RenderArgs) {
        use graphics::*;

        let colors = &self.colors;
        let tile_size = &self.tile_size;
        let grid_rows = self.grid.get_rows_as_slice();

        self.gl.draw(args.viewport(), |c, gl| {
            clear(colors.get_background(), gl);

            let transform = c.transform.trans(0.0, 0.0);

            // for each tile in the grid, draw a rectangle
            for (x, row) in grid_rows.iter().enumerate() {
                for (y, tile) in row[1..row.len() - 2].iter().enumerate() {
                    let color = match tile {
                        Alive => colors.get_alive_color(),
                        Dead => colors.get_dead_color(),
                    };

                    let render = rectangle::square(
                        (x * tile_size) as f64,
                        (y * tile_size) as f64,
                        *tile_size as f64,
                    );

                    rectangle(color, render, transform, gl);
                }
            }
        });
    }


    pub fn update(&mut self) {
        self.grid = self.grid.next();
    }
}
