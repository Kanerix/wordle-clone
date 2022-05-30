use rand::seq::SliceRandom;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Write};

enum ErrorKind {
    WordNotInWordlist,
    WordNotFiveChars,
    GameIsOver,
}

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

impl Display for Guess {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output = Vec::with_capacity(5);
        for i in 0..5 {
            if self.green_chars[i] != '\0' {
                output.push(format!("\x1b[92m{}\x1b[0m", self.green_chars[i]));
            } else if self.yellow_chars[i] != '\0' {
                output.push(format!("\x1b[93m{}\x1b[0m", self.yellow_chars[i]));
            } else {
                output.push(format!("{}", self.word.chars().nth(i).unwrap()));
            }
        }

        let final_output: String = output.into_iter().collect();

        write!(fmt, "{}", final_output)
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
            history: Vec::with_capacity(6), 
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

    fn try_guess_word(&mut self, guess: &mut Guess) -> Result<(), ErrorKind> {
        if guess.word.len() != 5 {
            return Err(ErrorKind::WordNotFiveChars);
        }

        if !self.all_words.contains(&guess.word) {
            return Err(ErrorKind::WordNotInWordlist);
        }


        for c_in_guess in guess.word.chars() {
            let count_c_in_yellow = guess
                .yellow_chars
                .clone()
                .into_iter()
                .filter(|&c| c == c_in_guess)
                .count();

            let count_c_in_green = guess
                .green_chars
                .clone()
                .into_iter()
                .filter(|&c| c == c_in_guess)
                .count();

            let count_c_in_word = self.word.matches(c_in_guess).count();

            if self.word.contains(c_in_guess)
                && count_c_in_yellow <= count_c_in_word
                && count_c_in_green < count_c_in_word
            {
                guess.yellow_chars.push(c_in_guess);
            } else {
                guess.yellow_chars.push('\0');
            }
        }

        let mut words_zip = self.word.chars().zip(guess.word.chars());
        while let Some(words_zip) = words_zip.next() {
            if words_zip.0 == words_zip.1 {
                guess.green_chars.push(words_zip.1);
            } else {
                guess.green_chars.push('\0');
            }
        }

        // Minus 1 because of current guess
        if self.history.len() >= 5 {
            return Err(ErrorKind::GameIsOver);
        }

        if guess.word == self.word {
            self.word_is_guessed = true;
        }

        self.history.push(guess.word.clone());

        Ok(())
    }
}

fn main() {
    let mut wordle = Wordle::new();
    wordle.load_words_from_file();
    wordle.pick_random_word();

    println!(
        "
 __          __           _ _      
 \\ \\        / /          | | |     
  \\ \\  /\\  / /__  _ __ __| | | ___ 
   \\ \\/  \\/ / _ \\| '__/ _` | |/ _ \\
    \\  /\\  / (_) | | | (_| | |  __/
     \\/  \\/ \\___/|_|  \\__,_|_|\\___|
    "
    );

    while !wordle.word_is_guessed && !wordle.game_is_over {
        let output = format!("You have {} guesses left. Take a guess: ", 6 - wordle.history.len());
        stdout().write_all(output.as_bytes()).unwrap();
        stdout().flush().unwrap();

        let mut guess = Guess::new();
        stdin().read_line(&mut guess.word).unwrap();

        guess.word = guess.word.trim().to_string();

        match wordle.try_guess_word(&mut guess) {
            Ok(()) => {
                println!("{guess}");
            }
            Err(error) => match error {
                ErrorKind::WordNotFiveChars => println!("\x1b[91mThe word is not five characters\x1b[0m"),
                ErrorKind::WordNotInWordlist => println!("\x1b[91mThe word was not found in the wordlist\x1b[0m"),
                ErrorKind::GameIsOver => {
                    println!("{guess}");
                    wordle.game_is_over = true;
                }
            },
        };
    }

    if wordle.word_is_guessed {
        println!("You guessed the word \"{}\". Congrats!", wordle.word);
    } else {
        println!(
            "You didn't guess the word. The word was \"{}\".",
            wordle.word
        );
    }
}
