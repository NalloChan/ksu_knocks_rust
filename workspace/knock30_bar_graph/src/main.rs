use std::io;

const DOT: &str = "*";

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Read error");
    let number: usize = buffer.trim().parse().expect("Parse error");

    println!("{}", DOT.repeat(number));
}
