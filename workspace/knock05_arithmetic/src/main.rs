use std::{error, io};

fn main() {
    let a = input_number().map_err(|e| panic!("{}", e)).unwrap();
    let b = input_number().map_err(|e| panic!("{}", e)).unwrap();

    let add_ans = a + b;
    let sub_ans = a - b;
    let mul_ans = a * b;
    let div_ans = a / b;
    let remainder = a % b;

    println!("{} + {} = {}", a, b, add_ans);
    println!("{} - {} = {}", a, b, sub_ans);
    println!("{} * {} = {}", a, b, mul_ans);
    println!("{} / {} = {} ... {}", a, b, div_ans, remainder);
}

fn input_number() -> Result<i32, Box<dyn error::Error>> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().parse::<i32>()?)
}
