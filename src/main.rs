use std::io;

use rand::Rng;

fn main() {
    let max = 100000;
    let secret_number = rand::thread_rng().gen_range(1..=max);

    println!("Guess the number!");
    let mut left_bound = 0;
    let mut right_bound = max;
    let mut mid_point;

    loop {
        eprint!("Please input your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => {
                left_bound = guess;
                mid_point = (left_bound + right_bound) / 2;
                println!("Too small! {left_bound:?} <- [{mid_point:?}] -> {right_bound:?}");
            }
            std::cmp::Ordering::Greater => {
                right_bound = guess;
                mid_point = (left_bound + right_bound) / 2;
                println!("Too Large! {left_bound:?} <- [{mid_point:?}] -> {right_bound:?}");
            }
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
