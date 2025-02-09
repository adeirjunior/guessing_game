use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    loop {
        println!("Menu:");
        println!("1. Start Game");
        println!("2. Exit");
        
        let choice = get_input();
        
        match choice.as_str() {
            "1" => start_game(),
            "2" => {
                println!("Exiting...");
                break;
            },
            _ => println!("Invalid option, please choose again."),
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn start_game() {
    println!("Guess the number!");
    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);
    let mut attempts: u8 = 5;

    while attempts > 0 {
        println!("You have {} attempts remaining.", attempts);
        let guess = match get_input().parse::<u8>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                return;
            }
        }
        attempts -= 1;
    }
    println!("You've run out of attempts! The secret number was {}.", secret_number);
}
