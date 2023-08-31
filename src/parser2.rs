use std::{collections::HashMap, env, fs, io};

#[derive(Debug, Clone)]
pub struct Tokenizer {
    text: String,
    offset: usize,
    len: usize,
    functions: HashMap<String, Vec<Token>>,
}

#[derive(Debug, Clone)]
enum Token {
    Division,
    Multiply,
    Plus,
    Minus,
    Period,
    Operation(String),
    Num(i32),
    Unknown(String),
    Func(String),
}

impl Token {
    fn is_valid_operator(op: &str) -> bool {
        matches!(op, "DUP" | "SWAP" | "DROP" | "OVER")
    }
}

impl Tokenizer {
    pub fn new(text: &str) -> Self {
        Tokenizer {
            text: text.to_string(),
            offset: 0,
            len: text.len(),
            functions: HashMap::new(),
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.read_whitespace();
        match self.read_next_char()? {
            '/' => Some(Token::Division),
            '*' => Some(Token::Multiply),
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '.' => Some(Token::Period),
            ':' => self.read_func(),
            x if x.is_ascii_digit() => self.tokenize_numer(x),
            x if x.is_alphabetic() => self.read_operation(x),
            x => Some(Token::Unknown(x.to_string())),
        }
    }

    fn read_func(&mut self) -> Option<Token> {
        let mut aux = String::new();
        while let Some(c) = self.peek() {
            if c != ';' {
                aux.push(c);
                self.read_next_char();
            } else {
                break;
            }
        }
        if let Some(c) = self.peek() {
            if c == ';' {
                self.read_next_char();
            }
        }
        let mut words_iter = aux.split_whitespace();
        dbg!(&words_iter);
        let identifier = words_iter.nth(0).unwrap();
        dbg!(&identifier);
        // TODO: Find how to join words from an iterator
        let body = words_iter.fold(String::new(), |acc, word| acc + " " + word);
        let x = parse(&body);
        self.functions.insert(identifier.to_string(), x);
        dbg!(&self.functions);

        Some(Token::Func(identifier.to_string()))
    }

    fn read_operation(&mut self, char: char) -> Option<Token> {
        let mut s = char.to_string();
        while let Some(c) = self.peek() {
            if c.is_alphabetic() {
                s.push(c);
                self.read_next_char();
            } else {
                break;
            }
        }
        Some(Token::Operation(s.to_owned()))
    }

    fn tokenize_numer(&mut self, char: char) -> Option<Token> {
        let mut aux = char.to_string();
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                aux.push(c);
                self.read_next_char();
            } else {
                break;
            }
        }
        let num = aux.parse().unwrap();
        Some(Token::Num(num))
    }

    fn read_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.read_next_char();
            } else {
                break;
            }
        }
    }

    fn read_next_char(&mut self) -> Option<char> {
        if self.offset == self.len {
            return None;
        }
        let offset = self.offset;
        self.offset += 1;

        self.text[offset..self.offset].chars().next()
    }

    fn peek(&self) -> Option<char> {
        match self.offset == self.len {
            false => self.text[self.offset..self.offset + 1].chars().next(),
            true => None,
        }
    }
}

fn parse(input: &str) -> Vec<Token> {
    let mut tokenizer = Tokenizer::new(input);
    dbg!(&tokenizer);
    let mut tokens = Vec::new();
    while let Some(token) = tokenizer.next_token() {
        tokens.push(token);
    }
    tokens.clone()
}

#[derive(Debug)]
struct Program {
    stack: Vec<i32>,
}

#[derive(Debug)]
enum Error {
    DivisionByZero,
    StackUnderflow,
}

impl Program {
    pub fn new() -> Self {
        Program { stack: Vec::new() }
    }
    fn eval(&mut self, input: &str) -> Result<(), Error> {
        let tokens = parse(input);
        for token in tokens {
            match token {
                Token::Num(n) => self.stack.push(n),
                Token::Period => {
                    if self.stack.len() >= 1 {
                        println!("{:#?}", self.stack.last())
                    } else {
                        return Err(Error::StackUnderflow);
                    }
                }
                // Token::Func(f) => {self.}
                Token::Operation(op) => {
                    let nums = self.stack.len();

                    match op.as_ref() {
                        "SWAP" if nums >= 2 => {
                            self.stack.swap(0, 1);
                        }
                        "DROP" if nums >= 1 => {
                            let _ = self.stack.pop();
                        }
                        "DUP" if nums >= 1 => {
                            let num1 = self.stack.pop().unwrap();
                            self.stack.push(num1);
                            self.stack.push(num1);
                        }
                        "OVER" if nums >= 2 => {
                            self.stack.push(self.stack[1].clone());
                        }
                        _ => return Err(Error::StackUnderflow),
                    }
                }
                op => {
                    if self.stack.len() < 2 {
                        return Err(Error::StackUnderflow);
                    }
                    let num1 = self.stack.pop().unwrap();
                    let num2 = self.stack.pop().unwrap();
                    match op {
                        Token::Plus => self.stack.push(num1 + num2),
                        Token::Minus => self.stack.push(num1 - num2),
                        Token::Multiply => self.stack.push(num1 * num2),
                        Token::Division => {
                            if num2 == 0 {
                                return Err(Error::DivisionByZero);
                            } else {
                                self.stack.push(num1 / num2)
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
        // now we need to
        Ok(())
    }
}

pub fn parse2_test() {
    // let aux = parse(input);
    let env = env::args().skip(1).collect::<String>();
    dbg!(&env);
    let mut s = Program::new();
    if env == "" {
        let stdin = io::stdin();
        loop {
            let mut user_input = String::new();
            stdin.read_line(&mut user_input).unwrap();
            let e = s.eval(&user_input);
            if e.is_err() {
                println!("{:#?}", e);
                break;
            }
        }
    } else {
        let f = fs::read_to_string(env).expect("Failed to read file");
        let e = s.eval(&f);
        if e.is_err() {
            println!("{:#?}", e);
        } else {
            println!("{:#?}", s.stack);
        }
    }
}
