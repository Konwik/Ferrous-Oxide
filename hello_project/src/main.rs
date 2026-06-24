use std::io;
fn main() {
    println!("Hello,Welcome to guessing Game.");

    println!("Please Enter Your Input:");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("sorry amigo!");

    println!("You Guessed {guess}")
}
