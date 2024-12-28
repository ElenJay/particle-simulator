use raylib::prelude::*;

const PARTICLE_RADIUS: f32 = 15.0;

pub struct Particle {
    pos: Vector2,
    prev_pos: Vector2,
    accl: Vector2,
    radius: f32,
}

impl Particle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Vector2::new(x, y),
            prev_pos: Vector2::new(x, y),
            accl: Vector2::new(0.0, 0.0),
            radius: PARTICLE_RADIUS,
        }
    }

    pub fn get_position(&self) -> Vector2 {
        self.pos
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn apply_force(&mut self, force: Vector2) {
        self.accl += force;
    }

    pub fn update(&mut self, time_step: f32) {
        // Verlet integration
        let velocity: Vector2 = self.pos - self.prev_pos;
        self.prev_pos = self.pos;
        self.pos += velocity + self.accl * time_step * time_step;
        (self.accl.x, self.accl.y) = (0.0, 0.0);
    }

    pub fn constraint_to_bounds(&mut self, width: f32, height: f32) {
        if self.pos.x < self.radius { self.pos.x = self.radius }
        if self.pos.x > width - self.radius { self.pos.x = width - self.radius }
        if self.pos.y < self.radius { self.pos.y = self.radius }
        if self.pos.y > height - self.radius { self.pos.y = height - self.radius }
    }
}