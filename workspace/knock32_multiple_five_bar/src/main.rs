fn main() {
    for i in 1..=20 {
        let output = if i % 5 == 0 {
            "bar".to_string()
        } else {
            i.to_string()
        };

        println!("{}", output);
    }
}
