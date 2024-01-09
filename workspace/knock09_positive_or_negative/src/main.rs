use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Read error");
    match buffer.trim().parse::<i32>().expect("Parse error") {
        0 => println!("zero"),
        1.. => println!("positive"),
        ..=-1 => println!("negative"),
    };
}
