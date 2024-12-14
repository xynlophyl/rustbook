use std::io;

fn main() {
    let high:u32;

    loop{

        println!("provide a number to start fizzbuzz");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) if num > 0 => {
                high = num;
                break;
            },
            _ => continue,
        };
    }

    println!();
    for n in 1..high+1 {
        let mut res = String::new();
        if n%3 == 0 {
            res = res + "Fizz"
        }
        if n%5 == 0 {
            res = res + "Buzz"
        }
        if res == String::new() {
            res = n.to_string();
        }
        println!("{}: {}", n, res)
    }
}
    
