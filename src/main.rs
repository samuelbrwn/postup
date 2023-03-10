use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let txt = &args[2];
    fs::write(filename, txt)
        .expect("Expected to open file.");
}
