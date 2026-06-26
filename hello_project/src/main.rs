use std::io;
use rand::Rng;
use rand::RngExt;
use std::cmp::Ordering;

fn main() {
    println!("Hello,Welcome to guessing Game.");
    
loop{
    println!("Please Enter Your Input:");

    let mut rng = rand::rng();
    let number: u32 = rng.random_range(1..=100);

    println!("Numer is {number}");
   


    let mut guess = String::new();
    

    io::stdin()
        .read_line(&mut guess)
        .expect("sorry amigo!");

    let guess: u32 = match guess.trim().parse() 
    {
        Ok(num) => num,
        Err(_) => continue,
    };

    match guess.cmp(&number){
        Ordering::Less => println!("Small"),
        Ordering::Greater => println!("Bigger"),
        Ordering::Equal => {
            println!("You Win");
            break;
        }
    }
  }
}
//implemented from "The Rust Programming Language"
