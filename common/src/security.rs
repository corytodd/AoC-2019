pub struct Login {
    password_len: usize,
    range_start: i32,
    range_end: i32,
}

impl Login {
    pub fn new() -> Login {
        Login {
            password_len: 6,
            range_start: 156218,
            range_end: 652527,
        }
    }

    pub fn is_valid(&self, s: &String) -> bool {
        self.check_length(s)
            && self.check_in_range(s)
            && self.check_has_adjacent_doubles(s)
            && self.check_is_not_decreasing(s)
    }

    fn check_length(&self, s: &String) -> bool {
        s.len() == self.password_len
    }

    // No range check on unit tests
    #[cfg(test)]
    fn check_in_range(&self, _: &String) -> bool {
        true
    }

    #[cfg(not(test))]
    fn check_in_range(&self, s: &String) -> bool {
        match s.parse() {
            Ok(d) => self.range_start <= d && d <= self.range_end,
            Err(_) => false,
        }
    }

    fn check_has_adjacent_doubles(&self, s: &String) -> bool {
        let digits = self.get_digits(s);

        if digits.is_none() {
            return false;
        }

        let digits = digits.unwrap();
        let mut prev = &digits[0];

        for next in digits.iter().skip(1) {
            if next == prev {
                return true;
            }
            prev = next;
        }

        false
    }

    fn check_is_not_decreasing(&self, s: &String) -> bool {
        let digits = self.get_digits(s);
        if digits.is_none() {
            return false;
        }

        let digits = digits.unwrap();
        let mut prev = 0;
        for d in digits.iter() {
            if *d < prev {
                return false;
            }
            prev = *d;
        }

        return true;
    }

    fn get_digits(&self, s: &String) -> Option<Vec<i32>> {
        let numbers: Vec<_> = s
            .chars()
            .map(|s| s.to_string().parse::<i32>())
            .filter_map(Result::ok)
            .collect();

        if numbers.len() == self.password_len {
            Some(numbers)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn pwd_1() {
        let login = Login::new();
        assert_eq!(true, login.is_valid(&"111111".to_string()));
    }

    #[test]
    fn pwd_2() {
        let login = Login::new();
        assert_eq!(false, login.is_valid(&"223450".to_string()));
    }

    #[test]
    fn pwd_3() {
        let login = Login::new();
        assert_eq!(false, login.is_valid(&"123789".to_string()));
    }
}
