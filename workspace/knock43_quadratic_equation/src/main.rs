use std::io;

fn main() {
    let a = get_input();
    let b = get_input();
    let c = get_input();

    let d = b.pow(2) - 4 * a * c;

    match d {
        d if 0 < d => println!("実数解"),
        0 => println!("重解"),
        d if d < 0 => println!("虚数解"),
        _ => println!("error"),
    };
}

fn get_input() -> i32 {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Read error");
    buffer.trim().parse().expect("Parse error")
}
