use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {

    let secret = rand::thread_rng().gen_range(1..=52);

    println!("Secret number: {}", secret);

    println!("Guessing game!");

    loop {
        println!("Input your guess:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("fail");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}