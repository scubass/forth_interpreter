use std::{
    collections::HashMap,
    env, fs, io,
    sync::mpsc::{self, channel},
};
// TODO: implement message passing to allow execution while we keep lexing the input
// so replace the vec for mpsc

#[derive(Debug, Clone)]
pub struct Tokenizer {
    text: String,
    offset: usize,
    len: usize,
    functions: HashMap<String, Vec<Token>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Token {
    Division,
    Multiply,
    Plus,
    Minus,
    Period,
    Operation(String),
    Num(i32),
    Unknown(String),
    FuncDefinition(String),
    FuncCall(String),
    Eof,
}

fn is_valid_operator(op: &str) -> bool {
    matches!(op, "DUP" | "SWAP" | "DROP" | "OVER")
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
        // dbg!(&aux);
        let mut words_iter = aux.split_whitespace();
        let identifier = words_iter.next().unwrap();
        // dbg!(&identifier);
        // dbg!(&words_iter);
        // TODO: Find how to join words from an iterator, or more efficient way
        let body = words_iter.fold(String::new(), |acc, word| acc + " " + word);
        let (sender, receiver) = mpsc::channel();
        parse(sender, &body);
        let mut aux_tokens = Vec::new();
        loop {
            let t = receiver.recv().unwrap();
            match t {
                Token::Eof => break,
                token => aux_tokens.push(token),
            }
        }
        // dbg!(&aux_tokens);
        self.functions.insert(identifier.to_string(), aux_tokens);
        // dbg!(&self.functions);

        Some(Token::FuncDefinition(identifier.to_string()))
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
        if is_valid_operator(&s) {
            Some(Token::Operation(s.to_owned()))
        } else if self.functions.get(&s).is_some() {
            Some(Token::FuncCall(s.to_owned()))
        } else {
            Some(Token::Unknown(s.to_owned()))
        }
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
            if c.is_whitespace() || c == ' ' {
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

fn parse_to_vec(input: &str) -> Vec<Token> {
    let mut tokenizer = Tokenizer::new(input);
    // dbg!(&tokenizer);
    let mut tokens = Vec::new();
    while let Some(token) = tokenizer.next_token() {
        // send the token to the eval function
        match token {
            Token::FuncCall(func_identifier) => {
                let mut func_tokens = tokenizer.functions.get(&func_identifier).unwrap().clone();
                tokens.append(&mut func_tokens);
            }
            token => tokens.push(token),
        }
    }
    tokens
}
fn parse(sender: mpsc::Sender<Token>, input: &str) {
    let mut tokenizer = Tokenizer::new(input);
    // dbg!(&tokenizer);
    while let Some(token) = tokenizer.next_token() {
        // send the token to the eval function
        match token {
            Token::FuncCall(ref func_identifier) => {
                let func_tokens = tokenizer.functions.get(func_identifier).unwrap().clone();
                for t in &func_tokens {
                    let _ = sender.send(t.clone());
                }
            }
            token => {
                let _ = sender.send(token);
            }
        }
    }
    let _ = sender.send(Token::Eof);
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
        let (sender, reciver) = channel::<Token>();
        parse(sender, input);
        loop {
            let token = reciver.recv().unwrap();
            match token {
                Token::Eof => break,
                Token::Num(n) => self.stack.push(n),
                Token::Period => {
                    if !self.stack.is_empty() {
                        println!("{:#?}", self.stack.last())
                    } else {
                        return Err(Error::StackUnderflow);
                    }
                }
                Token::FuncCall(_) => (),
                Token::FuncDefinition(_) => (),
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
                            self.stack.push(self.stack[1]);
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
        Ok(())
    }
}

pub fn parse2_test() {
    let env = env::args().skip(1).collect::<String>();
    dbg!(&env);
    let mut s = Program::new();
    if env.is_empty() {
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
