use std::io;

fn main() {
    println!("Guess The Number!!");

    println!("Please Enter The Number");

    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed To Read!");

    println!("Your Guessed Number is: {}", guess)

}
