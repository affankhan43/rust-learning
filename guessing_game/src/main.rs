use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess The Number!!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..101);

    println!("Secret Number Is {}", secret_number);
    loop{
        println!("Please Enter The Number");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed To Read!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //.expect("Please Type Number");

        println!("Your Guessed Number is: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}","Correct Guess wow!!".green());
                break;
            },
            Ordering::Less => println!("{}","Too Small".red()),
            Ordering::Greater => println!("{}","Too High".red())
        }
    }

}
