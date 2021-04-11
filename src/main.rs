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

        let line_split: Vec<&str> = line.split(" ").collect();
        println!("(debug) {:?}", line_split);

        if line_split[0] == "exit" {
            return;
        } else if line_split[0] == "ls" {
            list_segment(Path::new("./"));
        } else if line_split[0] == "cd" {
            chage_directory(Path::new(line_split[1]));
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

fn chage_directory(p: &Path) {
    match env::set_current_dir(p) {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e)
    };
}