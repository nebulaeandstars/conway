use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::ButtonState;
use piston::input::{ButtonEvent, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

mod colors;
mod game;
mod grid;
mod tile;

use crate::colors::GameColors;
use crate::game::Game;
use crate::grid::Grid;

fn main() {
    // default settings
    const OPENGL: OpenGL = OpenGL::V3_2;
    const GRID_X: usize = 200;
    const GRID_Y: usize = 100;
    const TILE_SIZE: usize = 6;
    let colors = GameColors::new(
        [0.2, 0.2, 0.2, 1.0], // background
        [0.8, 0.8, 0.8, 1.0], // alive
        [0.2, 0.2, 0.2, 1.0], // dead
    );

    let mut window: Window = WindowSettings::new(
        "snake",
        [
            (TILE_SIZE * (GRID_X - 1)) as u32,
            (TILE_SIZE * (GRID_Y - 1)) as u32,
        ],
    )
    .graphics_api(OPENGL)
    .exit_on_esc(true)
    .resizable(false)
    .samples(0)
    .build()
    .unwrap();

    let grid = Grid::random(GRID_X, GRID_Y);
    let mut game = Game::new(GlGraphics::new(OPENGL), grid, colors, TILE_SIZE);
    let mut events = Events::new(EventSettings::new()).ups(8);

    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.render_args() {
            game.render(args);
        }

        if let Some(_) = event.update_args() {
            game.update();
        }

        if let Some(args) = event.button_args() {
            if args.state == ButtonState::Press {
                game.update();
            }
        }
    }
}
