use opengl_graphics::GlGraphics;
use piston::input::{Button, Key, RenderArgs};

use crate::colors::GameColors;
use crate::grid::Grid;

pub struct Game {
    gl: GlGraphics,
    grid: Grid,
    colors: GameColors,
}

impl Game {
    pub fn new(gl: GlGraphics, grid: Grid, colors: GameColors) -> Self {
        Self { gl, grid, colors }
    }


    pub fn render(&mut self, args: RenderArgs) {
        use graphics::*;

        let background = self.colors.get_background();

        self.gl.draw(args.viewport(), |c, gl| {
            clear(background, gl);
        });
    }
}
