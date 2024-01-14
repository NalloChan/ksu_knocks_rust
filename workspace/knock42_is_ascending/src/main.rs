use std::io;

fn main() {
    let array = vec![get_input(), get_input(), get_input()];
    let mut previous = array[0];
    for current in array {
        if previous > current {
            println!("NG");
            return;
        }

        previous = current;
    }

    println!("OK");
}

fn get_input() -> i32 {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Read error");
    buffer.trim().parse().expect("Parse error")
}
