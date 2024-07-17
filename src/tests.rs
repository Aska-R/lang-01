use crate::lexer;

use super::*;

#[test]
fn test_print() {
    let tokens = lexer::tokenizer("print(\"abc\")".to_string());

    let instructions = parser::parse(tokens).unwrap();
    
    let mut variables: Vec<interpreter::Variable> = Vec::new();
    let mut functions: Vec<interpreter::Function> = Vec::new();
    interpreter::interpret(instructions, &mut variables, &mut functions).unwrap();
}