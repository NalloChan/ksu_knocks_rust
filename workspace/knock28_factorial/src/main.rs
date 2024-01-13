use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Read error");
    let number: i32 = buffer.trim().parse().expect("Parse error");

    let factorial = factorial(number);
    println!("{}", factorial);
}

fn factorial(number: i32) -> i32 {
    if number <= 1 {
        return 1;
    }

    number * factorial(number - 1)
}
