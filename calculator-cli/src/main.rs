use std::io::{self, Write};
use std::process::exit;

use calculator::calculate;

fn main() {
    println!("Welcome to our calculator! You can put \"quit\" to quit.");
    loop {
        print!(">>> ");
        io::stdout().flush().expect("Failed to flush!");

        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line!");
        line = line.trim().to_string();

        match line.as_str() {
            "" => continue,
            "q" | "quit" | "exit" => exit(0),
            expr => {
                match calculate(expr) {
                    Ok(n) => println!("{}", n),
                    Err(e) => eprintln!("{}", e)
                }
            }
        }

        io::stdout().flush().expect("Failed to flush!");
    }
}
