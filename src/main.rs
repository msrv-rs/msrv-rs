use std::io::BufRead;
use std::io::Write;

fn handle_line(line: &str) {
    if line.contains("adler32 1.0.4") {
        println!("false");
    } else {
        println!("true");
    }
    std::io::stdout().flush().unwrap();
}

fn main() {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    for line in stdin.lines() {
        handle_line(&line.unwrap());
    }
}
