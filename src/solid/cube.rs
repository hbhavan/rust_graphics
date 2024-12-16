use crate::shape::*;
#[allow(dead_code)]
pub struct Cube<'a> {
    side_length: f32,
    offset: &'a Point,
    points: Vec<Point>,
}

impl GetLines for Cube<'_> {
    fn get_lines(&self) -> Vec<Line<&Point>> {
        let mut lines = Vec::new();
        (0..4).into_iter().for_each(|i| {
            lines.push(Line::new(&self.points[i], &self.points[i + 4]));
            lines.push(Line::new(&self.points[i], &self.points[(i + 1) % 4]));
            lines.push(Line::new(
                &self.points[i + 4],
                &self.points[(i + 1) % 4 + 4],
            ));
        });
        return lines;
    }
}

#[allow(dead_code)]
impl<'a> Cube<'a> {
    pub fn new(side_length: f32, offset: &'a Point) -> Self {
        let (x0, y0, z0, x1, y1, z1) = (
            offset.x,
            offset.y,
            offset.z,
            side_length + offset.x,
            side_length + offset.y,
            side_length + offset.z,
        );

        let p1 = Point::from_f32(x0, y0, z0);
        let p2 = Point::from_f32(x1, y0, z0);
        let p3 = Point::from_f32(x1, y1, z0);
        let p4 = Point::from_f32(x0, y1, z0);
        let p5 = Point::from_f32(x0, y0, z1);
        let p6 = Point::from_f32(x1, y0, z1);
        let p7 = Point::from_f32(x1, y1, z1);
        let p8 = Point::from_f32(x0, y1, z1);

        let points = vec![p1, p2, p3, p4, p5, p6, p7, p8];

        Cube {
            side_length,
            offset,
            points,
        }
    }
}
