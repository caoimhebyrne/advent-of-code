// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
#[derive(Debug)]
pub struct Card {
    pub id: u32,
    pub expected_numbers: Vec<u32>,
    pub actual_numbers: Vec<u32>,
}

impl Card {
    pub fn parse(input: &str) -> Option<Self> {
        let (header, contents) = input.split_once(": ")?;
        let id = header.replace("Card ", "").trim().parse::<u32>().ok()?;

        let (winning_numbers_str, numbers_str) = contents.split_once(" | ")?;

        let expected_numbers: Vec<u32> = winning_numbers_str
            .split(' ')
            .flat_map(|it| it.parse::<u32>().ok())
            .collect();

        let actual_numbers: Vec<u32> = numbers_str
            .split(' ')
            .flat_map(|it| it.parse::<u32>().ok())
            .collect();

        Some(Self {
            id,
            expected_numbers,
            actual_numbers,
        })
    }

    pub fn score(&self) -> u32 {
        let winning_numbers = self
            .actual_numbers
            .iter()
            .filter(|it| self.expected_numbers.contains(it))
            .count();

        if winning_numbers == 0 {
            return 0;
        }

        // 1 for the first match..
        let mut score = 1;

        // ... and then doubled for each match after the first
        for _ in 1..winning_numbers {
            score *= 2;
        }

        score
    }
}
