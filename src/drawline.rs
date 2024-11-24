use std::f32;
use std::rc::Rc;

use nannou::geom::Point2;

use crate::{
    matrix::Matrix,
    solid::{Line, Point, Scene, ScreenParameter},
    transformations::{basic_scale, translate},
};

pub fn get_sin_x(x: f32, y: f32, z: f32) -> Option<f32> {
    if x + y + z == 0.0 {
        return None;
    } else {
        let result = z / f32::sqrt(f32::powi(x, 2) + f32::powi(y, 2) + f32::powi(z, 2));
        return Some(result);
    }
}

pub fn get_cos_x(x: f32, y: f32, z: f32) -> Option<f32> {
    if x + y + z == 0.0 {
        return None;
    } else {
        let result = f32::sqrt(f32::powi(x, 2) + f32::powi(y, 2))
            / f32::sqrt(f32::powi(x, 2) + f32::powi(y, 2) + f32::powi(z, 2));
        return Some(result);
    }
}

pub fn get_sin_y(x: f32, y: f32) -> Option<f32> {
    if x + y == 0.0 {
        return None;
    }
    let result = x / f32::sqrt(f32::powi(x, 2) + f32::powi(y, 2));
    return Some(result);
}

pub fn get_cos_y(x: f32, y: f32) -> Option<f32> {
    if x + y == 0.0 {
        return None;
    }
    let result = y / f32::sqrt(f32::powi(x, 2) + f32::powi(y, 2));

    return Some(result);
}

pub fn get_view_matrix(x: f32, y: f32, z: f32) -> Matrix {
    let sin_x = get_sin_x(x, y, z).unwrap_or(0.0);
    let cos_x = get_cos_x(x, y, z).unwrap_or(0.0);
    let sin_y = get_sin_y(x, y).unwrap_or(0.0);
    let cos_y = get_cos_y(x, y).unwrap_or(0.0);

    let t1 = translate(-x, -y, -z);

    let mut t2 = Matrix::new(4, 4);
    t2.set(0, 0, 1.0);
    t2.set(1, 2, -1.0);
    t2.set(2, 1, 1.0);
    t2.set(3, 3, 1.0);

    let mut t3 = Matrix::new(4, 4);
    t3.set(0, 0, -cos_y);
    t3.set(0, 2, sin_y);
    t3.set(1, 1, 1.0);
    t3.set(2, 0, -sin_y);
    t3.set(2, 2, -cos_y);
    t3.set(3, 3, 1.0);

    let mut t4 = Matrix::new(4, 4);
    t4.set(0, 0, 1.0);
    t4.set(1, 1, cos_x);
    t4.set(1, 2, sin_x);
    t4.set(2, 1, -sin_x);
    t4.set(2, 2, cos_x);
    t4.set(3, 3, 1.0);

    let mut t5 = Matrix::new(4, 4);
    t5.set(0, 0, 1.0);
    t5.set(1, 1, 1.0);
    t5.set(2, 2, -1.0);
    t5.set(3, 3, 1.0);

    let r = t1
        .matrix_multiply(&t2)
        .unwrap()
        .matrix_multiply(&t3)
        .unwrap()
        .matrix_multiply(&t4)
        .unwrap()
        .matrix_multiply(&t5)
        .unwrap();

    return r;
}

pub fn get_clip_matrix(view_angle: f32) -> Matrix {
    let ds = view_angle;
    let r = basic_scale(ds, ds, 1.0);

    return r;
}

pub fn get_clipping_coordinates(
    eye_coordinates: [f32; 3],
    view_angle: f32,
    scene: &Scene<&Point>,
) -> Scene<Point> {
    let view_matrix = get_view_matrix(eye_coordinates[0], eye_coordinates[1], eye_coordinates[2]);
    let clip_matrix = get_clip_matrix(view_angle);

    let result_matrix = view_matrix.matrix_multiply(&clip_matrix).unwrap();

    let clipped_lines = scene
        .lines
        .iter()
        .map(|line| Line {
            a: Rc::new(line.a.apply_matrix(&result_matrix)),
            b: Rc::new(line.b.apply_matrix(&result_matrix)),
        })
        .collect::<Vec<Line<Point>>>();

    return Scene {
        num_lines: clipped_lines.len(),
        lines: clipped_lines,
    };
}

pub fn get_screen_coordinates(
    scene: &Scene<Point>,
    vsx: f32,
    vsy: f32,
    vcx: f32,
    vcy: f32,
) -> Scene<Point2> {
    let screen_lines = scene
        .lines
        .iter()
        .map(|line| Line {
            a: Rc::new(line.a.to_screen_point(vsx, vsy, vcx, vcy)),
            b: Rc::new(line.b.to_screen_point(vsx, vsy, vcx, vcy)),
        })
        .collect::<Vec<Line<Point2>>>();

    return Scene {
        num_lines: screen_lines.len(),
        lines: screen_lines,
    };
}

pub fn create_scene(scene: Scene<&Point>, screen_parameter: &ScreenParameter) -> Scene<Point2> {
    let eye_coordinates = screen_parameter.get_eye_coordinates();
    let view_angle = screen_parameter.get_view_angle();
    let (vsx, vsy, vcx, vcy) = screen_parameter.get_view_data();

    let clipped_points = get_clipping_coordinates(eye_coordinates, view_angle, &scene);
    let screen_points = get_screen_coordinates(&clipped_points, vsx, vsy, vcx, vcy);

    return screen_points;
}

#[allow(dead_code)]
fn print_original_points(scene: &Scene<&Point>) {
    println!("----------------ORIGINAL LINES----------------");
    scene.lines.iter().for_each(|line| {
        println!("{} - {}", line.a.to_string(), line.b.to_string());
    })
}
#[allow(dead_code)]
fn print_clipped_points(scene: &Scene<Point>) {
    println!("----------------CLIPPED LINES----------------");
    scene.lines.iter().for_each(|line| {
        println!("{} - {}", line.a.to_string(), line.b.to_string());
    })
}
#[allow(dead_code)]
fn print_screen_points(scene: &Scene<Point2>) {
    println!("----------------SCREEN LINES----------------");
    scene.lines.iter().for_each(|line| {
        println!("{} - {}", line.a.to_string(), line.b.to_string());
    })
}
