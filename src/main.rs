use whoami;
use std::io::{stdin, stdout, Write};
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    loop {
        let cwd = env::current_dir().unwrap();
        print!("{}@{}:{} $ ", whoami::username(), whoami::hostname(), cwd.display());
        stdout().flush().unwrap();

        let mut line = String::new();
        stdin().read_line(&mut line).expect("Failed to read line.");
        line.remove(line.len() - 1);
        println!("(debug) {}", line);

        if line == "exit" {
            return;
        } else if line == "ls" {
            list_segment(Path::new("./"))
        }
    }
}

fn list_segment(p: &Path) {
    match fs::read_dir(p) {
        Err(e) => eprintln!("{}", e),
        Ok(paths) => for path in paths {
            println!("{}", path.unwrap().path().display());
        }
    
    }
}