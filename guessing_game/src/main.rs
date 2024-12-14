use std::io; // use input/output library 
use std::cmp::Ordering;
use rand::Rng; // use Rng from rand library
// rust has a standard library (prelude) to define scope of program, anything else has to be imported with "use"

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100); //
    // println!("secret number: {}", secret_number);

    loop{

        println!("Please input your guess.");

        let mut guess = String::new(); // creates mutable string variable
        // ::new: associates new to string to define an empty string 
        // mut: can change the value throughout program (variables are immutable by default)

        io::stdin() // stdin is a function of the io library, :: is what is used to call it
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("type a number");
                continue;
            }
        };
        
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("small"),
            Ordering::Equal => {
                println!("correct!");
                break
            },
            Ordering::Greater => println!("large"),
        }

    }
}
