extern crate termion;

use termion::color;

fn main() {
    println!("{}Red", color::Fg(color::Red));
    println!("{}Blue", color::Fg(color::Blue));
}
