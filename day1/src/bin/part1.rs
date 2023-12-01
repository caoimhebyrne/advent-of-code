use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let total = contents
        .lines()
        .map(|line| {
            let numbers = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<Vec<char>>();

            let first = numbers.first().unwrap();
            let last = numbers.last().unwrap();

            format!("{}{}", first, last).parse::<u32>().unwrap()
        })
        .sum::<u32>();

    println!("Sum of all calibration values: {:?}", total);
}
