use std::io;

const BASE_FARE: i32 = 610;
const BASE_DISTANCE: i32 = 1700;

const DISTANCE_FARE: i32 = 80;
const DISTANCE: f64 = 313.0;

fn main() {
    let mileage = get_input();

    let meters = i32::max(mileage - BASE_DISTANCE, 0);
    let mut value = BASE_FARE;

    let unit = meters as f64 / DISTANCE;
    value += DISTANCE_FARE * unit.ceil() as i32;

    println!("{}m, {}yen", mileage, value);
}

fn get_input() -> i32 {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Read error");
    buffer.trim().parse().expect("Parse error")
}
