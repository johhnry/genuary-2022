// (c) 2022 Joseph HENRY
// This code is licensed under MIT license (see LICENSE for details)

use nannou::{color::rgb_u32, noise::NoiseFn, prelude::*};

trait FromAngle {
    /// Returns a unit vector from an angle
    fn from_angle(angle: f32) -> Vec2 {
        Vec2::new(angle.cos(), angle.sin())
    }
}

// Implement that for the Vec2 (should be in the API)
impl FromAngle for Vec2 {}

/// Particle has position, speed and size
struct Particle {
    position: Vec2,
    velocity: Vec2,
    strength: f32,
}

impl Particle {
    /// Display the particle on the screen
    fn display(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .w_h(self.strength * 10.0, self.strength * 10.0)
            .rgba8(230, 51, 42, 20);
    }

    fn update(&mut self, win: &Rect) {
        self.position += self.velocity;

        let half_w = win.w() / 2.0;
        let half_h = win.h() / 2.0;

        // Make the particle cycle when leaving the screen
        if self.position.x > half_w {
            self.position.x = -half_w;
        }

        if self.position.x < -half_w {
            self.position.x = half_w;
        }

        if self.position.y > half_h {
            self.position.y = -half_h;
        }

        if self.position.y < -half_h {
            self.position.y = half_h;
        }
    }

    fn add_force(&mut self, force: Vec2) {
        self.velocity += force / 10.0;
        // Clamp the max velocity with the strength
        self.velocity = self.velocity.clamp_length_max(self.strength * 5.0);
    }
}

fn main() {
    nannou::app(model)
        .size(500, 500)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    perlin: nannou::noise::Perlin,
    flow_field: Vec<Vec2>,
    divisions: u32,
    particles: Vec<Particle>,
}

fn model(app: &App) -> Model {
    let win = app.window_rect();

    // The flow field resolution
    let divisions = 50;

    // Create random particles
    let particles = (0..600)
        .map(|_i| Particle {
            position: Vec2::new(
                random_range(-win.w() / 2.0, win.w() / 2.0),
                random_range(-win.h() / 2.0, win.h() / 2.0),
            ),
            velocity: Vec2::from_angle(random::<f32>() * PI * 2.0),
            strength: random(),
        })
        .collect();

    Model {
        divisions,
        perlin: nannou::noise::Perlin::new(),
        flow_field: vec![Vec2::ZERO; (divisions * divisions) as usize],
        particles,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let win = app.window_rect();

    // Compute the force field
    for i in 0..model.divisions {
        for j in 0..model.divisions {
            let noise =
                model
                    .perlin
                    .get([i as f64 * 10.0 as f64, j as f64 * 10.0, app.time as f64]);
            let index = (i + j * model.divisions) as usize;

            model.flow_field[index] = Vec2::from_angle(noise as f32 * PI * 2.0);
        }
    }

    // Update the particles
    let cell_size = win.w() / model.divisions as f32;

    // Get the flow vector at (i, j)
    let flow_at = |i: u32, j: u32| model.flow_field[(i + j * model.divisions) as usize];

    for particle in model.particles.iter_mut() {
        let grid_coords = (particle.position / cell_size).floor().as_u32();
        let mut force = flow_at(grid_coords.x, grid_coords.y) * 4.0;

        // For every neighbour cells add its force
        if grid_coords.x > 0 {
            force += flow_at(grid_coords.x - 1, grid_coords.y)
        }

        if grid_coords.x < model.divisions - 1 {
            force += flow_at(grid_coords.x + 1, grid_coords.y)
        }

        if grid_coords.y > 0 {
            force += flow_at(grid_coords.x, grid_coords.y - 1)
        }

        if grid_coords.y < model.divisions - 1 {
            force += flow_at(grid_coords.x, grid_coords.y + 1)
        }

        particle.add_force(force);
        particle.update(&win);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    // Only display background once
    if app.elapsed_frames() == 0 {
        draw.background().color(rgb_u32(0x263444));
    }

    // Display particles
    for particle in model.particles.iter() {
        particle.display(&draw);
    }

    // Blue mask
    draw.ellipse()
        .no_fill()
        .stroke_weight(280.0)
        .stroke_color(rgba8(38, 52, 68, 240))
        .w_h(600.0, 600.0);

    draw.to_frame(app, &frame).unwrap();
}
