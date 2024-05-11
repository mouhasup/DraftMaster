pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Point {
/// Creates a new Point.
///
/// # Parameters
/// - `x`: i32 representing the x-coordinate.
/// - `y`: i32 representing the y-coordinate.
///
/// # Returns
/// Returns a new Point instance with the specified x and y values.

/// Calculates the distance from this point to another point.
///
/// # Parameters
/// - `other`: the other Point to which distance is calculated.
///
/// # Returns
/// Returns the Euclidean distance as a f64.



/// Calculates the length from this point to a given line using perpendicular distance.
///
/// # Parameters
/// - `line`: Line from which the distance will be calculated.
///
/// # Returns
/// Returns the minimum distance as a f64.



/// Calculates the intersection point of this line with another line if they intersect.
///
/// # Parameters
/// - `line`: another Line to check intersection with.
///
/// # Returns
/// Returns an Option<Point>. If the lines intersect, returns Some(Point) where the lines intersect; otherwise, None.



/// Checks if this point lies on the given line.
///
/// # Parameters
/// - `line`: Line to check if the point lies on.
///
/// # Returns
/// Returns true if this point lies on `line`, false otherwise.



/// Scales the point coordinates by a specified factor.
///
/// # Parameters
/// - `factor`: f64 scaling factor for both x and y coordinates.
///
/// # Returns
/// Returns a new Point with scaled coordinates.



/// Rotates this point around another point by a given angle.
///
/// # Parameters
/// - `center`: Point around which this point will be rotated.
/// - `angle`: f64 representing the angle in radians to rotate.
///
/// # Returns
/// Returns the new Point after rotation.



/// Calculates the angle between this point and another point relative to the negative vertical direction.
///
/// # Parameters
/// - `other`: another Point used to calculate the angle relative to this point.
///
/// # Returns
/// Returns the angle in degrees with adjustments for negative trigonometric sense.



/// Creates a new Line.
///
/// # Parameters
/// - `start`: Point where the line starts.
/// - `end`: Point where the line ends.
///
/// # Returns
/// Returns a new Line instance with the specified start and end points.



/// Computes the length of the line.
///
/// # Returns
/// Returns the distance between the start and end points of the line as a f64.

    
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    pub fn distance_to(&self, other: Point) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }
    pub fn length_from_line(&self, line: Line) -> f64 {
        self.distance_to(self.intersection_point(line))
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
    pub fn is_in_line(&self, line: Line) -> bool {
        self.intersection_point(line).map_or(false, |intersect| {
            let line_length = self.length();
            let start_to_intersect = self.start.distance_to(intersect);
            let end_to_intersect = self.end.distance_to(intersect);

            (start_to_intersect + end_to_intersect - line_length).abs() < f64::EPSILON
        })
    }
    pub fn scale(&self, factor: f64) -> Self {
        Point {
            x: (self.x as f64 * factor) as i32,
            y: (self.y as f64 * factor) as i32,
        }
    }
    pub fn rotate_around(&self, center: &Point, angle: f64) -> Self {
        let dx = self.x - center.x;
        let dy = self.y - center.y;

        let cos_angle = angle.cos();
        let sin_angle = angle.sin();

        let new_x = (dx * cos_angle - dy * sin_angle) + center.x;
        let new_y = (dx * sin_angle + dy * cos_angle) + center.y;

        Point::new(new_x as i32, new_y as i32)
    }
    pub fn angle_to_vertical_neg(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        // Calculer la longueur du vecteur formé par le premier point et le deuxième point
        let vector_length = (dx.pow(2) + dy.pow(2)) as f64;

        // Calculer la longueur du vecteur vertical (0, 1)
        let vertical_length = 1.0;

        // Utiliser la formule de l'angle entre deux vecteurs pour calculer l'angle
        let angle = (dx / vector_length).atan2(dy / vector_length);

        // Convertir l'angle en degrés et ajuster pour le sens trigonométrique négatif
        let adjusted_angle = angle.to_degrees();
        if adjusted_angle < 0.0 {
            adjusted_angle += 360.0; // Ajouter 360 degrés pour ajuster dans le sens trigonométrique négatif
        }
        adjusted_angle
    }
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        Line { start, end }
    }

    pub fn length(&self) -> f64 {
        self.start.distance_to(self.end)
    }
}
