#![allow(dead_code, unused_imports, unused_variables)]
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

mod wordanalyzer;
//TODO: When given a yellow letter, we know where that letter isn't. We could filter that too.

fn main() {
    let path: &str = "./fiveletterwords.txt";
    let file = File::open(path)
        .unwrap_or_else(|_| panic!("File not found at {}, please recheck your path.", path));

    let reader = BufReader::new(file);

    let read: Vec<String> = reader
        .lines()
        .map(|x| x.unwrap().trim_end().to_string())
        .collect();

    let lexicon: wordanalyzer::WordAnalyzer = wordanalyzer::WordAnalyzer::new(read);

    println!("Welcome to my Wordle Solver!\nI appreciate you using my program :)\nPlease input the word \"weary\" and record the result.");

    println!("Of the word weary, please put in any green letters in their place.\n Put a _ for any placeholders");

    let mut letters = String::new();

    io::stdin()
        .read_line(&mut letters)
        .expect("Failed to read line.");

    while letters.len() != 5 {
        println!()
    }
}

fn green_input(inp: &String) -> Vec<(usize, char)> {
    let mut result: Vec<(usize, char)> = Vec::new();
    for elem in inp.char_indices() {
        if elem.1.is_alphabetic() && elem.1.is_ascii() {
            result.push(elem);
        }
    }
    result
}

fn other_input(inp: &String) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    for letter in inp.chars() {
        if letter.is_alphabetic() && letter.is_ascii() {
            result.push(letter);
        }
    }
    result
}
