use whoami;
use std::env;
use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        print!("{}@{}:{} $ ", whoami::username(), whoami::hostname(), env::current_dir().unwrap().display());
        stdout().flush().unwrap();
        let mut line = String::new();
        stdin().read_line(&mut line).expect("Failed to read line.");
        print!("{}", line);
    }
}
