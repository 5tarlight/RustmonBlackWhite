use std::io::{self, Write};

use ansi_term::Color;

pub fn read_line(help: &str) -> String {
    if help.len() > 0 {
        print!("{}", Color::Yellow.paint(help));
        io::stdout().flush().unwrap();
    }

    let mut line = String::new();
    if let Ok(_) = io::stdin().read_line(&mut line) {
        return String::from(line.trim());
    } else {
        // TODO : Something wrong happens while reading input from terminal
        // TODO : Log and desukete!!!
        return String::new();
    }
}

pub fn println(str: String) {
    println!("{}", str);
}

pub fn print(str: String) {
    print!("{}", str);
    io::stdout().flush().unwrap();
}
