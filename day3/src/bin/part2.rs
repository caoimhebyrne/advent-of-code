use day3::{has_neighboring_symbol, Parser, Position, Token, TokenType};
use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<Vec<Token>> = contents
        .lines()
        .map(|it| {
            let mut parser = Parser::new(it);
            parser.parse()
        })
        .collect();

    let mut gears_to_numbers: HashMap<(usize, Position), Vec<u32>> = HashMap::new();

    for (i, tokens) in lines.iter().enumerate() {
        for Token {
            token_type,
            position,
        } in tokens
        {
            if let TokenType::Number(value) = token_type {
                if let Some((
                    line_i,
                    Token {
                        position,
                        token_type: TokenType::Symbol('*'),
                    },
                )) = has_neighboring_symbol(&lines, i, position)
                {
                    gears_to_numbers
                        .entry((line_i, position))
                        .or_default()
                        .push(*value)
                }
            }
        }
    }

    let gear_ratios: Vec<u32> = gears_to_numbers
        .values()
        .filter(|it| it.len() == 2)
        .map(|values| values.iter().product::<u32>())
        .collect();

    println!("{}", gear_ratios.iter().sum::<u32>());
}
