use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    print!("Enter text: ");
    io::stdout().flush().unwrap();

    // User input
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .ok()
        .expect("Couldn't read line");

    let mut color_code = 31; // red
    loop {
        for c in text.chars() {
            if c == ' ' {
                print!(" ");
            } else {
                // Rainbow!
                print!("{}", color_text(&c.to_string(), color_code));
                if color_code != 35 {
                    color_code += 1;
                } else {
                    color_code = 31;
                }
            }

            io::stdout().flush().unwrap();
        }

        sleep(Duration::from_millis(100));
        print!("\x1B[2J\x1B[1;1H"); // clear terminal screen
    }
}

/// Return colored text
fn color_text(text: &str, color_code: i32) -> String {
    let colored_text = format!("\x1b[0;{}m{}{}", color_code, text, "\x1b[0m");
    return colored_text;
}
