use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    //generating a secret number
    let secret_number = rand::thread_rng()
        .gen_range(1..=100);
    println!("secret number is {secret_number}");

    loop {
        //taking user's guess in input
        let mut guess = String::new();
        println!("Please input your guess!");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        //convert string input to u32 type
        let guess: u32 = match guess.trim().parse() {
            //.expect("please write a number");
            Ok(num) => num, // or Result::Ok(num) => num,
            Err(_) => { // or Result::Err(_) => {
                println!("Please input a number!");
                continue;
            },
        };
        println!("You guessed: {guess}");

        //matching guess status
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("guess is lower"),
            Ordering::Greater => println!("guess is higher"),
            Ordering::Equal => break println!("You win!"),
        }
    }
}
