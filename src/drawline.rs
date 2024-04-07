use crate::matrix::Matrix;

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug)]
struct ScreenPoint {
    x: i32,
    y: i32,
}

struct Line {
    a: i32,
    b: i32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

const EYE_COORDINATES: Vec<f32> = vec![0.0, 4.0, 100.0];

static NUM_POINTS: i32 = 0;
static NUM_LINES: i32 = 0;
static POINTS: Vec<Point> = Vec::new();
static LINES: Vec<Vec<Line>> = Vec::new();

pub fn set_point(p: Point) {}

pub fn set_line(a: i32, b: i32) {}

pub fn get_sin_x(x: f32, y: f32, z: f32) -> f32 {}

pub fn get_cos_x(x: f32, y: f32, z: f32) -> f32 {}

pub fn get_sin_y(x: f32, y: f32, z: f32) -> f32 {}

pub fn get_cos_y(x: f32, y: f32, z: f32) -> f32 {}

pub fn get_view_matrix(x: f32, y: f32, z: f32) -> Matrix {}

pub fn get_clipping_coordinates() {}

pub fn get_screen_coordinates() {}

pub fn apply_transformation(m: Matrix) {}

pub fn get_num_lines() -> i32 {}

pub fn get_lines() -> Vec<Vec<ScreenPoint>> {}

pub fn print_lines() {}
