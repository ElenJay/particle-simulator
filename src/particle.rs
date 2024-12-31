use raylib::prelude::*;

use crate::utils::{fact, hypot};

const PARTICLE_RADIUS: f32 = 15.0;

pub struct ParticalStorage {
    particles: Vec<Particle>,
    constraints: Vec<Constraint>,
}

pub struct Particle {
    pos: Vector2,
    prev_pos: Vector2,
    accl: Vector2,
    radius: f32,
}

pub struct Constraint {
    pub p1_index: usize,
    pub p2_index: usize,
    init_length: f32,
}

impl ParticalStorage {
    pub fn new(capacity: i32) -> Self {
        Self {
            particles: Vec::with_capacity(capacity as usize),
            constraints: Vec::with_capacity((fact(capacity) / 2 / fact(capacity - 2)) as usize),
        }
    }

    pub fn add(&mut self, x: f32, y: f32) {
        self.particles.push(Particle::new(x, y));
    }

    pub fn add_constraints(&mut self) {
        for i in 0..(self.particles.len() - 1) {
            for j in (i + 1)..self.particles.len() {
                self.constraints.push(Constraint::new(i, j, &self.particles));
            }
        }
    }

    pub fn get_mut_particles(&mut self) -> &mut Vec<Particle> {
        &mut self.particles
    }

    pub fn satisfy_constraints(&mut self) {
        for item in self.constraints.iter() {
            let correction = item.get_correction(&self.particles);
            let p1_pos = self.particles[item.p1_index].get_position();
            let p2_pos = self.particles[item.p2_index].get_position();

            self.particles[item.p1_index].set_position(p1_pos + correction);
            self.particles[item.p2_index].set_position(p2_pos - correction);
        }
    }
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

    pub fn set_position(&mut self, pos: Vector2) {
        self.pos = pos;
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

impl Constraint {
    pub fn new(p1_index: usize, p2_index: usize, particles: &Vec<Particle>) -> Self {
        let delta: Vector2 = particles[p2_index].get_position() - particles[p1_index].get_position();
        Self {
            p1_index: p1_index,
            p2_index: p2_index,
            init_length: hypot(delta.x, delta.y),
        }
    }

    pub fn get_correction(&self, particles: &Vec<Particle>) -> Vector2 {
        let delta: Vector2 = particles[self.p2_index].get_position() - particles[self.p1_index].get_position();
        let curr_length: f32 = hypot(delta.x, delta.y);
        let diff: f32 = (curr_length - self.init_length) / curr_length;
        let correction: Vector2 = delta * 0.5 * diff;

        correction
    }
}