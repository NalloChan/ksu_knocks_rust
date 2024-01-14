use std::io;

fn main() {
    let array = [3, 7, 0, 8, 4, 1, 9, 6, 5, 2];

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Read error");
    let number: usize = buffer.trim().parse().expect("Parse error");

    println!("{}", array[number])
}
