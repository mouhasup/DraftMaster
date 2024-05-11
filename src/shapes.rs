pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    pub fn length_form_point(&self, other: Point) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }
    pub fn length_from_line(&self, line: Line) -> f64 {
        self.length_form_point(self.intersection_point(line))
    }
    pub fn intersection_point(&self, line: Line) -> Option<Point> {
        let Point { x: x1, y: y1 } = self.start;
        let Point { x: x2, y: y2 } = self.end;
        let Point { x: x3, y: y3 } = line.start;
        let Point { x: x4, y: y4 } = line.end;

        let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        if denominator == 0 {
            return None; // Lines are parallel or coincident
        }

        let determinant1 = x1 * y2 - y1 * x2;
        let determinant2 = x3 * y4 - y3 * x4;

        let intersect_x =
            ((determinant1 * (x3 - x4)) - (determinant2 * (x1 - x2))) as f64 / denominator as f64;
        let intersect_y =
            ((determinant1 * (y3 - y4)) - (determinant2 * (y1 - y2))) as f64 / denominator as f64;

        Some(Point::new(
            intersect_x.round() as i32,
            intersect_y.round() as i32,
        ))
    }
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        Line { start, end }
    }

    pub fn length(&self) -> f64 {
        self.start.length_form_point(self.end)
    }
}
