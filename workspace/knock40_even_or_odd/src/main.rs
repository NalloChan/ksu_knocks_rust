use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Read error");
    let number: i32 = buffer.trim().parse().expect("Parse error");

    if number % 2 == 0 {
        println!("even");
    } else {
        println!("odd");
    }
}
