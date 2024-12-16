use std::f32;

use crate::matrix::Matrix;

pub enum RotationAxis {
    X,
    Y,
    Z,
}

pub fn translate(x: f32, y: f32, z: f32) -> Matrix {
    let t = Matrix::from_vec(vec![
        vec![1.0, 0.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0, 0.0],
        vec![0.0, 0.0, 1.0, 0.0],
        vec![x, y, z, 1.0],
    ]);

    return t;
}

pub fn basic_scale(x: f32, y: f32, z: f32) -> Matrix {
    let t = Matrix::from_vec(vec![
        vec![x, 0.0, 0.0, 0.0],
        vec![0.0, y, 0.0, 0.0],
        vec![0.0, 0.0, z, 0.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ]);

    return t;
}

pub fn scale(x: f32, y: f32, z: f32, cx: f32, cy: f32, cz: f32) -> Matrix {
    let a = translate(-cx, -cy, -cz);
    let b = basic_scale(x, y, z);
    let c = translate(cx, cy, cz);

    return a.matrix_multiply(&b).unwrap().matrix_multiply(&c).unwrap();
}

pub fn rotate(axis: RotationAxis, theta: f32, x: f32, y: f32, z: f32) -> Matrix {
    let cos = f32::cos(theta);
    let sin = f32::sin(theta);

    let a = translate(-x, -y, -z);
    let b = translate(x, y, z);

    let r = match axis {
        RotationAxis::X => get_rotation_matrix_x(sin, cos),
        RotationAxis::Y => get_rotation_matrix_y(sin, cos),
        RotationAxis::Z => get_rotation_matrix_z(sin, cos),
    };

    return a.matrix_multiply(&r).unwrap().matrix_multiply(&b).unwrap();
}

#[allow(dead_code)]
fn get_rotation_matrix_x(sin: f32, cos: f32) -> Matrix {
    let r = Matrix::from_vec(vec![
        vec![1.0, 0.0, 0.0, 0.0],
        vec![0.0, cos, sin, 0.0],
        vec![0.0, -sin, cos, 0.0],
        vec![0.0, 0.0, 0.0, 0.0],
    ]);

    return r;
}

#[allow(dead_code)]
fn get_rotation_matrix_y(sin: f32, cos: f32) -> Matrix {
    let r = Matrix::from_vec(vec![
        vec![cos, 0.0, sin, 0.0],
        vec![0.0, 1.0, 0.0, 0.0],
        vec![-sin, 0.0, cos, 0.0],
        vec![0.0, 0.0, 0.0, 0.0],
    ]);

    return r;
}

#[allow(dead_code)]
fn get_rotation_matrix_z(sin: f32, cos: f32) -> Matrix {
    let r = Matrix::from_vec(vec![
        vec![cos, sin, 0.0, 0.0],
        vec![-sin, cos, 0.0, 0.0],
        vec![0.0, 0.0, 1.0, 0.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ]);

    return r;
}
