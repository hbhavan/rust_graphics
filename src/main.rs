mod drawline;
mod matrix;
mod solid;
mod tests;
mod transformations;

use nannou::prelude::*;
use solid::{GetLines, Scene, ScreenParameter};

fn get_scene_parameters() -> ScreenParameter {
    let eye_coodinates = [0.0, 4.0, 100.0];
    let view_distance = 60.0;
    let screen_size = 30.0;
    let (vsx, vsy, vcx, vcy) = (400.0, 400.0, 0.0, 0.0);

    return ScreenParameter::new(
        eye_coodinates,
        view_distance,
        screen_size,
        vsx,
        vsy,
        vcx,
        vcy,
    );
}

fn get_scene() -> Scene<Point2> {
    let origin = solid::Point {
        x: -20.0,
        y: -20.0,
        z: -20.0,
    };

    let mut scene_lines = Vec::new();

    let square = solid::square::Square::new(6.0, &origin);
    let square_lines = square.get_lines();
    square_lines.iter().for_each(|line| scene_lines.push(line));

    let cube = solid::cube::Cube::new(40.0, &origin);
    let cube_lines = cube.get_lines();
    cube_lines.iter().for_each(|line| scene_lines.push(line));

    let scene = Scene {
        num_lines: square_lines.len() + cube_lines.len(),
        lines: cube_lines,
    };

    let screen_parameter = get_scene_parameters();

    return drawline::create_scene(scene, &screen_parameter);
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BURLYWOOD);

    let win = app.window_rect();
    let t = app.time;

    let scene = get_scene();

    scene.lines.iter().for_each(|line| {
        draw.line()
            .start(*line.a)
            .end(*line.b)
            .weight(4.0)
            .color(STEELBLUE);
    });

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::sketch(view).size(800, 800).run();
}
