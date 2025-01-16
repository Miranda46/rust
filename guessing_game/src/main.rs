use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");
    loop {
        let mut guess = String::new();
        
        println!("Please enter your guess.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Nah, wrong");

        let guess : u32 = match guess.trim().parse() {  
            Ok(num) => num,
            Err(_) => {
                println!("Not a number"); 
                continue;
            },
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {println!("You won!"); break;},
            Ordering::Greater => println!("Too big!"),
        }
    }
    
}
