use whoami;
use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        print!("{} $ ", whoami::username());
        stdout().flush().unwrap();
        let mut line = String::new();
        stdin().read_line(&mut line).expect("Failed to read line.");
        print!("{}", line);
    }
}
