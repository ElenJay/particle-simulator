mod particle;

use raylib::prelude::*;

use particle::Particle;

const WINDOW_WIDTH: i32 = 1600;
const WINDOW_HEIGHT: i32 = 900;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Particle Simulator")
        .vsync()
        .build();

    let mut particles: Vec<Particle> = Vec::with_capacity(10);
    particles.push(Particle::new(WINDOW_WIDTH as f32 / 2.0, WINDOW_HEIGHT as f32 / 2.0));

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        for item in &particles {
            d.draw_circle_v(item.get_position(), 20.0, Color::WHITE);
        }
    }
}
