// AOC Day 2 Part 2
// 2019-12-02
// Create a simple opcode processor
use common;



fn main() {

    for noun in 0..=99 {
        for verb in 0..99 {
            for mut program in common::Program::from_file("aoc_2.txt".to_string()) {

                program.set(1, noun);
                program.set(2, verb);
        
                program.run();
                
                let result = program.get(0);

                if result == 19690720 {
                    println!("Found Noun,Verb: {},{}", noun, verb);
                    println!("{}", 100 * noun + verb);
                    break;
                }
            }
        }    
    }
}