fn main() {
    println!("Hello, world!");
    let mut lexer = Lexer::new("1+1+(1+1)");
}

enum Operators {
    Plus,
    Minus,
    Star,
    Slash
}

enum Token {
    Operator(Operators),
    Digit(f64),
    OpenBracket,
    ClosedBracket
}

fn operator(input: &str) -> Option<(Token, i32)> {
    match input.chars().next().unwrap() {
        '+' => Some((Token::Operator(Operators::Plus),1)),
        '-' => Some((Token::Operator(Operators::Minus),1)),
        '*' => Some((Token::Operator(Operators::Star),1)),
        '/' => Some((Token::Operator(Operators::Slash),1)),
        _ => None
    }
}

fn digit(input: &str) -> Option<(Token, i32)> {

}

fn openBracket(input: &str) -> Option<(Token, i32)> {

}

fn closedBracket(input: &str) -> Option<(Token, i32)> {

}

static lexemes: Vec<fn(&str) -> Option<(Token, i32)>> = vec![operator, digit, openBracket, closedBracket];

struct Lexer {
    input: String,
    pos: i32
}

impl Lexer {
    fn new(input: &str) -> Lexer {
        Lexer {
            input: input.to_owned(),
            pos: 0
        }
    }

    fn advance(&mut self, len: i32) {
        self.pos += len;
    }

    fn next_token(&mut self) -> Option<Token> {
        for lex_func in lexemes {
            if let Some((tok, len)) = lex_func(&self.input) {
                self.advance(len);
                return Some(tok);
            }
        }
        None
    }
}
