use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("[INFO]: Start guessing game.");

    // choose a number.
    let _answer = rand::thread_rng().gen_range(1..=100);
    println!("[INFO]: Secrect number: {}", _answer);

    loop {
        // input number.
        // notice the `mut` keyword
        println!("[INFO]: Input a number:");
        let mut _guess = String::new();

        io::stdin()
            .read_line(&mut _guess)
            .expect("Failed to readline.\n");

        // notice: dont forget .trim().
        let _i_guess: u32 = _guess.trim().parse().expect("Please input a number.");
        println!("[INFO]: You guess: {}", _i_guess);

        match _i_guess.cmp(&_answer) {
            Ordering::Greater => println!("[INFO]: a little big!"),
            Ordering::Less => println!("[INFO]: a little small!"),
            Ordering::Equal => {
                println!("[INFO]: Correct!");
                break;
            }
        }
    }
}
