#![allow(dead_code)]

use lexer::dump_tokens;

mod lexer;
mod parser;
mod interpreter;
mod st;

#[cfg(test)]
mod tests;

fn main() {
    let _short_input = "3+3-2/()1*\"123\"456\"789\"";
    let _long_input = "3 + 3
    2 + 2
    7 - 9
    \"5235\"
    123123
    \"this is a test of the string feature\"
    42
    6*7
    let 
    test 
    abc
    abc.test.abc
    true
    \"true\"";
    let _repeat_test_input = "repeat(3) { abc 7 + 3 * 7 \"test\" }";
    let _invalid_test = "1+a";
    let _print_test = "print(\"abc\")";

    // Select which test input you want to use here
    let input = _print_test.to_string(); 
    println!("{}", input);

    let tokens = lexer::tokenizer(input.clone());
    dump_tokens(tokens.clone());

    let instructions = parser::parse(tokens).unwrap();

    println!("Input file: {}", input.clone());

    println!("--- RUNNING ---");
    
    let mut variables: Vec<interpreter::Variable> = Vec::new();
    let mut functions: Vec<interpreter::Function> = Vec::new();
    interpreter::interpret(instructions, &mut variables, &mut functions).unwrap();
}