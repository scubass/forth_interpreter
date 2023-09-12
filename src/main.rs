// TODO: Change direction of lexer
mod parser2;

// #[derive(Clone, Debug)]
// struct Tokenizer {
//     text: String,
//     offset: usize,
// }
//
// #[derive(Clone, Debug)]
// enum Token {
//     Division,
//     Multiply,
//     Plus,
//     Minus,
//     OtherSymbol(String),
//     Operation(String),
//     Num(i32),
// }
//
// impl Token {
//     fn is_valid_operator(s: &str) -> bool {
//         matches!(s, "DUP" | "SWAP" | "DROP" | "OVER")
//     }
// }
//
// impl Tokenizer {
//     pub fn new(text: &str) -> Self {
//         Tokenizer {
//             text: text.to_string(),
//             offset: text.len(),
//         }
//     }
//
//     pub fn next_token(&mut self) -> Option<Token> {
//         self.read_whitespace();
//         match self.read_next_char()? {
//             '/' => Some(Token::Division),
//             '*' => Some(Token::Multiply),
//             '+' => Some(Token::Plus),
//             '-' => Some(Token::Minus),
//             x if x.is_ascii_digit() => self.tokenize_number(x),
//             x if x.is_alphabetic() => self.read_ident(x),
//             x => Some(Token::OtherSymbol(x.to_string())),
//         }
//     }
//
//     fn tokenize_number(&mut self, char: char) -> Option<Token> {
//         let mut aux_s = char.to_string();
//
//         while let Some(c) = self.peek() {
//             if c.is_ascii_digit() {
//                 aux_s.push(c);
//                 self.read_next_char();
//             } else {
//                 break;
//             }
//         }
//         let num = aux_s.parse().unwrap();
//         Some(Token::Num(num))
//     }
//
//     fn read_ident(&mut self, char: char) -> Option<Token> {
//         let mut operation = char.to_string();
//
//         while let Some(c) = self.peek() {
//             if c.is_alphabetic() {
//                 operation.push(c);
//                 self.read_next_char();
//             } else {
//                 break;
//             }
//         }
//         let operation = operation.chars().rev().collect::<String>();
//         match Token::is_valid_operator(&operation) {
//             true => Some(Token::Operation(operation)),
//             false => Some(Token::OtherSymbol(operation)),
//         }
//     }
//
//     fn peek(&self) -> Option<char> {
//         if self.offset == 0 {
//             return None;
//         }
//         self.text[self.offset - 1..self.offset].chars().next()
//     }
//
//     fn read_next_char(&mut self) -> Option<char> {
//         if self.offset == 0 {
//             return None;
//         }
//         let offset = self.offset;
//         self.offset -= 1;
//
//         self.text[offset - 1..offset].chars().next()
//     }
//
//     fn read_whitespace(&mut self) -> Option<()> {
//         while let Some(c) = self.peek() {
//             if c.is_whitespace() {
//                 self.read_next_char();
//             } else {
//                 break;
//             }
//         }
//         None
//     }
// }
//
// fn parse(str: &str) -> Vec<Token> {
//     let mut tokenizer = Tokenizer::new(str);
//     let mut tokens = Vec::new();
//     while let Some(token) = tokenizer.next_token() {
//         tokens.push(token.clone());
//     }
//     tokens
// }
//
// // TODO: Find better name
// #[derive(Debug)]
// struct Aux {
//     stack: Vec<i32>,
// }
//
// // TODO: Copy from exercism the error codes
// #[derive(Debug)]
// enum Error {
//     DivisionByZero,
//     StackUnderflow,
//     UnknownWord,
//     // InvalidWord,
// }
//
// impl Aux {
//     fn new() -> Self {
//         Aux { stack: Vec::new() }
//     }
//     fn eval(&mut self, input: &str) -> Result<(), Error> {
//         let tokens = parse(input);
//         dbg!(&tokens);
//         for i in tokens.into_iter().rev() {
//             match i {
//                 Token::Num(n) => self.stack.push(n),
//                 Token::OtherSymbol(_) => return Err(Error::UnknownWord),
//                 Token::Operation(op) => {
//                     let nums = self.stack.len();
//
//                     match op.as_ref() {
//                         "SWAP" if nums >= 2 => {
//                             self.stack.swap(0, 1);
//                         }
//                         "DROP" if nums >= 1 => {
//                             let _ = self.stack.pop();
//                         }
//                         "DUP" if nums >= 1 => {
//                             let num1 = self.stack.pop().unwrap();
//                             self.stack.push(num1);
//                             self.stack.push(num1);
//                         }
//                         "OVER" if nums >= 2 => {
//                             self.stack.push(self.stack[1]);
//                         }
//                         _ => return Err(Error::StackUnderflow),
//                     }
//                 }
//                 op => {
//                     if self.stack.len() < 2 {
//                         return Err(Error::StackUnderflow);
//                     }
//                     let num1 = self.stack.pop().unwrap();
//                     let num2 = self.stack.pop().unwrap();
//                     match op {
//                         Token::Plus => self.stack.push(num1 + num2),
//                         Token::Minus => self.stack.push(num1 - num2),
//                         Token::Multiply => self.stack.push(num1 * num2),
//                         Token::Division => {
//                             if num2 == 0 {
//                                 return Err(Error::DivisionByZero);
//                             } else {
//                                 self.stack.push(num1 / num2)
//                             }
//                         }
//                         _ => unreachable!(),
//                     }
//                 }
//             }
//         }
//         // now we need to
//         Ok(())
//     }
// }

fn main() {
    // let mut aux = Aux::new();
    // aux.eval("1 2 + DUP").unwrap();
    // dbg!(aux);
    parser2::parse2_test();
}

// #[test]
// fn valid_sum() {
//     let mut aux = Aux::new();
//     aux.eval("1 2 + DUP").unwrap();
//     assert_eq!(vec![3, 3], aux.stack);
//     assert_eq!(aux.stack.pop(), Some(3));
//     assert_eq!(aux.stack.pop(), Some(3));
// }
