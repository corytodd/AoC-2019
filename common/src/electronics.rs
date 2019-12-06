use crate::get_file_lines;
use crate::linalg::{Line, Point};

#[derive(Debug)]
struct Node {
    direction: char,
    steps: i32,
}

pub struct Circuit {
    pub wire_1: Vec<Line>,
    pub wire_2: Vec<Line>,
}

impl Circuit {
    pub fn from_file(file_name: String) -> Circuit {
        let lines: Vec<String> = get_file_lines(file_name)
            .map(|l| l.expect("Could not read circuit"))
            .collect();

        let wire_1 = Circuit::read_segments(&lines[0]);
        let wire_2 = Circuit::read_segments(&lines[1]);

        Circuit { wire_1, wire_2 }
    }

    pub fn origin() -> Point {
        Point{x: 1, y: 1}
    }

    #[cfg(test)]
    fn from_segments(s1: &String, s2: &String) -> Circuit {
        let wire_1 = Circuit::read_segments(s1);
        let wire_2 = Circuit::read_segments(s2);

        Circuit { wire_1, wire_2 }
    }

    fn read_segments(wire: &String) -> Vec<Line> {
        let mut segments: Vec<Line> = Vec::new();

        let codes: Vec<Node> = wire
            .split(",")
            .filter_map(|w| {
                let direction = w.chars().nth(0).unwrap();
                let steps = *(&w[1..].parse().unwrap());
                Some(Node { direction, steps })
            })
            .collect();

        // Relative origin
        let mut p1 = Circuit::origin();

        for node in codes {
            // Moves to next point
            let mut p2 = p1.clone();

            match node.direction {
                'L' => p2.x -= node.steps,
                'R' => p2.x += node.steps,
                'U' => p2.y += node.steps,
                'D' => p2.y -= node.steps,
                _ => panic!("Bad direction"),
            }

            segments.push(Line {
                p1: p1.clone(),
                p2: p2.clone(),
            });

            // Move p1 to next start
            p1 = p2.clone()
        }

        segments
    }

    pub fn closest_intersection(&self) -> Option<Point> {
        let mut inter: Vec<Point> = Vec::new();

        // Skip the origin segment for both wires
        for l1 in self.wire_1.iter() {
            for l2 in self.wire_2.iter() {

                match l1.get_intersection(&l2) {
                    Some(p) => {
                        // Do not allow intersections on the origin
                        if p == Circuit::origin() {
                            continue;
                        }
                        inter.push(p);
                    },
                    None => continue,
                }
            }
        }

        if inter.len() > 0 {
            let p1 = Circuit::origin();
            let mut dist = std::u64::MAX;
            let mut closest: Point = p1.clone();

            for p in inter.iter() {

                if *p == p1 {
                    continue;
                }

                let d = p1.mdist(&p);

                if d < dist {
                    dist = d;
                    closest = p.clone();
                }
            }


            // Not:
            // 6

            Some(closest)

        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::electronics::Circuit;
    use crate::linalg::Point;

    #[test]
    fn circuit_1() {
        let wire_1 = "R8,U5,L5,D3".to_string();
        let wire_2 = "U7,R6,D4,L4".to_string();

        let expected_point = Point {x: 4, y: 4};
        let expected_dist: u64 = 6;

        let circuit = Circuit::from_segments(&wire_1, &wire_2);
        let actual_point = circuit.closest_intersection().unwrap();

        let p1 = Circuit::origin();
        let actual_dist = p1.mdist(&actual_point);

        assert_eq!(expected_point, actual_point);
        assert_eq!(expected_dist, actual_dist);
    }

    #[test]
    fn circuit_2() {
        let wire_1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string();
        let wire_2 = "U62,R66,U55,R34,D71,R55,D58,R83".to_string();

        let circuit = Circuit::from_segments(&wire_1, &wire_2);
        let closest_point = circuit.closest_intersection().unwrap();

        let p1 = Circuit::origin();
        let d = p1.mdist(&closest_point);

        assert_eq!(159u64, d);
    }

    #[test]
    fn circuit_3() {
        let wire_1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string();
        let wire_2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string();

        let circuit = Circuit::from_segments(&wire_1, &wire_2);
        let closest_point = circuit.closest_intersection().unwrap();

        let p1 = Circuit::origin();
        let d = p1.mdist(&closest_point);

        assert_eq!(135u64, d);
    }
}
