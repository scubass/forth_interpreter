mod parser;

fn main() {
    // let mut aux = Aux::new();
    // aux.eval("1 2 + DUP").unwrap();
    // dbg!(aux);
    parser::parser();
}

// #[test]
// fn valid_sum() {
//     let mut aux = Aux::new();
//     aux.eval("1 2 + DUP").unwrap();
//     assert_eq!(vec![3, 3], aux.stack);
//     assert_eq!(aux.stack.pop(), Some(3));
//     assert_eq!(aux.stack.pop(), Some(3));
// }
