extern crate termion;

use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;
use termion::color;

fn main() {
    let mut color = "blue";

    print!("Enter text: ");
    io::stdout().flush().unwrap();

    // User input
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .ok()
        .expect("Couldn't read line");

    loop {
        for c in text.chars() {
            if c == ' ' {
                print!(" ");
            } else {
                // Rainbow!
                if color == "blue" {
                    print!("{}{}", color::Fg(color::Blue), c);
                    color = "green";
                } else if color == "green" {
                    print!("{}{}", color::Fg(color::Green), c);
                    color = "magenta";
                } else if color == "magenta" {
                    print!("{}{}", color::Fg(color::Magenta), c);
                    color = "red";
                } else if color == "red" {
                    print!("{}{}", color::Fg(color::Red), c);
                    color = "yellow";
                } else if color == "yellow" {
                    print!("{}{}", color::Fg(color::Yellow), c);
                    color = "blue";
                }
            }

            io::stdout().flush().unwrap();
        }

        sleep(Duration::from_millis(100));
        print!("\x1B[2J\x1B[1;1H"); // clear terminal screen
    }
}
