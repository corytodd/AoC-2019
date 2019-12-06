// 2D point
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn mdist(&self, other: &Point) -> u64 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u64
    }
}

// Line segment
#[derive(Debug)]
pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

impl Line {
    pub fn get_intersection(&self, other: &Line) -> Option<Point> {
        let x1 = self.p1.x as i64;
        let x2 = self.p2.x as i64;
        let y1 = self.p1.y as i64;
        let y2 = self.p2.y as i64;

        let x3 = other.p1.x as i64;
        let x4 = other.p2.x as i64;
        let y3 = other.p1.y as i64;
        let y4 = other.p2.y as i64;

        let n1 = (((x1 * y2) - (y1 * x2)) * (x3 - x4)) - ((x1 - x2) * ((x3 * y4) - (y3 * x4)));
        let n2 = (((x1 * y2) - (y1 * x2)) * (y3 - y4)) - ((y1 - y2) * ((x3 * y4) - (y3 * x4)));
        let d = ((x1 - x2) * (y3 - y4)) - ((y1 - y2) * (x3 - x4));

        if d == 0 {
            
            None

        } else {

            let p = Point {
                x: (n1 as f64 / d as f64).round() as i32,
                y: (n2 as f64 / d as f64).round() as i32,
            };

            // Check that point is on both lines
            if !self.contains(&p) {
                return None
            }
            
            if !other.contains(&p) {
                return None
            }

            Some(p)
        }
    }

    // Return true in point [p] is on this segment
    fn contains(&self, p: &Point) -> bool {
        let xmin = std::cmp::min(self.p1.x, self.p2.x);
        let xmax = std::cmp::max(self.p1.x, self.p2.x);
        let ymin = std::cmp::min(self.p1.y, self.p2.y);
        let ymax = std::cmp::max(self.p1.y, self.p2.y);

        xmin <= p.x && p.x <= xmax && ymin <= p.y && p.y <= ymax
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn line_intersect() {
        let line1 = Line {
            p1: Point { x: 15, y: 10 },
            p2: Point { x: 49, y: 25 },
        };
        let line2 = Line {
            p1: Point { x: 29, y: 5 },
            p2: Point { x: 32, y: 32 },
        };

        let expected = Point { x: 30, y: 17 };
        let actual = line1.get_intersection(&line2);

        assert_eq!(true, actual.is_some());

        let actual = actual.unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn line_not_intersect() {
        let line1 = Line {
            p1: Point { x: 0, y: 0 },
            p2: Point { x: 0, y: 5 },
        };
        let line2 = Line {
            p1: Point { x: 2, y: 1 },
            p2: Point { x: 2, y: 6 },
        };

        let actual = line1.get_intersection(&line2);

        assert_eq!(false, actual.is_some());
    }

    #[test]
    fn line_not_intersect_2() {
        let line1 = Line {
            p1: Point { x: 9, y: 1 },
            p2: Point { x: 9, y: 6 },
        };
        let line2 = Line {
            p1: Point { x: 1, y: 8 },
            p2: Point { x: 7, y: 8 },
        };

        let actual = line1.get_intersection(&line2);

        assert_eq!(false, actual.is_some());
    }
}
