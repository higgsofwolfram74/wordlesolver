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

    pub fn set(
        &mut self,
        g_letters: Vec<(usize, char)>,
        y_letters: Vec<char>,
        b_letters: Vec<char>,
    ) -> () {
        self.g_letters = g_letters;
        self.y_letters = y_letters;
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
        self.words
            .retain(|x| WordAnalyzer::word_checker(x, &letters));
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
