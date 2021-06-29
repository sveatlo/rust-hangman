use rand::prelude::SliceRandom;

mod game;

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

    let mut g = game::Guesser::new(word.to_string(), 10);

    let game_result = g.play();

    println!();
    match game_result {
        Ok(n) => println!("You won in {} tries. The word was *{}*", n, word),
        Err(x) => println!("You lost! {}", x),
    }
}
