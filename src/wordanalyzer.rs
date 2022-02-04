pub struct WordAnalyzer {
    words: Vec<String>,
    
}

pub struct LettersManager {
    g_letters: Vec<(usize, char)>,
    y_letters: Vec<char>,
    b_letters: Vec<char>,
}

impl LettersManager {
    pub fn new(greens: Vec<(usize, char)>, yellows: Vec<char>, blacks: Vec<char>) -> Self {
        Self {
            g_letters: greens,
            y_letters: yellows,
            b_letters: blacks
        }
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
        Self {
            words: wordlist,
        }
    }

    pub fn filter_words(self, letters: LettersManager) -> Self {
        let mut initial_words: Vec<String> = self.words;
        let lets: &LettersManager = &letters;
        initial_words = WordAnalyzer::green_letters(initial_words, lets);
        initial_words = WordAnalyzer::yellow_letters(initial_words, lets);

        return Self {
            words: initial_words
        }
    }

    fn green_letters(words: Vec<String>, letters: &LettersManager) -> Vec<String> {
        words.into_iter().filter(|x| WordAnalyzer::word_checker_green(x, letters)).collect()      
    }

    fn yellow_letters(words: Vec<String>, letters: &LettersManager) -> Vec<String> {
        words.into_iter().filter(|x| WordAnalyzer::word_checker_by(x, letters)).collect()
    }

    fn word_checker_green(word: &String, letters: &LettersManager) -> bool{
        for letter in letters.get_green() {
            if char::from_u32(word.as_bytes()[letter.0] as u32).unwrap() != letter.1 {
                return false;
            } 
        }
        return true;
    }

    fn word_checker_by(word: &String, letters: &LettersManager) -> bool {
        for letter in letters.get_black() {
            if word.contains(*letter) {
                return false;
            }
        }
        for letter in letters.get_yellow() {
            if !word.contains(*letter) {
                return false
            }
        }
        return true;
    }
}