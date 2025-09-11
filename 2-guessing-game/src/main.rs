use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("Guess the number!");

    let secret_number: u16 = rand::random_range(1..=100);

    loop {
        let mut guess = String::new();
        print!("Please input your guess: ");
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u16 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Enter a valid number, {:?}", err.to_string());
                continue;
            }
        };

        // println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}
