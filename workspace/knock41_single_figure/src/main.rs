use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Read error");
    let number: i32 = buffer.trim().parse().expect("Parse error");

    if (1..=9).contains(&number) {
        println!("single figure");
    } else {
        println!("not single figure");
    }
}
