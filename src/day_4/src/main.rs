// (c) 2022 Joseph HENRY
// This code is licensed under MIT license (see LICENSE for details)

use nannou::{color::rgb_u32, lyon::lyon_tessellation::StrokeOptions, noise::NoiseFn, prelude::*};

fn main() {
    nannou::app(model)
        .size(500, 500)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    resolution: usize,
    flow_field: Vec<Vec2>,
    start_points: Vec<Vec2>,
    n_curves: usize,
    perlin: nannou::noise::Perlin,
    time: f32,
}

fn model(app: &App) -> Model {
    let win = app.window_rect();
    let resolution: usize = 70;
    let n_curves = 800;

    let mut flow_field: Vec<Vec2> = vec![Vec2::ZERO; (resolution + 1).pow(2) as usize];

    for i in 0..=resolution {
        for j in 0..=resolution {
            let y_factor = j as f32 / resolution as f32;
            let angle = y_factor * PI * 4.0 + random::<f32>() * PI / 5.0 - PI / 10.0;

            let length = random::<f32>() * 5.0;
            flow_field[(i + j * resolution) as usize] =
                pt2(f32::cos(angle) * length, f32::sin(angle) * length);
        }
    }

    let start_points = (0..=n_curves)
        .map(|_i| {
            pt2(
                random_range(30.0, win.w() - 30.0),
                random_range(30.0, win.h() - 30.0),
            )
        })
        .collect();

    Model {
        n_curves,
        start_points,
        resolution,
        flow_field,
        time: 0.0,
        perlin: nannou::noise::Perlin::new(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.time += 0.05;

    for i in 0..=model.resolution {
        for j in 0..=model.resolution {
            let index = (i + j * model.resolution) as usize;
            model.flow_field[index] = model.flow_field[index].rotate(0.01);
        }
    }
}

fn ease_in_ou_cubic(x: f32) -> f32 {
    if x < 0.5 {
        4.0 * x * x * x
    } else {
        1.0 - pow(-2.0 * x + 2.0, 3) / 2.0
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let win = app.window_rect();
    let draw = app.draw().x_y(-win.w() / 2.0, -win.h() / 2.0);

    draw.background().color(rgb_u32(0x263444));

    for i in 0..model.n_curves {
        let mut start: Vec2 = model.start_points[i].clone();
        let mut points: Vec<Vec2> = vec![start];

        for _i in 0..20 {
            let x = (start.x / model.resolution as f32).floor() as usize;
            let y = (start.y / model.resolution as f32).floor() as usize;

            if x <= model.resolution && y <= model.resolution {
                start += model.flow_field[x + y * model.resolution];
                points.push(start.clone());
            }
        }

        let noise = model.perlin.get([
            (start.x / 5.0 + model.time.cos()) as f64,
            (start.y / 5.0 + model.time.sin()) as f64,
            ease_in_ou_cubic(((model.time * 10.0).cos() + 1.0) / 2.0) as f64,
        ]);

        let mut options = StrokeOptions::DEFAULT;
        options.line_width = (1.0 - noise as f32) * 12.0 + 1.0;

        draw.path()
            .stroke()
            .stroke_opts(options)
            .points(points)
            .color(rgba8(230, 51, 42, (noise * 100.0 + 20.0) as u8));
    }

    draw.rect()
        .x_y(win.w() / 2.0, win.h() / 2.0)
        .wh(win.wh())
        .no_fill()
        .stroke_color(rgb_u32(0x263444))
        .stroke_weight(100.0);

    draw.to_frame(app, &frame).unwrap();
}
