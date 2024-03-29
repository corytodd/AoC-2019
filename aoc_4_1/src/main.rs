// AOC Day 4 part 1
// 2019-12-06
// Pattern detection
use common::security;

fn main() {
    let login = security::Login::new(6, 156218, 652527, false);

    let mut count = 0;
    for i in login.get_range_iter() {
        if login.is_valid(&i.to_string()) {
            count += 1;
        }
    }

    println!("Found {} password options", count);
}
