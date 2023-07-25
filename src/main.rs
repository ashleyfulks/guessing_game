use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;
use std::process::Command;

fn clear_console() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

fn main() {
    clear_console();
    println!("");
    println!("    +-------+");
    println!("   /       /|");
    println!("{}","  /   6   / |".green());
    println!(" +-------+  +");
    println!(" |       |  /  ");
    println!(" |       | /  ");
    println!(" +-------+  ");
    println!("");
    println!("{}","Guess the number! You have 10 tries.".green());
    println!("");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);

    let mut loop_count: u8 = 0;

    loop {

        if loop_count == 10 {
            println!("{}","You took too many guesses. You lose!".red());
            break;
        }

        if loop_count >= 1 {
            println!("Try again, you have {} guesses left.", 10 - loop_count);
        } else {
            println!("{}","Please input your guess and press enter.".yellow());
        }

        println!("");

        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
           
        println!(" ");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}","Too big!".blue()),
            Ordering::Equal => println!("{}","You win!".green()),
        }

        if guess == secret_number {
            println!("You took {} tries.", loop_count);
            break;
        }
        loop_count = loop_count + 1;
    }


}
