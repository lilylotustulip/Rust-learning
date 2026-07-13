use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess a number1");

    let secret_number = rand::thread_rng()
                        .gen_range(1..=100);

    loop{
        println!("please, input a guess.");

        let mut guess = String::new(); // this line creates a mutable variable assigned to an empty string.

        io::stdin()
            .read_line(&mut guess) // gets input from the user with .read_line method, and then stores the input into the guess variable.
            .expect("failed to read line"); //error handeling that returns a result as an ERR or OK.

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
                                
                            
            println!("you guessed, {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("bingo, u win!");
                break;
            }

        }
    }
}
