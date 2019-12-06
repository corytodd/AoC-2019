// AOC Day 3 Part 1
// 2019-12-02
// Geometry and linear algebra
use common::electronics;

fn main() {
    let circuit = electronics::Circuit::from_file("aoc_3.txt".to_string());

    let point = circuit.closest_intersection().unwrap();

    let d = electronics::Circuit::origin().mdist(&point);
    println!("{:?}", d);
}
