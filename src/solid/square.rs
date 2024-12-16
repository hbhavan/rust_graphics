use crate::shape::*;

pub struct Square<'a> {
    side_length: f32,
    offset: &'a Point,
    points: Vec<Point>,
}

impl<'a> GetLines for Square<'a> {
    fn get_lines(&self) -> Vec<Line<&Point>> {
        let lines = (0..4)
            .into_iter()
            .map(|i| Line::new(&self.points[i], &self.points[(i + 1) % 4]))
            .collect::<Vec<Line<&Point>>>();

        return lines;
    }
}

impl<'a> Square<'a> {
    pub fn new(side_length: f32, offset: &'a Point) -> Self {
        let (x, y) = (side_length + offset.x, side_length + offset.y);

        let p1 = Point::from_f32(0.0, 0.0, 0.0);
        let p2 = Point::from_f32(x, 0.0, 0.0);
        let p3 = Point::from_f32(x, y, 0.0);
        let p4 = Point::from_f32(0.0, y, 0.0);

        let points = vec![p1, p2, p3, p4];

        Square {
            side_length,
            offset,
            points,
        }
    }
}
