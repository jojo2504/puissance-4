use std::{io, process};
use std::io::prelude::*;

use crate::api::engine::Game;

pub fn input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Make sure the prompt is printed
    io::stdin()
    .read_line(&mut input)
    .unwrap();
    input.trim().to_string()
}

pub fn play (game: &Game) -> i32 {
    loop {
        let column_raw = input("Enter a column (1-8) : ");
        if column_raw.eq("quit") {
            process::exit(0x0100);
        }

        if let Ok(column) = column_raw.trim().parse::<i32>() {
            if !(1 <= column &&  column <= game.board.width) {
                println!("Invalid column entered (out of bounds).");
                continue;
            }
            println!("Column chosen : {}", column);
            return column;

        } else {
            println!("Invalid column entered (bad chars).");
            continue;
        }
    }
}