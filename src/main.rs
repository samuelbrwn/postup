use std::env;
use std::io::Write;
use std::fs;
use chrono::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let utc: DateTime<Local> = Local::now();

    let filename = &args[1];
    let txt = &args[2];

    let msgcontent = format!("POSTD@{}:\n\t-{}\n\n", utc, txt);

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&filename)
        .expect("Unable to open.");

    file.write_all(msgcontent.as_bytes()).expect("write failed."); 
    

}
