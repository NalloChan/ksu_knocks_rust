use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Read error");
    let number: i32 = buffer.trim().parse().expect("Parse error");

    match number {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        _ => println!("Other"),
    }
}
