// AOC Day 3 Part 2
// 2019-12-06
// Geometry and linear algebra
use common::electronics;

fn main() {
    let circuit = electronics::Circuit::from_file("aoc_3.txt".to_string());

    let (_, d) = circuit.fastest_intersection().unwrap();

    println!("{:?}", d);
}
