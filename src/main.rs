use std::io::stdin;

fn main() {
    loop {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("Failed to read line.");
        print!("{}", line);
    }
}
