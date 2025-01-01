mod particle;
mod utils;

use raylib::prelude::*;
use raylib::consts::MouseButton::*;

use particle::ParticalStorage;

const WINDOW_WIDTH: i32 = 1600;
const WINDOW_HEIGHT: i32 = 900;

const GRAVITY: f32 = 9.8;
const TIME_STEP: f32 = 0.05;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Particle Simulator")
        .vsync()
        .build();

    let capacity: Vec<i32> = vec![8, 8];
    let particle_dist = 50.0;
    let mut particle_storage: ParticalStorage = ParticalStorage::new(capacity[0], capacity[1]);
    for i in 0..capacity[0] {
        for j in 0..capacity[1] {
            particle_storage.add(
                i as f32 * particle_dist + WINDOW_WIDTH as f32 / 3.0, 
                j as f32 * particle_dist + WINDOW_HEIGHT as f32 / 3.0,
                j == 0,
            );
        }
    }
    particle_storage.add_constraints();

    while !rl.window_should_close() {
        let mouse_pos = rl.get_mouse_position();
        if rl.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
            particle_storage.tear_cloth(mouse_pos);
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        particle_storage.satisfy_gravity(GRAVITY, TIME_STEP);
        particle_storage.satisfy_constraints();

        particle_storage.draw_particles(&mut d, WINDOW_WIDTH, WINDOW_HEIGHT);
        particle_storage.draw_constraints(&mut d);
    }
}
