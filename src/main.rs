extern crate regex;

use regex::Regex;

fn main() {
    let mut lexer = Lexer::new("1+1+(1+1)");

    let mut tokens = Vec::new();
    while let Some(tok) = lexer.next_token() {
        tokens.push(tok);
    }
    tokens.push(Token::EOF);

    println!("{:?}", tokens);
}

#[derive(Debug)]
enum Operators {
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Debug)]
enum Token {
    Operator(Operators),
    Digit(i32),
    OpenBracket,
    ClosedBracket,
    EOF,
}

fn operator(input: &str) -> Option<(Token, usize)> {
    match input.chars().next().unwrap() {
        '+' => Some((Token::Operator(Operators::Plus), 1)),
        '-' => Some((Token::Operator(Operators::Minus), 1)),
        '*' => Some((Token::Operator(Operators::Star), 1)),
        '/' => Some((Token::Operator(Operators::Slash), 1)),
        _ => None,
    }
}

fn digit(input: &str) -> Option<(Token, usize)> {
    let re = Regex::new("^\\d+").unwrap();
    if let Some(matches) = re.captures(input) {
        let digit_str = matches.at(0).unwrap();
        let digit = digit_str.parse::<i32>().unwrap();
        return Some((Token::Digit(digit), digit_str.len()));
    }
    None
}

fn open_bracket(input: &str) -> Option<(Token, usize)> {
    match input.chars().next().unwrap() {
        '(' => Some((Token::OpenBracket, 1)),
        _ => None,
    }
}

fn closed_bracket(input: &str) -> Option<(Token, usize)> {
    match input.chars().next().unwrap() {
        ')' => Some((Token::ClosedBracket, 1)),
        _ => None,
    }
}

struct Lexer<'a> {
    input: &'a str,
    pos: usize, // lexemes: Vec<fn(&str) -> Option<(Token, usize)>>
}

impl<'a> Lexer<'a> {
    fn new(input: &str) -> Lexer {
        Lexer {
            input: input,
            pos: 0, // lexemes: vec![operator, digit, openBracket, closedBracket]
        }
    }

    fn advance(&mut self, len: usize) {
        self.pos += len;
        self.input = &self.input[len..];
    }

    fn next_token(&mut self) -> Option<Token> {
        let lexemes: Vec<fn(&str) -> Option<(Token, usize)>> =
            vec![operator, digit, open_bracket, closed_bracket];

        for lexeme in &lexemes {
            if self.input.is_empty() {
                return None;
            }
            if let Some((tok, len)) = lexeme(&self.input) {
                self.advance(len);
                return Some(tok);
            }
        }
        None
    }
}
