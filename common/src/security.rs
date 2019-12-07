use std::collections::HashMap;

pub struct Login {
    password_len: usize,
    range_start: i32,
    range_end: i32,
    strict: bool,
}

impl Login {
    pub fn new(required_len: usize, min_value: i32, max_value: i32, strict: bool) -> Login {
        Login {
            password_len: required_len,
            range_start: min_value,
            range_end: max_value,
            strict,
        }
    }

    // TODO we could make this smart to prefilter invalid values
    pub fn get_range_iter(&self) -> std::ops::Range<i32> {
        self.range_start..self.range_end
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

    fn check_in_range(&self, s: &String) -> bool {
        // If min and max are the same, skip range test
        if self.range_start == self.range_end {
            return true;
        }

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
        let repeated = self.find_repeated(&digits);

        match repeated {
            Some(r) => {
                if self.strict {
                    digits.iter().filter(|&x| *x == r).count() == 2

                } else {
                    true
                }
            },
            None => false
        }
    }

    fn check_is_not_decreasing(&self, s: &String) -> bool {
        let digits = self.get_digits(s);
        if digits.is_none() {
            return false;
        }

        let mut prev = 0;
        for d in digits.unwrap().iter() {
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

    fn find_repeated(&self, digits: &Vec<i32>) -> Option<i32> {
        let mut counts: HashMap<i32, i32> = HashMap::new();

        let mut i = 0;
        for j in 1..digits.len() {
            if digits[i] == digits[j] {
                *counts.entry(digits[i]).or_insert(1) += 1;
            } else {
                i = j;
            }
        }

        let mut result: Option<i32> = None;

        for (k, v) in &counts {
            if !self.strict && *v >= 2 {
                result = Some(*k);
                break;

            } else if self.strict && *v == 2 {
                result = Some(*k);
                break;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const PASSWORD_LEN: usize = 6;
    const RANGE_START: i32 = -1;
    const RANGE_END: i32 = -1;

    #[test]
    fn pwd_1() {
        let login = Login::new(PASSWORD_LEN, RANGE_START, RANGE_END, false);
        assert_eq!(true, login.is_valid(&"111111".to_string()));
    }

    #[test]
    fn pwd_2() {
        let login = Login::new(PASSWORD_LEN, RANGE_START, RANGE_END, false);
        assert_eq!(false, login.is_valid(&"223450".to_string()));
    }

    #[test]
    fn pwd_3() {
        let login = Login::new(PASSWORD_LEN, RANGE_START, RANGE_END, false);
        assert_eq!(false, login.is_valid(&"123789".to_string()));
    }

    #[test]
    fn pwd_4() {
        let login = Login::new(PASSWORD_LEN, RANGE_START, RANGE_END, true);
        assert_eq!(true, login.is_valid(&"112233".to_string()));
    }

    #[test]
    fn pwd_5() {
        let login = Login::new(PASSWORD_LEN, RANGE_START, RANGE_END, true);
        assert_eq!(false, login.is_valid(&"123444".to_string()));
    }

    #[test]
    fn pwd_6() {
        let login = Login::new(PASSWORD_LEN, RANGE_START, RANGE_END, true);
        assert_eq!(true, login.is_valid(&"111122".to_string()));
    }
}
