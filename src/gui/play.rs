use std::io;
use std::io::prelude::*;

pub fn wait_for_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Make sure the prompt is printed
    io::stdin()
    .read_line(&mut input)
    .unwrap();
    input.trim().to_string()
}

pub fn play () {
    while(true) {
        let column_raw = wait_for_input("Enter a column (1-8) : ");
        if column_raw.eq("quit") {
            break;
        }

        if let Ok(column) = column_raw.trim().parse::<i32>() {
            if !(1 <= column &&  column <= 8) {
                println!("Invalid column entered (out of bounds).");
                continue;
            }

            println!("Column chosen : {}", column);
        } else {
            println!("Invalid column entered (bad chars).");
            continue
        }
    }
}