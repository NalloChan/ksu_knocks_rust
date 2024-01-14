use std::io;

fn main() {
    let array = [3, 7, 0, 8, 4, 1, 9, 6, 5, 2];

    let a = array[get_input()];
    let b = array[get_input()];

    println!("{}", a * b);
}

fn get_input() -> usize {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Read error");
    buffer.trim().parse().expect("Parse error")
}
