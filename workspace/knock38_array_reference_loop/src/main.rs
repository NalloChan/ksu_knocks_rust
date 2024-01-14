fn main() {
    let array = [3, 7, 0, 8, 4, 1, 9, 6, 5, 2];
    let mut index = 0usize;

    for _ in 0..10 {
        let number = array[index];
        index = number;
        println!("{}", number);
    }
}
