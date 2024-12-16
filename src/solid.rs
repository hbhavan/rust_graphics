pub mod cube;
pub mod pyramid;
pub mod square;
pub mod triangle;

use std::rc::Rc;

use nannou::geom::{pt2, Point2};

use crate::matrix::Matrix;

pub trait GetLines {
    fn get_lines(&self) -> Vec<Line<&Point>>;
}

pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Line<P> {
    pub a: Rc<P>,
    pub b: Rc<P>,
}

pub struct Scene<P> {
    pub num_lines: usize,
    pub lines: Vec<Line<P>>,
}

pub struct ScreenParameter {
    eye_coordinates: [f32; 3],
    view_distance: f32,
    screen_size: f32,
    vcx: f32,
    vcy: f32,
    vsx: f32,
    vsy: f32,
}

impl Point {
    pub fn from_f32(x: f32, y: f32, z: f32) -> Point {
        return Point { x, y, z };
    }

    pub fn from_binary(index: i32, scalor: f32, offset: &Point) -> Point {
        return Point {
            x: (index & 1) as f32 * scalor + offset.x,
            y: (index & 2 >> 1) as f32 * scalor + offset.y,
            z: (index & 4 >> 2) as f32 * scalor + offset.z,
        };
    }

    pub fn apply_matrix(&self, matrix: &Matrix) -> Point {
        let mut a = Matrix::new(1, 4);
        a.set(0, 0, self.x);
        a.set(0, 1, self.y);
        a.set(0, 2, self.z);
        a.set(0, 3, 1.0);

        let b = a.matrix_multiply(matrix).unwrap();

        return Point {
            x: b.at(0, 0),
            y: b.at(0, 1),
            z: b.at(0, 2),
        };
    }

    pub fn to_screen_point(&self, vsx: f32, vsy: f32, vcx: f32, vcy: f32) -> Point2 {
        let x = ((self.x / self.z) * vsx) + vcx;
        let y = ((self.y / self.z) * vsy) + vcy;

        return pt2(x, y);
    }

    pub fn to_string(&self) -> String {
        return format!(
            "({}, {}, {})",
            self.x.to_string(),
            self.y.to_string(),
            self.z.to_string()
        );
    }
}

impl<P> Line<P> {
    pub fn new(a: P, b: P) -> Self {
        Self {
            a: Rc::new(a),
            b: Rc::new(b),
        }
    }
}

impl<P> Scene<P> {
    pub fn new() -> Self {
        Self {
            num_lines: 0,
            lines: Vec::new(),
        }
    }
}

impl ScreenParameter {
    pub fn new(
        eye_coordinates: [f32; 3],
        view_distance: f32,
        screen_size: f32,
        vsx: f32,
        vsy: f32,
        vcx: f32,
        vcy: f32,
    ) -> Self {
        Self {
            eye_coordinates,
            view_distance,
            screen_size,
            vsx,
            vsy,
            vcx,
            vcy,
        }
    }
    pub fn get_eye_coordinates(&self) -> [f32; 3] {
        return self.eye_coordinates;
    }
    pub fn get_view_data(&self) -> (f32, f32, f32, f32) {
        return (self.vsx, self.vsy, self.vcx, self.vcy);
    }

    pub fn get_view_angle(&self) -> f32 {
        return self.view_distance / self.screen_size;
    }
}
