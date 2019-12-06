// Calculate fuel required to carry [mass]
pub fn calculate_fuel(mass: i32) -> i32 {
    let d = (mass as f32 / 3.0).floor() as i32;
    std::cmp::max(d - 2, 0)
}

// Calculate fuel required to carry [mass], including mass of fuel itself
pub fn calculate_fuel_with_fuel(mass: i32) -> i32 {
    if mass <= 0 {
        return 0;
    }

    let fuel = calculate_fuel(mass);
    fuel + calculate_fuel_with_fuel(fuel)
}

#[cfg(test)]
mod tests {

    use crate::engine::*;

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
