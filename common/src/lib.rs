use std::cmp;
use std::path::{Path};
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};

pub fn calculate_fuel(mass: i32) -> i32 {
    let d = (mass as f32 / 3.0).floor() as i32;
    cmp::max(d - 2, 0)
}

pub fn calculate_fuel_with_fuel(mass: i32) -> i32 {
    if mass <= 0 {
        return 0;
    }

    let fuel = calculate_fuel(mass);
    fuel + calculate_fuel_with_fuel(fuel)
}

pub fn get_file_lines(file_name: String) -> Lines<BufReader<File>> {
    let cd = &env::current_dir().unwrap();
    let file_path = Path::new(cd).join("assets").join(file_name);
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    reader.lines()
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
}
