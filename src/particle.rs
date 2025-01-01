use raylib::prelude::*;

use crate::utils::{fact, hypot};

const PARTICLE_RADIUS: f32 = 10.0;

pub struct ParticalStorage {
    capacity_row: usize,
    capacity_col: usize,
    particles: Vec<Particle>,
    constraints: Vec<Constraint>,
}

pub struct Particle {
    pos: Vector2,
    prev_pos: Vector2,
    is_pinned: bool,
    accl: Vector2,
    radius: f32,
}

pub struct Constraint {
    pub p1_index: usize,
    pub p2_index: usize,
    init_length: f32,
}

impl ParticalStorage {
    pub fn new(capacity_row: i32, capacity_col: i32) -> Self {
        let particles_capacity: usize = (capacity_row * capacity_col) as usize;
        let constraints_capacity: usize = if capacity_row * capacity_col < 10 {
            fact(particles_capacity) / fact(particles_capacity - 2) / 2
        } else {
            (2 * capacity_row * capacity_col + 2) as usize
        };

        Self {
            capacity_row: capacity_row as usize,
            capacity_col: capacity_col as usize,
            particles: Vec::with_capacity(particles_capacity as usize),
            constraints: Vec::with_capacity(constraints_capacity as usize),
        }
    }

    pub fn add(&mut self, x: f32, y: f32, is_pinned: bool) {
        self.particles.push(Particle::new(x, y, is_pinned));
    }

    pub fn get_mut_particles(&mut self) -> &mut Vec<Particle> {
        &mut self.particles
    }

    pub fn satisfy_gravity(&mut self, gravity: f32, time_step: f32) {
        for item in self.particles.iter_mut() {
            item.apply_force(Vector2::new(0.0, gravity));
            item.update(time_step);
        }
    }

    pub fn draw_particles(&mut self, d: &mut RaylibDrawHandle, window_width: i32, window_height: i32) {
        for item in self.particles.iter_mut() {
            item.constraint_to_bounds(window_width as f32, window_height as f32);
            d.draw_circle_v(item.get_position(), item.get_radius(), Color::WHITE);
        }
    }

    pub fn add_constraints(&mut self) {
        if self.particles.len() < 10 {
            for i in 0..(self.particles.len() - 1) {
                for j in (i + 1)..self.particles.len() {
                    self.constraints.push(Constraint::new(i, j, &self.particles));
                }
            }
        } else {
            for row in 0..self.capacity_row {
                for col in 0..self.capacity_col {
                    if col < (self.capacity_col - 1) {
                        self.constraints.push(Constraint::new(row * self.capacity_col + col, row * self.capacity_col + col + 1, &self.particles));
                    }
                    if row < (self.capacity_row - 1) {
                        self.constraints.push(Constraint::new(row * self.capacity_col + col, (row + 1) * self.capacity_col + col, &self.particles));
                    }
                }
            }
            self.constraints.push(Constraint::new(0, self.capacity_row * self.capacity_col - 1, &self.particles));
            self.constraints.push(Constraint::new(self.capacity_row * (self.capacity_col - 1), self.capacity_col - 1, &self.particles));
        }
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

    pub fn draw_constraints(&self, d: &mut RaylibDrawHandle) {
        for item in self.constraints.iter() {
            let p1_pos = self.particles[item.p1_index].get_position();
            let p2_pos = self.particles[item.p2_index].get_position();
            d.draw_line_ex(p1_pos, p2_pos, 1.0, Color::WHITE);
        }
    }
}

impl Particle {
    pub fn new(x: f32, y: f32, is_pinned: bool) -> Self {
        Self {
            pos: Vector2::new(x, y),
            prev_pos: Vector2::new(x, y),
            is_pinned: is_pinned,
            accl: Vector2::new(0.0, 0.0),
            radius: PARTICLE_RADIUS,
        }
    }

    pub fn get_position(&self) -> Vector2 {
        self.pos
    }

    pub fn set_position(&mut self, pos: Vector2) {
        if !self.is_pinned {
            self.pos = pos;
        }
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn apply_force(&mut self, force: Vector2) {
        if !self.is_pinned {
            self.accl += force;
        }
    }

    pub fn update(&mut self, time_step: f32) {
        // Verlet integration
        if !self.is_pinned {
            let velocity: Vector2 = self.pos - self.prev_pos;
            self.prev_pos = self.pos;
            self.pos += velocity + self.accl * time_step * time_step;
            (self.accl.x, self.accl.y) = (0.0, 0.0);
        }
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