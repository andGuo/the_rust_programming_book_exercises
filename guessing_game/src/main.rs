use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Number guessing game!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is {}", secret_number);
    
    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // type case string to int
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("INVALID: Enter a number!");
                continue
            },
        };
        
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Guess higher"),
            Ordering::Greater => println!("Guess lower"),
            Ordering::Equal => { println!("You won!"); break; }
        };
    }
}
