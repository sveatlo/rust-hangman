pub struct Hangman {
    word: String,
    masked_word: String,
    guessed_letters: Vec<char>,
    used_guesses: i8,
    allowed_fails: i8,
    failed_guesses: i8,
}

impl Hangman {
    pub fn new(word: String, allowed_fails: i8) -> Hangman {
        Hangman {
            word: word.clone(),
            allowed_fails,
            used_guesses: 0,
            failed_guesses: 0,
            masked_word: "_".repeat(word.len()),
            guessed_letters: Vec::new(),
        }
    }

    pub fn play(&mut self) -> Result<i8, &'static str> {
        println!("Let's play!");

        loop {
            if self.failed_guesses >= self.allowed_fails {
                break Err("too many tries");
            }

            println!();
            println!("{}", self.masked_word);
            println!("Enter your guess: ");
            let mut guess_raw = String::new();
            std::io::stdin()
                .read_line(&mut guess_raw)
                .expect("failed to read from stdin");
            guess_raw = guess_raw.trim().to_string();
            // check input size
            if guess_raw.len() != 1 {
                println!("you must enter exactly one character per guess. try again");
                continue;
            }

            let guess = guess_raw
                .chars()
                .next()
                .expect("cannot get 0th char from guess");

            if self.guessed_letters.contains(&guess) {
                println!("already guessed. try again.");
                continue;
            }
            self.guessed_letters.push(guess);
            self.used_guesses += 1;

            if self.word.contains(guess) {
                self.mask_word();
            } else {
                self.failed_guesses += 1;
            }

            if self.masked_word == self.word {
                break Ok(self.used_guesses);
            }
        }
    }

    fn mask_word(&mut self) {
        self.masked_word = self.word.chars().fold(String::new(), |mut acc, c| {
            if self.guessed_letters.contains(&c) {
                acc.push(c);
            } else {
                acc.push('_');
            }

            acc
        });
    }
}
