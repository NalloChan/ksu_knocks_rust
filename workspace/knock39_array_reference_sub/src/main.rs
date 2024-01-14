fn main() {
    let array = [3, 7, 0, 8, 4, 1, 9, 6, 5, 2];

    for (index, element) in array.iter().enumerate() {
        let Some(number) = array.get(index + 1) else {
            break;
        };

        println!("{}", element - number);
    }
}
