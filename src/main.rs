#![allow(dead_code, unused_imports, unused_variables)]
use std::any::Any;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

use crate::wordanalyzer::LettersManager;

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

    let mut lexicon: wordanalyzer::WordAnalyzer = wordanalyzer::WordAnalyzer::new(read);

    println!("Welcome to my Wordle Solver!\nI appreciate you using my program :)\nPlease input the word \"weary\" and record the result.");

    let mut letters: LettersManager = expect_input();
    
    lexicon.filter_words(letters);

    println!("Please put in the word \"pious\" next.");

    letters = expect_input();

    lexicon.filter_words(letters);

    println!("Finding optimal words to guess...");

    let possible_words: Vec<&String> = lexicon.words_to_try();
    
    for word in possible_words {
        print!("{} ", word);
    }

}

fn expect_input() -> LettersManager{
    let mut found_letters: LettersManager = wordanalyzer::LettersManager::new();

    println!("Please put in any green letters in their place.\n Put a _ for any placeholders");

    let mut letters = handle_input();

    while letters.len() != 5 {
        println!("Input needs to be 5 letters. Length was {}: {}", letters.len(), letters);
        letters = handle_input();
    }

    found_letters.set_green(green_input(&letters));

    println!("Please put in any yellow letters. (Order doesn't matter)");

    letters = handle_input();

    found_letters.set_yellow(other_input(&letters));

    println!("Please input all greyed out letters.");

    letters = handle_input();

    found_letters.set_black(other_input(&letters));

    found_letters
}

fn handle_input() -> String {
    let mut letters: String = String::new();

    io::stdin().read_line(&mut letters).expect("Failed to read line");

    letters.trim_end().to_string()
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
