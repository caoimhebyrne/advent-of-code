use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut total_ids = 0;

    contents
        .lines()
        .map(|it| it.split(": "))
        .for_each(|mut section| {
            let game_id = section
                .next()
                .unwrap()
                .replace("Game ", "")
                .parse::<u32>()
                .unwrap();

            // 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            let subsets: Vec<HashMap<&str, u32>> = section
                .next()
                .unwrap()
                .split(';')
                .map(|it| {
                    it.split(", ")
                        // 3 blue
                        .map(|it| {
                            let mut amount_and_color = it.trim().split(' ');
                            let amount = amount_and_color.next().unwrap().parse::<u32>().unwrap();
                            let color = amount_and_color.next().unwrap();

                            (color, amount)
                        })
                        .collect()
                })
                .collect();

            let valid_sets: Vec<&HashMap<&str, u32>> = subsets
                .iter()
                .filter(|subset| {
                    let red = **subset.get("red").get_or_insert(&0);
                    let green = **subset.get("green").get_or_insert(&0);
                    let blue = **subset.get("blue").get_or_insert(&0);

                    red <= 12 && green <= 13 && blue <= 14
                })
                .collect();

            if valid_sets.len() == subsets.len() {
                total_ids += game_id;
            }
        });

    println!("Sum: {total_ids}");
}
