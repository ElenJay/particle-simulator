mod particle;

use raylib::prelude::*;

use particle::Particle;

const WINDOW_WIDTH: i32 = 1600;
const WINDOW_HEIGHT: i32 = 900;

const GRAVITY: f32 = 9.8;
const TIME_STEP: f32 = 0.1;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Particle Simulator")
        .vsync()
        .build();

    let mut particles: Vec<Particle> = Vec::with_capacity(10);
    particles.push(Particle::new(WINDOW_WIDTH as f32 / 2.0 - 50.0, WINDOW_HEIGHT as f32 / 2.0 - 50.0));
    particles.push(Particle::new(WINDOW_WIDTH as f32 / 2.0 - 50.0, WINDOW_HEIGHT as f32 / 2.0 + 50.0));
    particles.push(Particle::new(WINDOW_WIDTH as f32 / 2.0 + 50.0, WINDOW_HEIGHT as f32 / 2.0 - 50.0));
    particles.push(Particle::new(WINDOW_WIDTH as f32 / 2.0 + 50.0, WINDOW_HEIGHT as f32 / 2.0 + 50.0));

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        for item in &mut particles {
            item.apply_force(Vector2::new(0.0, GRAVITY));
            item.update(TIME_STEP);
            item.constraint_to_bounds(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);
            d.draw_circle_v(item.get_position(), item.get_radius(), Color::WHITE);
        }
    }
}
