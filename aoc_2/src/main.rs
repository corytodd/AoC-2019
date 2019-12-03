// AOC Day 2
// 2019-12-02
// Create a simple opcode processor
use common;

// OpCode
//    add = 1,      // read and add from two positions, store in third position
//    multiply = 2, // same as add except a multiply is performed
//    end = 99,     // Stop processing immediately

struct Program {
    mem: Vec<i32>,    
}


impl Program {
    fn from_file(file: String) -> Vec<Program> {

        let mut programs: Vec<Program> = Vec::new();

        for program in common::get_file_lines(file) {
            let intcode = program.unwrap().split(",").filter_map(|w| w.parse().ok()).collect();
            programs.push(Program::from_intcode(intcode));
        }

        programs
    }

    fn from_intcode(intcode: Vec<i32>) -> Program {    
        Program {
            mem: intcode
        }
    }

    fn get(&self, index: usize) -> i32 {
        self.mem[index]
    }

    fn set(&mut self, index: usize, new_val: i32) {
        self.mem[index] = new_val;
    }

    fn run(&mut self) -> Vec<i32> {
        
        let mut index: usize = 0;

        loop {
            match self.mem[index] {
                1 => self.add(index),
                2 => self.mult(index),
                99 => break,
                _ => panic!("Unknown opcode: {}", self.mem[index]),
            }

            index += 4;
        }
        
        self.mem.to_vec()
    }

    fn add(&mut self, index: usize) {
        if let [p1, p2, dest] = self.mem[index+1..=index+3] {
            self.mem[dest as usize] = self.mem[p1 as usize] + self.mem[p2 as usize];
        } else {
            panic!("Failed to destructure add");
        }
    }

    fn mult(&mut self, index: usize) {
        if let [p1, p2, dest] = self.mem[index+1..=index+3] {
            self.mem[dest as usize] = self.mem[p1 as usize] * self.mem[p2 as usize];
        } else {
            panic!("Failed to destructure mult");
        }
    }
}

fn main() {
    let data = "aoc_2.txt".to_string();
    
    for mut program in Program::from_file(data) {

        program.set(1, 12);
        program.set(2, 2);

        program.run();

        println!("{}", program.get(0));
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn prog_1() {
        let intcode = vec![1,0,0,0,99];
        let expected = vec![2,0,0,0,99];

        let mut program = Program::from_intcode(intcode);
        let actual = program.run();

        assert_eq!(expected, actual);
    }

    #[test]
    fn prog_2() {
        let intcode = vec![2,3,0,3,99];
        let expected = vec![2,3,0,6,99];

        let mut program = Program::from_intcode(intcode);
        let actual = program.run();

        assert_eq!(expected, actual);
    }

    #[test]
    fn prog_3() {
        let intcode = vec![2,4,4,5,99,0];
        let expected = vec![2,4,4,5,99,9801];

        let mut program = Program::from_intcode(intcode);
        let actual = program.run();

        assert_eq!(expected, actual);
    }

    #[test]
    fn prog_4() {
        let intcode = vec![1,1,1,4,99,5,6,0,99];
        let expected = vec![30,1,1,4,2,5,6,0,99];

        let mut program = Program::from_intcode(intcode);
        let actual = program.run();

        assert_eq!(expected, actual);
    }
}
