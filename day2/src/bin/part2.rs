use day2::{Game, Set};
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let total_power: u32 = contents
        .lines()
        .map(|line| {
            let game = Game::parse(line.to_string()).unwrap();
            let mut min_set = Set::new();

            for set in game.sets {
                if set.red > min_set.red {
                    min_set.red = set.red;
                }

                if set.green > min_set.green {
                    min_set.green = set.green;
                }

                if set.blue > min_set.blue {
                    min_set.blue = set.blue;
                }
            }

            min_set.power()
        })
        .sum();

    println!("Total Power: {total_power}");
}
