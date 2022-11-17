use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("*** Guess it please! ***");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("Secret: {}", secret_number);
    loop {
        println!("\t -------------------\nPls enter a no: ");
        let mut guess_number = String::new();
        std::io::stdin().read_line(&mut guess_number).expect("Failed big time");
        // let guess_number: u32 = guess_number.trim().parse().expect("I said: Please enter a no");
        let guess_number: u32 = match guess_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        println!("Guessed: {}", guess_number);
        match guess_number.cmp(&secret_number) {
            Ordering::Equal => { 
                println!("Bravo!");
                break;
            },
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small")
        }
    }
}
