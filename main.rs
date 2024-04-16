

////building a guessing game
use std::{cmp::Ordering, io};
use rand::Rng;
use colored::*;
fn main(){
    //we generate a random number/////////////////////////////
    let secret_number = rand::thread_rng().gen_range(1..101);
    //println!("the secret_number is {}", secret_number);///////////////////////////
    let mut attempts = 0;
    //print statement that prompt a user to input a number//////////////////////////
    while   attempts < 10{
        println!("KINDLY NOTE THAT: you have {} attempts left", 10-attempts);
    println!("guess a number");
    println!("please input your guess!");

    let mut guess: String = String::new();
    io::stdin().read_line(& mut guess).expect("Failed to readline");
    let  guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("please enter a valid number");
            continue;
        }
    };
    println!("you guessed the number:{}", guess);

// since the cargo.toml does not have the dependency on random numbers yet, we have to add it to the dependency session in cargo.toml//////////////
//we run cargo build to help build the rand crate and bring it into our scope //////////////////////////

// next we compare the guess number to the input number using the match expression///////////////////////
   match guess.cmp(&secret_number){
           Ordering::Less => println!("{}","Too small".red()),
           Ordering::Greater => println!("{}","Too big".red()),
           Ordering::Equal => {
           println!("{}","You win".green());
           break;
   }
         
}
attempts += 1;
         println!(".......................................");
}
   if attempts == 10{
    println!("{} {}", "sorry, you've run out of attempts. The secret number was:".red(), secret_number);
   }
}







