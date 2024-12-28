use raylib::prelude::*;

pub struct Particle {
    pos: Vector2,
    prev_pos: Vector2,
    accl: Vector2,
}

impl Particle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Vector2::new(x, y),
            prev_pos: Vector2::new(x, y),
            accl: Vector2::new(x, y),
        }
    }

    pub fn get_position(&self) -> Vector2 {
        self.pos
    }
}