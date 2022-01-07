// (c) 2022 Joseph HENRY
// This code is licensed under MIT license (see LICENSE for details)

use nannou::{color::rgb_u32, prelude::*};

// The maximum recursive depth
const MAX_DEPTH: u8 = 7;

fn main() {
    nannou::app(model)
        .size(500, 500)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    time: f32,
}

fn model(_app: &App) -> Model {
    // We only use a time value
    Model { time: 0.0 }
}

/// Recursively draw squares at each one of the corners
/// The limit is used to halt at 10_000
fn recursive_rect(draw: &Draw, rect: Rect, depth: u8, mut limit: i32, time: f32) {
    if depth == 0 {
        return;
    }

    // How far we are in the recursive calls
    let factor = depth as f32 / MAX_DEPTH as f32;

    draw.rect()
        .xy(rect.xy())
        .wh(rect.wh())
        .rgba8(230, 51, 42, (factor * 255.0) as u8)
        .z_degrees((time + depth as f32).sin() * 90.0 * (1.1 - (0.9 * factor)));

    // Test that we don't go over the limit number
    if limit - 4 >= 0 {
        let time_cos_pos = (time.cos() + 1.0) / 2.0;

        // Loop for every corners
        for corner in rect.corners_iter() {
            let corner = Vec2::new(corner[0], corner[1]);

            // Compute an offset from the center of the square
            let offset = (corner - Vec2::ZERO)
                .normalize()
                .rotate(factor * PI / 2.0 * time.sin())
                * 15.0
                * time_cos_pos
                * factor;

            // Offset the corner position
            let next_pos = corner + offset;

            recursive_rect(
                draw,
                Rect::from_xy_wh(next_pos, rect.wh() / 2.0),
                depth - 1,
                limit,
                time + 5.0,
            );

            limit -= 1;
        }
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.time += 0.1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(rgb_u32(0x263444));

    recursive_rect(
        &draw,
        Rect::from_xy_wh(Vec2::ZERO, pt2(180.0, 180.0)),
        MAX_DEPTH,
        10_000,
        model.time,
    );

    draw.to_frame(app, &frame).unwrap();
}
