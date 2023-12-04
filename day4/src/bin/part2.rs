use std::{collections::HashMap, fs};

use day4::Card;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let cards: Vec<Card> = contents.lines().flat_map(Card::parse).collect();
    let mut card_to_amount: HashMap<u32, u32> = cards.iter().map(|card| (card.id, 1)).collect();

    for card in &cards {
        // We pre-populated `card_to_amount` with 1 for each card id
        let amount = *card_to_amount.get(&card.id).unwrap();

        for i in 0..card.matching_numbers() {
            if let Some(card) = cards.get((card.id as usize) + i) {
                let entry = card_to_amount.entry(card.id).or_default();
                *entry += amount;
            }
        }
    }

    let total = card_to_amount.values().sum::<u32>();
    println!("Total: {total}");
}
