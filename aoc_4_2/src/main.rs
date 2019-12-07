// AOC Day 4 part 2
// 2019-12-06
// Pattern detection - more
use common::security;

fn main() {
    let login = security::Login::new(6, 156218, 652527, true);

    let mut count = 0;
    for i in login.get_range_iter() {
        if login.is_valid(&i.to_string()) {
            count += 1;
        }
    }

    println!("Found {} password options", count);
}
