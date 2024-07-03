use crate::lexer;

use super::*;

// Because the unit tests in this function rely on create_fake_iter 
// being correct I will make a unit test to test my unit test
// Surely there is no way that will get out of hand !! Clueless

// Got told to use 'static lifetime, have to recheck if this is correct later
fn create_fake_tokens(input: String) -> Vec<Tokens> {
    let input = input;

    let tokens = lexer::tokenizer(input);
    
    return tokens;
}

fn create_fake_node(node: Node) -> Result<Vec<Node>, SyntaxError> {
    let mut node_vec: Vec<Node> = Vec::new();

    node_vec.push(node);

    return Ok(node_vec);
}

#[test]
fn test_set_variable() {
    let binding = create_fake_tokens(
        "= \"the answer to life the universe and everything\";".to_string()
    );
    let mut iter = binding.iter().peekable();
    assert_eq!(
        set_variable("variable_name".to_string(), 42, &mut iter), 
        create_fake_node(Node::SetVariable 
            {
                name: ("variable_name".to_string()), 
                value: (
                    Box::new(
                        Node::String("the answer to life the universe and everything".to_string())
                    )
                ),
            }
        )
    );
}