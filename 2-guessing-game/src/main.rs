use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess");

    let  mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    // Appends to string instead of overwriting
    guess = "".to_string();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed: {guess}");

}
