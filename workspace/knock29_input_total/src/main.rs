use std::io;

fn main() {
    let mut total = 0;
    for _ in 0..5 {
        total += get_input();
    }

    println!("{}", total);
}

fn get_input() -> i32 {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Read error");
    buffer.trim().parse().expect("Parse error")
}
