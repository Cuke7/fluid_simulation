use nannou::prelude::*;
use rand::Rng;

struct Particle {
    x: f32,
    y: f32,
}

const ARRAY_SIZE: i32 = 100;

struct Model {
    particles: Vec<Particle>,
}

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    let app_boundary = _app.window_rect();
    let (l, r, b, t) = app_boundary.l_r_b_t();

    // Create an array of random positioned particles
    let mut particles: Vec<Particle> = Vec::new();
    for _number in 1..ARRAY_SIZE {
        particles.push(Particle {
            x: rand::thread_rng().gen_range(l..=r),
            y: rand::thread_rng().gen_range(b..=t),
        })
    }
    Model { particles }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // Draw all particles
    for particle in &_model.particles {
        draw.ellipse()
            .color(STEELBLUE)
            .x_y(particle.x, particle.y)
            .radius(5.0);
    }

    draw.to_frame(app, &frame).unwrap();
}
