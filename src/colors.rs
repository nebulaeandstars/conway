type Color = [f32; 4];

pub struct GameColors {
    background: Color,
    alive: Color,
    dead: Color,
}

impl GameColors {
    pub fn new(background: Color, alive: Color, dead: Color) -> GameColors {
        GameColors {
            background,
            alive,
            dead,
        }
    }

    pub fn get_background(&self) -> Color {
        self.background
    }

    pub fn get_alive_color(&self) -> Color {
        self.alive
    }

    pub fn get_dead_color(&self) -> Color {
        self.dead
    }
}
