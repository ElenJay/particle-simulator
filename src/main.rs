mod particle;
mod utils;

use raylib::prelude::*;

use particle::ParticalStorage;

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

    let mut particle_storage: ParticalStorage = ParticalStorage::new(10);
    particle_storage.add(WINDOW_WIDTH as f32 / 2.0 - 50.0, WINDOW_HEIGHT as f32 / 2.0 - 50.0);
    particle_storage.add(WINDOW_WIDTH as f32 / 2.0 - 50.0, WINDOW_HEIGHT as f32 / 2.0 + 50.0);
    particle_storage.add(WINDOW_WIDTH as f32 / 2.0 + 50.0, WINDOW_HEIGHT as f32 / 2.0 - 50.0);
    particle_storage.add(WINDOW_WIDTH as f32 / 2.0 + 50.0, WINDOW_HEIGHT as f32 / 2.0 + 50.0);
    particle_storage.add_constraints();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        for item in particle_storage.get_mut_particles() {
            item.apply_force(Vector2::new(0.0, GRAVITY));
            item.update(TIME_STEP);
            item.constraint_to_bounds(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);
            d.draw_circle_v(item.get_position(), item.get_radius(), Color::WHITE);
        }

        particle_storage.satisfy_constraints();
    }
}
