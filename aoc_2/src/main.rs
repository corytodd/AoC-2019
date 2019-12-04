// AOC Day 2
// 2019-12-02
// Create a simple opcode processor
use common;



fn main() {
    let data = "aoc_2.txt".to_string();
    
    for mut program in common::Program::from_file(data) {

        program.set(1, 12);
        program.set(2, 2);

        program.run();

        println!("{}", program.get(0));
    }
}