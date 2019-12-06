// AOC Day 4 part 1
// 2019-12-06
// Pattern detection
use common::security;

fn main() {
    let login = security::Login::new();

    let mut count = 0;
    for i in 156218..=652527 {
        if login.is_valid(&i.to_string()) {
            count += 1;
        }
    }

    println!("Found {} password options", count);

}
