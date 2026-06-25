use std::io;
use rand::Rng;
use rand::RngExt;
use std::cmp::Ordering;

fn main() {
    println!("Hello,Welcome to guessing Game.");

    println!("Please Enter Your Input:");

    let mut rng = rand::rng();
    let number: i32 = rng.random_range(1..=100);


    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("sorry amigo!");

    let guess: i32 = guess.trim().parse().expect("Numero aqui:");

    match guess.cmp(&number){
        Ordering::Less => println!("Small"),
        Ordering::Greater => println!("Bigger"),
        Ordering::Equal => println!("Yo Boii!")
    }


}
//implemented from "The Rust Programming Language"
