// AOC Day 1
// 2019-12-01
// Solve the fuel equation: sum(round(m/3) - 2)
// for all modules in aoc_1.txt
use common;

fn main() {

    let data = "aoc_1.txt".to_string();

    let mut sum = 0;

    for line in common::get_file_lines(data) {
        let raw = line.unwrap();        
        let mass: i32 = match raw.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        sum += common::calculate_fuel(mass);
    }    

    print!("Total fuel required is {}", sum);

}