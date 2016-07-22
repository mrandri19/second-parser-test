extern crate regex;

use regex::Regex;

fn main() {
    let tokens = lexer("21+1+(1+1)");

    let ast = parser(tokens.clone());

    println!("{:?}", tokens);
}

struct AST {
    digit: i32,
    expr: Option<Vec<Expr>>
}

enum Either<L, R> {
    Left(L),
    Right(R)
}

struct Expr {
    operator: Operators,
    //TODO: read about Box
    exprOrBG: Either<i32, Box<BracketGroup>>
}

struct BracketGroup {
    digit: i32,
    expr: Option<Vec<Expr>>
}

fn parser(input: Vec<Token>) -> Result<AST, &'static str> {
    if let Token::Digit(digit) = input[0] {
        if input[1] == Token::EOF {
            return Ok(AST {
                digit: digit,
                expr: None
            });
        }

        if let Token::Operator(op) = input[1] {
            return Ok(AST {
                // TODO: finish
                digit: digit,
                expr: Some(try!(expr(input[1..])))
            });
        } else {
            return Err("Expected operator");
        }
    } else {
        return Err("Expected digit");
    }
}

fn expr(input: Vec<Token>) -> Result<Expr, &'static str> {
    Ok(Expr {
        operator: Operators::Minus,
        exprOrBG: Either::Left(13)
    })
}


fn lexer(input: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(input);

    let mut tokens = Vec::new();
    while let Some(tok) = lexer.next_token() {
        tokens.push(tok);
    }
    tokens.push(Token::EOF);
    tokens
}

#[derive(Debug, PartialEq, Clone)]
enum Operators {
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Debug, PartialEq, Clone)]
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

#[test]
fn single_digit() {
    assert_eq!(lexer("1"), vec![Token::Digit(1), Token::EOF]);
}
#[test]
fn multi_digit_number() {
    assert_eq!(lexer("12"), vec![Token::Digit(12), Token::EOF]);
}
#[test]
fn simple_addition() {
    assert_eq!(lexer("132+2"), vec![Token::Digit(132), Token::Operator(Operators::Plus), Token::Digit(2), Token::EOF]);
}
