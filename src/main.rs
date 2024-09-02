use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number between 1 and 100!");

    //generates random secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); //handles potential failure w/ result

        //compares secret number with guess number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Try again :)"),
            Ordering::Greater => println!("Too big! Try again :)"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}