use std::fs;

use day4::Card;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let score: u32 = contents
        .lines()
        .flat_map(Card::parse)
        .map(|it| it.score())
        .sum();

    println!("Total score: {score}");
}
