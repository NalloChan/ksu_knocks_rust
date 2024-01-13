use std::io;

const SIZE: usize = 5;

fn main() {
    let mut array = [0; SIZE];
    for element in array.iter_mut() {
        *element = get_input();
    }
    println!("{:?}", array);
}

fn get_input() -> i32 {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Read error");
    buffer.trim().parse().expect("Parse error")
}
