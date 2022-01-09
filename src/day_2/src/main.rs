// (c) 2022 Joseph HENRY
// This code is licensed under MIT license (see LICENSE for details)

use nannou::{
    color::rgb_u32,
    noise::{NoiseFn, Perlin},
    prelude::*,
};

fn main() {
    nannou::app(model)
        .size(500, 500)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    time: f32,
    perlin: Perlin,
}

fn model(_app: &App) -> Model {
    Model {
        time: 0.0,
        perlin: nannou::noise::Perlin::new(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.time += 0.08;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    draw.background().color(rgb_u32(0x263444));

    let margin = 50.0;
    let gap = 5.0;
    let width = win.w() - 2.0 * margin;
    let start = -width / 2.0;
    let n_pixels = 40;
    let pixel_size = (width - gap * (n_pixels - 1) as f32) / n_pixels as f32;

    // Pre compute cos and sin value of time
    let cos_time = (f64::cos(model.time.into()) + 1.0) / 2.0;
    let sin_time = (f64::sin(model.time.into()) + 1.0) / 2.0;

    for i in 0..n_pixels {
        let x = start + i as f32 * (pixel_size + gap);
        for j in 0..n_pixels {
            let y = start + j as f32 * (pixel_size + gap);

            let noise = model.perlin.get([
                x as f64 / 40.0 + cos_time / 2.0,
                y as f64 / 40.0 + sin_time / 2.0,
                cos_time,
            ]) + 0.1;

            let noise_map = map_range(noise, 0.0, 1.0, 0.5, 2.0);
            let offset_pos = pt2(x, y) + pt2(0.0, 1.0).rotate(noise as f32 * PI * 2.0) * 5.0;

            // Avoid unecessary renders
            if noise > 0.05 {
                draw.rect()
                    .xy(offset_pos)
                    .w_h(pixel_size * noise_map, pixel_size * noise_map)
                    .rotate(1.0 - noise as f32 * PI)
                    .rgba8(230, 51, 42, (noise * 255.0) as u8);
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
