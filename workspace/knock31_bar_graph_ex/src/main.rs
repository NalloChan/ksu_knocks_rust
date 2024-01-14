use std::io;

const DOT: &str = "*";
const DIVISOR: usize = 5;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Read error");
    let number: usize = buffer.trim().parse().expect("Parse error");

    let symbol_with_space = format!("{} ", DOT.repeat(DIVISOR));
    let repeated_symbols = symbol_with_space.repeat(number / DIVISOR);
    let remaining_symbols = DOT.repeat(number % DIVISOR);

    println!("{}{}", repeated_symbols, remaining_symbols);
}
