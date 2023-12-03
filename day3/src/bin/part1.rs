use day3::{has_neighboring_symbol, Parser, Token, TokenType};
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<Vec<Token>> = contents
        .lines()
        .map(|it| {
            let mut parser = Parser::new(it);
            parser.parse()
        })
        .collect();

    let mut valid_numbers: Vec<u32> = vec![];

    for (i, tokens) in lines.iter().enumerate() {
        for Token {
            token_type,
            position,
        } in tokens
        {
            if let TokenType::Number(value) = token_type {
                if has_neighboring_symbol(&lines, i, position).is_some() {
                    valid_numbers.push(*value);
                }
            }
        }
    }

    let sum: u32 = valid_numbers.iter().sum();
    println!("Sum: {sum}")
}
