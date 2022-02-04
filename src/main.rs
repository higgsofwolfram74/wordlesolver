#![allow(dead_code, unused_imports)]
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

mod wordanalyzer;

fn main() {
    let path: &str = "./fiveletterwords.txt";
    let file = File::open(path)
                    .unwrap_or_else(|_| panic!("File not found at {}, please recheck your path.", path));

    let reader = BufReader::new(file);

    let read: Vec<String> = reader.lines().map(|x| x.unwrap().trim_end().to_string()).collect();

    println!("{}", read[0]);
}
