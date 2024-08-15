use std::{cmp::Ordering, io};

use rand::Rng;


fn main() {
    println!("[INFO]: Start guessing game.");
    // input number.
    // notice the `mut` keyword
    println!("[INFO]: Input a number:");
    let mut _guess = String::new();
    
    // choose a number.
    let _answer = rand::thread_rng().gen_range(1..=100);
    println!("[INFO]: You pick: {}", _answer);
    
    io::stdin()
    .read_line(&mut _guess)
    .expect("Failed to readline.\n");

    // notice: dont forget .trim().
    let _i_guess: u32 = _guess.trim().parse().expect("Please input a number.");

    println!("[INFO]: You guess: {}", _guess);

    match _i_guess.cmp(&_answer) {
        Ordering::Equal => println!("[INFO]: Correct!"),
        Ordering::Less => println!("[INFO]: a little small!"),
        Ordering::Greater => println!("[INFO]: a little big!"),
    }
}
