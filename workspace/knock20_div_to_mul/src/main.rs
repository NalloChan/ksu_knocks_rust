use std::io;

fn main() {
    let a = get_input();
    let b = get_input();
    let div_ans = a / b;
    let ans = div_ans * b;
    println!("{}", ans);
}

fn get_input() -> i32 {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Read error");
    buffer.trim().parse().expect("Parse error")
}
