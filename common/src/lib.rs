pub mod computer;
pub mod electronics;
pub mod engine;
pub mod linalg;

use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};
use std::path::Path;

// Return each line from [file_name] assumed to be in "assets" directory
pub fn get_file_lines(file_name: String) -> Lines<BufReader<File>> {
    let cd = &std::env::current_dir().unwrap();
    let file_path = Path::new(cd).join("assets").join(file_name);
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    reader.lines()
}
