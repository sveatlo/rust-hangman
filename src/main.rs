mod game;

use rand::prelude::SliceRandom;

fn main() {
    let dictionary = vec![
        "voice",
        "remove",
        "two",
        "fry",
        "switch",
        "flame",
        "awake",
        "unknown",
        "gather",
        "coordinated",
        "cloudy",
        "imagine",
    ];
    let word = dictionary
        .choose(&mut rand::thread_rng())
        .expect("no word can be chosen");

    let mut g = game::Hangman::new(word.to_string(), 10);

    let game_result = g.play();

    println!();
    println!("{}", word);
    match game_result {
        Ok(n) => println!("You won in {} tries", n),
        Err(x) => println!("You lost! {}", x),
    }
}
