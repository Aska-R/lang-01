use std::iter::{self, from_fn};

pub enum Token {
    Number(i64),
    String(String),
    Other(String),
    Bool(bool),
    Plus,
    Dash,
    Star,
    Slash,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Dot,
    Semicolon,
    Equal,
    While,
    For,
    Repeat,
    If,
    Elseif,
    Else,
    Eof,
}

pub struct Tokens {
    pub token: Token,
    pub line: u64 // currently just implementing line number for errors, may include character number later
}

impl Tokens {
    pub fn new(token: Token, line: u64) -> Self {
        Self { token, line }
    }
}


pub fn tokenizer(input: String) -> Vec<Tokens> {
    let mut tokens: Vec<Tokens> = Vec::new();
    let mut iter = input.chars().peekable();
    let mut line: u64 = 1;

    while let Some(ch) = iter.next() {
        // pattern matching logic
        match ch {
            ch if ch.is_whitespace() => continue,
            '\n' => line += 1,
            '(' => tokens.push(Tokens::new(Token::LeftParen, line)),
            ')' => tokens.push(Tokens::new(Token::RightParen, line)),
            '{' => tokens.push(Tokens::new(Token::LeftBracket, line)),
            '}' => tokens.push(Tokens::new(Token::RightBracket, line)),
            '+' => tokens.push(Tokens::new(Token::Plus, line)),
            '-' => tokens.push(Tokens::new(Token::Dash, line)),
            '*' => tokens.push(Tokens::new(Token::Star, line)),
            '/' => tokens.push(Tokens::new(Token::Slash, line)),
            '1'..='9' => {
                let n: i64 = iter::once(ch)
                    .chain(
                        from_fn(
                            || iter.by_ref().next_if(|s| s.is_ascii_digit())
                        )
                    )
                    .collect::<String>()
                    .parse()
                    .unwrap();

                tokens.push(Tokens::new(Token::Number(n), line));
            },
            '"' => {
                let s: String = iter::once(ch)
                    .chain(
                        from_fn(
                            || iter.by_ref().next_if(|s| *s != '"')
                        )
                    )
                    .collect::<String>();


                // Cuts off the first character because im stupid and don't understand how iterators work
                let mut s = s.chars();
                s.next();
                let s = s.as_str().to_string();

                tokens.push(Tokens::new(Token::String(s), line));

                // Moves iter to next location because again im stupid and don't understand how iterators work
                iter.next();
            },
            '。' | '.' => tokens.push(Tokens::new(Token::Dot, line)),
            ';' | '；' => tokens.push(Tokens::new(Token::Semicolon, line)),
            '=' => tokens.push(Tokens::new(Token::Equal, line)),

            _ => {
                // All multicharacter tokens fall under here
                let s: String = iter::once(ch)
                .chain(
                    from_fn(
                        || iter.by_ref().next_if(|s| !is_reserved(*s))
                    )
                )
                .collect::<String>();

                match s.as_str() {
                    "true" => {
                        tokens.push(Tokens::new(Token::Bool(true), line));
                    }
                    "false" => {
                        tokens.push(Tokens::new(Token::Bool(false), line));
                    }
                    "while" => {
                        tokens.push(Tokens::new(Token::While, line));
                    }
                    "repeat" => {
                        tokens.push(Tokens::new(Token::Repeat, line));
                    }
                    "if" => {
                        todo!();
                    }
                    "else" => {
                        // if else would be implemented here
                        todo!();
                    }
                    
                    _ => {
                        tokens.push(Tokens::new(Token::Other(s), line));
                    }
                }
            }
        }
    }

    tokens.push(Tokens::new(Token::Eof, line));

    tokens
}

fn is_reserved(ch: char) -> bool {
    matches!(ch, '(' | ')' | '+' | '-' | '*' | '/' | '"' | ' ' | '\n' | '.' | '；' | '。' | '、' | '”' | '’' | '{' | '}')
}

pub fn dump_tokens(tokens: Vec<Token>) {
    for token in tokens {
        match token {
            Token::Number(token_value) => println!("Number: {}", token_value),
            Token::String(token_value) => println!("String: {}", token_value),
            Token::Other(token_value) => println!("Other: {}", token_value),
            Token::Bool(token_value) => println!("Bool: {}", token_value),
            Token::Dash => println!("-"),
            Token::Eof => println!("End of file"),
            Token::LeftParen => println!("("),
            Token::RightParen => println!(")"),
            Token::Plus => println!("+"),
            Token::Slash => println!("/"),
            Token::Star => println!("*"),
            Token::Dot => println!("."),
            Token::Semicolon => println!(";"),
            Token::Equal => println!("="),
            Token::LeftBracket => println!("{{"),
            Token::RightBracket => println!("}}"),
            Token::While => todo!(),
            Token::For => todo!(),
            Token::Repeat => todo!(),
            Token::If => todo!(),
            Token::Elseif => todo!(),
            Token::Else => todo!(),
        }
    }
}