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

fn display_ascii_art() {
    let ascii_art = r#"

    ░██████╗░██╗░░░██╗███████╗░██████╗░██████╗██╗███╗░░██╗░██████╗░  ░██████╗░░█████╗░███╗░░░███╗███████╗
    ██╔════╝░██║░░░██║██╔════╝██╔════╝██╔════╝██║████╗░██║██╔════╝░  ██╔════╝░██╔══██╗████╗░████║██╔════╝
    ██║░░██╗░██║░░░██║█████╗░░╚█████╗░╚█████╗░██║██╔██╗██║██║░░██╗░  ██║░░██╗░███████║██╔████╔██║█████╗░░
    ██║░░╚██╗██║░░░██║██╔══╝░░░╚═══██╗░╚═══██╗██║██║╚████║██║░░╚██╗  ██║░░╚██╗██╔══██║██║╚██╔╝██║██╔══╝░░
    ╚██████╔╝╚██████╔╝███████╗██████╔╝██████╔╝██║██║░╚███║╚██████╔╝  ╚██████╔╝██║░░██║██║░╚═╝░██║███████╗
    ░╚═════╝░░╚═════╝░╚══════╝╚═════╝░╚═════╝░╚═╝╚═╝░░╚══╝░╚═════╝░  ░╚═════╝░╚═╝░░╚═╝╚═╝░░░░░╚═╝╚══════╝
    
    Written by Ashley Fulks 2023
    "#;

    println!("{}", ascii_art.bright_green());
}


fn display_ascii_art_you_win() {
    let ascii_art = r#"

██╗░░░██╗░█████╗░██╗░░░██╗  ░██╗░░░░░░░██╗██╗███╗░░██╗
╚██╗░██╔╝██╔══██╗██║░░░██║  ░██║░░██╗░░██║██║████╗░██║
░╚████╔╝░██║░░██║██║░░░██║  ░╚██╗████╗██╔╝██║██╔██╗██║
░░╚██╔╝░░██║░░██║██║░░░██║  ░░████╔═████║░██║██║╚████║
░░░██║░░░╚█████╔╝╚██████╔╝  ░░╚██╔╝░╚██╔╝░██║██║░╚███║
░░░╚═╝░░░░╚════╝░░╚═════╝░  ░░░╚═╝░░░╚═╝░░╚═╝╚═╝░░╚══╝
"#;

println!("{}", ascii_art.green());
}


fn display_ascii_art_you_lose() {
    let ascii_art = r#"
██████╗░██╗███╗░░██╗░██████╗░██╗░░░██╗░██████╗██╗
██╔══██╗██║████╗░██║██╔════╝░██║░░░██║██╔════╝██║
██║░░██║██║██╔██╗██║██║░░██╗░██║░░░██║╚█████╗░██║
██║░░██║██║██║╚████║██║░░╚██╗██║░░░██║░╚═══██╗╚═╝
██████╔╝██║██║░╚███║╚██████╔╝╚██████╔╝██████╔╝██╗
╚═════╝░╚═╝╚═╝░░╚══╝░╚═════╝░░╚═════╝░╚═════╝░╚═╝
"#;

println!("{}", ascii_art.red());
}

fn main() {
    clear_console();
    display_ascii_art();
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut loop_count: u8 = 0;

    loop {
        if loop_count == 10 {
            display_ascii_art_you_lose();
            println!("{}","You took too many guesses.".red());
            println!("Would you like to play again? (y/n)");
            let mut play_again = String::new();
            io::stdin().read_line(&mut play_again)
                .expect("Failed to read line");
            if play_again.trim() == "y" {
                loop_count = 0;
                clear_console();
                display_ascii_art();
                continue;
            } else {
                break;
            }
        }

        if loop_count >= 1 {
            println!("Try again, you have {} guesses left.", 10 - loop_count);
        } else {
            println!("{}","Please input your guess and press enter.".yellow());
        }

        println!("");

        let mut guess = String::new();
        let mut play_again: String = String::new();
    
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
            Ordering::Equal => display_ascii_art_you_win(),
        }

        if guess == secret_number {
            println!("You took {} tries.", loop_count);
            println!("Would you like to play again? (y/n)");
            io::stdin().read_line(&mut play_again)
                .expect("Failed to read line");
            if play_again.trim() == "y" {
                loop_count = 0;
                clear_console();
                display_ascii_art();
                continue;
            } else {
                break;
            }
            
        }
        loop_count = loop_count + 1;
    }


}
