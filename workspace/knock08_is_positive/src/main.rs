use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Read error");
    if buffer.trim().parse::<i32>().expect("Parse error") > 0 {
        println!("positive");
    }
}
