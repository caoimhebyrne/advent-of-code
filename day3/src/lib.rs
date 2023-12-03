#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub start: usize,
    pub end: usize,
}

impl Position {
    pub fn new_single(column: usize) -> Self {
        Self {
            start: column,
            end: column,
        }
    }

    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Symbol(char),
    Number(u32),
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub token_type: TokenType,
    pub position: Position,
}

impl Token {
    pub fn new(token_type: TokenType, position: Position) -> Self {
        Self {
            token_type,
            position,
        }
    }
}

#[derive(Debug)]
pub struct Stream<T> {
    elements: Vec<T>,
    index: usize,
}

impl<T: Clone> Stream<T> {
    pub fn new(elements: Vec<T>) -> Self {
        Stream { elements, index: 0 }
    }

    pub fn peek(&self) -> Option<T> {
        self.elements.get(self.index).cloned()
    }

    pub fn consume(&mut self) -> Option<T> {
        let element = self.peek();

        if self.index < self.elements.len() {
            self.index += 1;
        }

        element
    }

    pub fn unconsume(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }
}

#[derive(Debug)]
pub struct Parser {
    stream: Stream<char>,
}

impl Parser {
    pub fn new(contents: &str) -> Self {
        Self {
            stream: Stream::new(contents.chars().collect()),
        }
    }

    pub fn parse(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        while let Some(char) = self.stream.consume() {
            let token = match char {
                '0'..='9' => {
                    let start = self.stream.index;
                    let mut chars = vec![char];

                    while let Some(char) = self.stream.peek() {
                        if char.is_numeric() {
                            self.stream.consume();
                            chars.push(char);
                        } else {
                            break;
                        }
                    }

                    let end = self.stream.index;

                    Token::new(
                        TokenType::Number(chars.iter().collect::<String>().parse::<u32>().unwrap()),
                        Position::new(start, end),
                    )
                }
                _ => {
                    if char != '.' {
                        Token::new(
                            TokenType::Symbol(char),
                            Position::new_single(self.stream.index),
                        )
                    } else {
                        continue;
                    }
                }
            };

            tokens.push(token);
        }

        tokens
    }
}

pub fn has_neighboring_symbol(lines: &[Vec<Token>], line_i: usize, position: &Position) -> bool {
    let min_line = line_i.checked_sub(1).unwrap_or(line_i);
    let max_line = line_i + 1;

    for index in min_line..(max_line + 1) {
        if let Some(tokens) = lines.get(index) {
            for token in tokens {
                if let Token {
                    token_type: TokenType::Symbol(_),
                    position: symbol_position,
                } = token
                {
                    let min_start = position.start.checked_sub(1).unwrap_or(position.start);
                    let max_end = position.end + 1;

                    if symbol_position.start >= min_start && symbol_position.end <= max_end {
                        return true;
                    }
                }
            }
        }
    }

    false
}
