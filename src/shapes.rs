#[derive(Clone,Copy,Debug)]
pub struct DmPoint {
    pub x: i32,
    pub y: i32,
}

pub struct DmLine {
    pub start: DmPoint,
    pub end: DmPoint,
}
impl DmPoint {
    pub fn new(x: i32, y: i32) -> Self {
        DmPoint { x, y }
    }
    pub fn intermediate_point(&self, p2: DmPoint, ratio: f32) -> DmPoint {
        let x = ((1.0 - ratio) * self.x as f32 + ratio * p2.x as f32) as i32;
        let y = ((1.0 - ratio) * self.y as f32 + ratio * p2.y as f32) as i32;
        DmPoint { x, y }
    }
    pub fn distance_to(&self, other: DmPoint) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }
    // pub fn length_from_line(&self, line: DmLine) -> f64 {
    //     self.distance_to(self.intersection_point(line))
    // }
    // pub fn intersection_point(&self, line: DmLine) -> Option<DmPoint> {
    //     let DmPoint { x: x1, y: y1 } = self.start;
    //     let DmPoint { x: x2, y: y2 } = self.end;
    //     let DmPoint { x: x3, y: y3 } = line.start;
    //     let DmPoint { x: x4, y: y4 } = line.end;

    //     let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

    //     if denominator == 0 {
    //         return None; // DmLines are parallel or coincident
    //     }

    //     let determinant1 = x1 * y2 - y1 * x2;
    //     let determinant2 = x3 * y4 - y3 * x4;

    //     let intersect_x =
    //         ((determinant1 * (x3 - x4)) - (determinant2 * (x1 - x2))) as f64 / denominator as f64;
    //     let intersect_y =
    //         ((determinant1 * (y3 - y4)) - (determinant2 * (y1 - y2))) as f64 / denominator as f64;

    //     Some(DmPoint::new(
    //         intersect_x.round() as i32,
    //         intersect_y.round() as i32,
    //     ))
    // }

    // pub fn is_in_line(&self, line: DmLine) -> bool {
    //     self.intersection_point(line).map_or(false, |intersect| {
    //         let line_length = self.length();
    //         let start_to_intersect = self.start.distance_to(intersect);
    //         let end_to_intersect = self.end.distance_to(intersect);

    //         (start_to_intersect + end_to_intersect - line_length).abs() < f64::EPSILON
    //     })
    // }
    pub fn scale(&self, factor: f64) -> Self {
        DmPoint {
            x: (self.x as f64 * factor) as i32,
            y: (self.y as f64 * factor) as i32,
        }
    }
    pub fn rotate_around(&self, center: &DmPoint, angle: f64) -> Self {
        let dx = self.x - center.x;
        let dy = self.y - center.y;

        let cos_angle = angle.cos();
        let sin_angle = angle.sin();

        let new_x = (dx as f64 * cos_angle - dy as f64 * sin_angle) + center.x as f64;
        let new_y = (dx as f64 * sin_angle + dy as f64 * cos_angle) + center.y as f64;

        DmPoint::new(new_x as i32, new_y as i32)
    }
    pub fn angle_to_vertical_neg(&self, other: &DmPoint) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        // Calculer la longueur du vecteur formé par le premier point et le deuxième point
        let vector_length = (dx.pow(2) + dy.pow(2)) as f64;

        // Calculer la longueur du vecteur vertical (0, 1)
        let vertical_length = 1.0;

        // Utiliser la formule de l'angle entre deux vecteurs pour calculer l'angle
        let angle = (dx as f64 / vector_length).atan2(dy as f64 / vector_length);

        // Convertir l'angle en degrés et ajuster pour le sens trigonométrique négatif
        let mut adjusted_angle = angle.to_degrees();
        if adjusted_angle < 0.0 {
            adjusted_angle += 360.0; // Ajouter 360 degrés pour ajuster dans le sens trigonométrique négatif
        }
        adjusted_angle
    }
}

impl DmLine {
    pub fn new(start: DmPoint, end: DmPoint) -> Self {
        DmLine { start, end }
    }

    pub fn length(&self) -> f64 {
        self.start.distance_to(self.end)
    }
}
