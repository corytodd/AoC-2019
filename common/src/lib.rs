use std::cmp;
use std::path::{Path};
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};

// Calculate fuel required to carry [mass]
pub fn calculate_fuel(mass: i32) -> i32 {
    let d = (mass as f32 / 3.0).floor() as i32;
    cmp::max(d - 2, 0)
}

// Calculate fuel required to carry [mass], including mass of fuel itself
pub fn calculate_fuel_with_fuel(mass: i32) -> i32 {
    if mass <= 0 {
        return 0;
    }

    let fuel = calculate_fuel(mass);
    fuel + calculate_fuel_with_fuel(fuel)
}

// Return each line from [file_name] assumed to be in "assets" directory
pub fn get_file_lines(file_name: String) -> Lines<BufReader<File>> {
    let cd = &env::current_dir().unwrap();
    let file_path = Path::new(cd).join("assets").join(file_name);
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    reader.lines()
}

// A simple program 
// OpCode
//    add = 1,      // read and add from two positions, store in third position
//    multiply = 2, // same as add except a multiply is performed
//    end = 99,     // Stop processing immediately
pub struct Program {
    mem: Vec<i32>,
    ip: usize 
}


impl Program {
    // Create new [Program] from text [file]
    pub fn from_file(file: String) -> Vec<Program> {

        let mut programs: Vec<Program> = Vec::new();

        for program in get_file_lines(file) {
            let intcode = program.unwrap().split(",").filter_map(|w| w.parse().ok()).collect();
            programs.push(Program::from_intcode(intcode));
        }

        programs
    }

    // Create new [Program] from raw [intcode]
    fn from_intcode(intcode: Vec<i32>) -> Program {    
        Program {
            mem: intcode,
            ip: 0
        }
    }

    // Return value at specified memory address
    pub fn get(&self, index: usize) -> i32 {
        self.mem[index]
    }

    // Set value at specified memory address
    pub fn set(&mut self, index: usize, new_val: i32) {
        self.mem[index] = new_val;
    }

    // Execute program and return final state of memory as vector
    pub fn run(&mut self) -> Vec<i32> {
        
        self.ip = 0;

        loop {
            match self.mem[self.ip] {
                1 => self.add(self.ip),
                2 => self.mult(self.ip),
                99 => break,
                _ => panic!("Unknown opcode: {}", self.mem[self.ip]),
            }

            self.ip += 4;
        }
        
        self.mem.to_vec()
    }

    // Perform an add operation at [index]
    fn add(&mut self, index: usize) {
        if let [p1, p2, dest] = self.mem[index+1..=index+3] {
            self.mem[dest as usize] = self.mem[p1 as usize] + self.mem[p2 as usize];
        } else {
            panic!("Failed to destructure add");
        }
    }

    // Perform an multiply operation at [index]
    fn mult(&mut self, index: usize) {
        if let [p1, p2, dest] = self.mem[index+1..=index+3] {
            self.mem[dest as usize] = self.mem[p1 as usize] * self.mem[p2 as usize];
        } else {
            panic!("Failed to destructure mult");
        }
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn mass_to_fuel() {
        let mass = 1969;
        let expected = 654;
        let actual = calculate_fuel(mass);

        assert_eq!(expected, actual);
    }

    #[test]
    fn mass_to_fuels_fuel1() {
        let mass = 1969;
        let expected = 966;
        let actual = calculate_fuel_with_fuel(mass);

        assert_eq!(expected, actual);
    }

    #[test]
    fn mass_to_fuels_fuel2() {
        let mass = 100756;
        let expected = 50346;
        let actual = calculate_fuel_with_fuel(mass);

        assert_eq!(expected, actual);
    }

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
