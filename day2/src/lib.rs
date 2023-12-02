// Game X: X blue, X red; X red, X green, X blue; X green
pub struct Game {
    pub id: u32,
    pub sets: Vec<Set>,
}

impl Game {
    pub fn parse(input: String) -> Option<Self> {
        let mut header_and_content = input.split(": ");

        let id = header_and_content
            .next()?
            .replace("Game ", "")
            .trim()
            .parse::<u32>()
            .ok()?;

        let sets = header_and_content
            .next()?
            .split("; ")
            .flat_map(Set::parse)
            .collect();

        Some(Self { id, sets })
    }
}

// X red, X green, X blue;
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct Set {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Set {
    pub fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    pub fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }

    // X red, X green, X blue
    pub fn parse(input: &str) -> Option<Self> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for color_and_amount in input.split(", ") {
            let mut color_and_amount = color_and_amount.split(' ');

            let amount = color_and_amount.next()?.parse::<u32>().ok()?;
            let color = color_and_amount.next()?;

            match color {
                "red" => red = amount,
                "green" => green = amount,
                "blue" => blue = amount,

                // Invalid color
                _ => return None,
            }
        }

        Some(Self { red, green, blue })
    }
}

impl Default for Set {
    fn default() -> Self {
        Self::new()
    }
}
