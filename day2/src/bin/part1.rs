use day2::Game;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let total_ids = contents
        .lines()
        .flat_map(|line| {
            let game = Game::parse(line.to_string()).unwrap();

            let valid_sets = game
                .sets
                .iter()
                .filter(|set| set.red <= 12 && set.green <= 13 && set.blue <= 14)
                .count();

            if valid_sets == game.sets.len() {
                Some(game.id)
            } else {
                None
            }
        })
        .sum::<u32>();

    println!("Sum: {total_ids}");
}
