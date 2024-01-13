use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Read error");
    let number: i32 = buffer.trim().parse().expect("Parse error");

    if number < -10 {
        println!("1");
    } else if (-10..0).contains(&number) {
        println!("2");
    } else if 0 <= number {
        println!("3");
    }
}
