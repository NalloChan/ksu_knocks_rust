use std::io;

fn main() {
    while get_input_number() != 0 {}
}

fn get_input_number() -> i32 {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Read error");
    buffer.trim().parse().expect("Parse error")
}
