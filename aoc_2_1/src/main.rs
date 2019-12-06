// AOC Day 2 Part 1
// 2019-12-02
// Create a simple opcode processor
use common::computer;

fn main() {
    let data = "aoc_2.txt".to_string();

    for mut program in computer::Program::from_file(data) {
        program.set(1, 12);
        program.set(2, 2);

        program.run();

        println!("{}", program.get(0));
    }
}
