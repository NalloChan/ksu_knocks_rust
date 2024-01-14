use std::{fmt::Debug, io, str::FromStr};

struct Jpy(i32);

struct Usd(f64);

impl Usd {
    fn from_jpy(jpy: Jpy, rate: f64) -> Self {
        Usd((jpy.0 as f64) / rate)
    }
}

impl Debug for Usd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dollars = self.0.floor() as i32;
        let cents = (self.0.fract() * 100.0).floor() as i32;
        write!(f, "{} {}", dollars, cents)
    }
}

fn main() {
    let jpy = Jpy(get_input());
    let rate = get_input();
    let usd = Usd::from_jpy(jpy, rate);

    println!("{:?}", usd);
}

fn get_input<T>() -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Read error");
    buffer.trim().parse().expect("Parse error")
}
