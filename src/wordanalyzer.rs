use std::collections::HashMap;
use rand::{seq::IteratorRandom, prelude::SliceRandom};

pub struct WordAnalyzer {
    words: Vec<String>,
}

pub struct LettersManager {
    g_letters: Vec<(usize, char)>,
    y_letters: Vec<char>,
    b_letters: Vec<char>,
}

impl LettersManager {
    pub fn new() -> Self {
        Self {
            g_letters: Vec::new(),
            y_letters: Vec::new(),
            b_letters: Vec::new(),
        }
    }

    pub fn set_green(
        &mut self,
        g_letters: Vec<(usize, char)>,
    ) -> () {
        self.g_letters = g_letters;
    }

    pub fn set_yellow(&mut self, y_letters: Vec<char>) -> () {
        self.y_letters = y_letters;
    }

    pub fn set_black(&mut self, b_letters: Vec<char>) -> () {
        self.b_letters = b_letters;
    }

    pub fn get_green(&self) -> &Vec<(usize, char)> {
        &self.g_letters
    }

    pub fn get_yellow(&self) -> &Vec<char> {
        &self.y_letters
    }

    pub fn get_black(&self) -> &Vec<char> {
        &self.b_letters
    }
}

impl WordAnalyzer {
    pub fn new(wordlist: Vec<String>) -> Self {
        Self { words: wordlist }
    }

    pub fn filter_words(&mut self, letters: LettersManager) -> () {
        println!("Removing dead words...");

        self.words
            .retain(|x| WordAnalyzer::word_checker(x, &letters));
    }

    pub fn words_to_try(&self) -> Vec<&String> {
        let mut c: char = char::from_u32(self.words[0].as_bytes()[0] as u32).unwrap();
        let mut counter: u16 = 1;
        let mut highest: u16 = 1;
        let mut start: usize = 0;
        let mut slice: (usize, usize) = (0, 1);

        for word in self.words.iter().enumerate().skip(1) {
            if !word.1.starts_with(c) {
                if counter > highest {
                    highest = counter;
                    slice = (start, word.0 - 1);
                }
                start = word.0;
                counter = 1;
                c = char::from_u32(word.1.as_bytes()[0] as u32).unwrap();
            } else {
                counter += 1;
            }
        }

        let mut rng = rand::thread_rng();

        return self.words[slice.0..slice.1].choose_multiple(&mut rng, 5).collect();
        
    }

    fn word_checker(word: &String, letters: &LettersManager) -> bool {
        // Each guess produces five letters we know, so iteration here is negligible imo

        for letter in letters.get_green() {
            if char::from_u32(word.as_bytes()[letter.0] as u32).unwrap() != letter.1 {
                return false;
            }
        }

        for letter in letters.get_black() {
            if word.contains(*letter) {
                return false;
            }
        }

        for letter in letters.get_yellow() {
            if !word.contains(*letter) {
                return false;
            }
        }

        return true;
    }
}
