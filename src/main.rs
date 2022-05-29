use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

struct Guess {
    word: String,
    yellow_chars: Vec<char>,
    green_chars: Vec<char>,
}

impl Guess {
    fn new() -> Guess {
        Guess {
            word: String::from(""),
            yellow_chars: Vec::with_capacity(5),
            green_chars: Vec::with_capacity(5),
        }
    }
}

struct Wordle {
    word: String,
    all_words: Vec<String>,
    history: Vec<String>,
    word_is_guessed: bool,
    game_is_over: bool,
}

impl Wordle {
    fn new() -> Wordle {
        Wordle {
            word: String::from(""),
            all_words: vec![],
            history: vec![],
            word_is_guessed: false,
            game_is_over: false,
        }
    }

    fn load_words_from_file(&mut self) {
        let file = File::open("words.txt").unwrap();

        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                self.all_words.push(line);
            }
        }
    }

    fn pick_random_word(&mut self) {
        let random_word = self
            .all_words
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone();
        self.word = random_word;
    }

    fn try_guess_word(&mut self, guess: &mut Guess) -> Result<(), &str> {
        self.history.push(guess.word.clone());

        // If the word is not 5 chars
        if guess.word.len() != 5 {
            println!("word: {}, len: {}", guess.word, guess.word.len());
            panic!("Word is not 5 characters");
        }

        // If the word is not in the wordlist
        if !self.all_words.contains(&guess.word) {
            panic!("Guess is not in the wordlist");
        }

        // If history is over 7, the game is over
        if self.history.len() > 6 {
            self.game_is_over = true;
            panic!("Hello");
        }

        // Yellow chars, the char is in the word ´but not´ in the right position
        for (i, c_in_guess) in guess.word.chars().enumerate() {
            if self.word.contains(c_in_guess) {
                guess.yellow_chars.insert(i, c_in_guess);
            }
        }

        // Green chars, the char is in the word ´and´ at the right position
        let mut words_zip = self.word.chars().zip(guess.word.chars());
        while let Some(words_zip) = words_zip.next() {
            if words_zip.0 == words_zip.1 {
                guess.green_chars.push(words_zip.1);
            } else {
                guess.green_chars.push('\0');
            }
        }

        Ok(())
    }
}

fn main() {
    let mut wordle = Wordle::new();
    wordle.load_words_from_file();
    wordle.pick_random_word();

    // Should loop when the game is on
    while wordle.history.len() < 6 && !wordle.word_is_guessed && !wordle.game_is_over {
        let mut guess = Guess::new();

        stdin().read_line(&mut guess.word).unwrap();

        guess.word = guess.word.trim().to_string();

        wordle.try_guess_word(&mut guess).unwrap();

        println!(
            "Word: {}, Guess: {}, Yellow: {:?}, Green: {:?}",
            wordle.word, guess.word, guess.yellow_chars, guess.green_chars
        );
    }
}
