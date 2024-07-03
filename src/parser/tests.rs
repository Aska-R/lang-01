use crate::lexer;

use std::iter::Peekable;
use std::slice::Iter;

use super::*;


// Because the unit tests in this function rely on create_fake_iter being correct I will make a unit test to test my unit test
// Surely there is no way that will get out of hand !! Clueless
// Got told to use 'static lifetime, have to recheck if this is correct later
fn create_fake_iter(input: String) -> Peekable<Iter<'static, Tokens>> {
    let input = input;

    let tokens = lexer::tokenizer(input);
    
    return tokens.iter().peekable();
}

#[test]
fn test_set_variable() {
    let iter = create_fake_iter("= \"the answer to life the universe and everything\"".to_string());
    let mut node: Vec<Node> = Vec::new();
    // assert_eq!(set_variable("variable_name".to_string(), 42, &mut iter), 
    //     Ok(node.push(Node::SetVariable {
    //         name: "variable_name".to_string(),
    //         value: Box::new(Node::String("the answer to life the universe and everything".to_string()))
    //     }
    // )));
}