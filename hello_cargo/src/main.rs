use core::num;
use std::io::{self, Read};
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the Number!");
    let set_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is:{}" ,set_number);

    loop {
        println!("Please input your guess");
        
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 =  match guess.trim().parse() {
            Ok((num)) => num,
            Err(_) => continue,
        };
        
        println!("You guessed:{}",guess);
        
        match guess.cmp(&set_number){
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }


}
