use std::{
    collections::{BTreeMap, HashMap},
    fs,
};

// Example: "eightwothree" -> [8, 2, 3]
fn parse_numbers_from_string(string: &str) -> Vec<u32> {
    let word_to_number: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    // BTreeMap is used to keep the indices sorted
    let mut numbers: BTreeMap<u32, u32> = BTreeMap::new();

    word_to_number.iter().for_each(|(word, number)| {
        string.match_indices(word).for_each(|(i, _)| {
            numbers.insert(i as u32, *number);
        });

        string
            .match_indices(number.to_string().as_str())
            .for_each(|(i, _)| {
                numbers.insert(i as u32, *number);
            });
    });

    // TODO: Is there a way to avoid cloning here?
    numbers.values().cloned().collect::<Vec<u32>>()
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let sum = contents
        .lines()
        .map(|line| {
            let numbers = parse_numbers_from_string(line);
            let first = numbers.first().unwrap();
            let last = numbers.last().unwrap();

            format!("{}{}", first, last).parse::<u32>().unwrap()
        })
        .sum::<u32>();

    println!("Sum of all calibration values: {:?}", sum);
}
