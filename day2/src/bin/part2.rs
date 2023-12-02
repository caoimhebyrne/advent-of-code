use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut total_power = 0;

    contents
        .lines()
        .map(|it| it.split(": "))
        .for_each(|mut section| {
            let _ = section
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

            let mut min_map = HashMap::new();

            for subset in subsets {
                for (color, amount) in subset {
                    let min_amount = min_map.get(color).unwrap_or(&0);

                    if amount > *min_amount {
                        min_map.insert(color, amount);
                    }
                }
            }

            let min_red = *min_map.get("red").unwrap_or(&1);
            let min_green = *min_map.get("green").unwrap_or(&1);
            let min_blue = *min_map.get("blue").unwrap_or(&1);

            total_power += min_red * min_green * min_blue;
        });

    println!("Total Power: {total_power}");
}
