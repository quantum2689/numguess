use core::num;
use std::io;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let number: i32 = rng.gen_range(1..=100); 
    
    loop {
        println!("guess number between 1-100");
        let mut input = String::new();
        println!("your guess is:");

        io::stdin().read_line(&mut input);

        let guess:i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) =>{
                println!("Please enter a valid number!");
                continue;
            }
        };

        if guess == number{
            println!("Congratulations! You guessed the correct number.");
            break;
        }
        else {
            if guess > number{
                println!("Sorry, that's not the correct number,you should guess LOWER. Please try again.")
            }
            if guess < number{
                println!("Sorry, that's not the correct number,you should guess HIGHER. Please try again.")
            }
        }
    }

}
