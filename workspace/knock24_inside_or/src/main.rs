use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Read error");
    let number: i32 = buffer.trim().parse().expect("Parse error");

    if (-10..0).contains(&number) || 10 <= number {
        println!("OK");
    } else {
        println!("NG");
    }
}
