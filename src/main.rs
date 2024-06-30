use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!\nPlease input your guess between 1 and 100.");
    let secret_num: u8 = rand::thread_rng().gen_range(1..=100);   

    loop {
        let mut guess: String = String::new();
        println!("Enter your guess: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too low!"),
            Ordering::Equal => {println!("You win!"); break},
            Ordering::Greater => println!("Too high!")
        }   
    }
}
