// AOC Day 2
// 2019-12-01
// Solve the recursive fuel equation: sum(round(m/3) - 2) + sum(round(m_f/3)-1)
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
        sum += common::calculate_fuel_with_fuel(mass);        
    }    

    // Not:
    // 5304019 (low)
    // 5304147 (correct)
    print!("Total fuel required is {}", sum);

}