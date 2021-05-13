extern crate termion;

use std::io::{self, Write};
use termion::color;

fn main() {
    let mut color = "blue";
    let text = "Text";

    for c in text.chars() {
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

        io::stdout().flush().unwrap();
    }
}
