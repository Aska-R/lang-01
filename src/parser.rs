// Reimplementation of the parser that isn't terrible implemented
use crate::lexer::{Tokens, Token}; 

// Tokens is a struct with a Token and a line number
use std::iter::Peekable;
use std::slice::Iter;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
    Minus, 
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq)]
pub enum Node {
    // Values
        // What purpose do values serve in the node area
    Int(i64),
    String(String), 
    Bool(bool),
    // Set Variables
    SetVariable {
        name: String,
        value: Box<Node>,
    },
    // Expressions
    UnaryExpr {
        op: Operator,
        child: Box<Node>,
    },
    BinaryExpr {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    CombineStr {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    // Other
    Repeat {
        count: i64,
        nodes: Vec<Node> // I'm not sure if I should be using a Vec here
    },
    Function {
        name: String,
        nodes: Vec<Node>
    },
    Eof
}

// Next scope is used when the parser reaches a scope indicator, used the next time the program goes into a scope
// Does this mean the error messages will be terrible? Yes. Do I have time to fix it now? No. :(
pub enum NextScope {
    Default, 
    IfState {
        check: Box<Node> // This node has to return either true or false
    },
    ElseIfState { // The check if this should run will be done in runtime
        check: Box<Node>
    },
    ElseState,
    WhileState {
        check: Box<Node>
    },
    RepeatState {
        repeat_count: i64
    },
    VariableSetState,
    FunctionCreateState,
}

#[derive(Debug, PartialEq)]
pub struct SyntaxError {
    message: String,
    line: u64
}

impl SyntaxError {
    pub fn new(message: String, line: u64) -> Self {
        SyntaxError {
            message,
            line
        }
    }
}

pub fn parse(tokens: Vec<Tokens>) -> Result<Vec<Node>, SyntaxError> {
    let mut iter: Peekable<Iter<Tokens>> = tokens.iter().peekable();

    return put_into_nodes(&mut iter, Token::Eof);
}

fn put_into_nodes(iter: &mut Peekable<Iter<Tokens>>, end_token: Token) -> Result<Vec<Node>, SyntaxError> {
    let mut nodes: Vec<Node> = Vec::new();
    let mut next_scope: NextScope = NextScope::Default;

    while let Some(token) = iter.next() {
        match &token.token {
            Token::Number(num) => {
                match iter.peek().unwrap().token {
                    Token::Plus => {
                        iter.next(); // this is equal to the plus
                        let next_value = &iter.next().unwrap().token;

                        match next_value {
                            Token::Number(next_num) => {
                                nodes.push(Node::BinaryExpr { op: (Operator::Plus), lhs: (Box::new(Node::Int(*num))), rhs: (Box::new(Node::Int(*next_num))) });
                            },

                            _ => {
                                return Err(SyntaxError::new(format!("Cannot combine Number and other type together using '+'"), token.line));
                            }
                        }
                    },
                    Token::Dash => {
                        iter.next(); // this is equal to the minus
                        let next_value = &iter.next().unwrap().token;

                        match next_value {
                            Token::Number(next_num) => {
                                nodes.push(Node::BinaryExpr { op: (Operator::Minus), lhs: (Box::new(Node::Int(*num))), rhs: (Box::new(Node::Int(*next_num))) });
                            }
                            
                            _ => {
                                return Err(SyntaxError::new(format!("Cannot combine Number and other type together using '-'"), token.line));
                            }
                        }
                    },
                    Token::Star => {
                        iter.next();
                        let next_value = &iter.next().unwrap().token;

                        match next_value {
                            Token::Number(next_num) => {
                                nodes.push(Node::BinaryExpr { op: (Operator::Minus), lhs: (Box::new(Node::Int(*num))), rhs: (Box::new(Node::Int(*next_num))) });
                            },

                            _ => {
                                return Err(SyntaxError::new(format!("Cannot combine Number and other types together using '*'"), token.line));
                            }
                        }
                    },
                    Token::Slash => {
                        iter.next();
                        let next_value = &iter.next().unwrap().token;

                        match next_value {
                            Token::Number(next_num) => {
                                nodes.push(Node::BinaryExpr { op: (Operator::Divide), lhs: (Box::new(Node::Int(*num))), rhs: (Box::new(Node::Int(*next_num))) });
                            },

                            _ => {
                                return Err(SyntaxError::new(format!("Cannot combine Number and other types together using '/'"), token.line));
                            }
                        }
                    },

                    _ => {
                        nodes.push(Node::Int(*num));
                    }
                }
            },
            Token::String(str) => {
                match iter.peek().unwrap().token {
                    Token::Equal => {
                        // Variable support goes here
                        set_variable(str.to_string(), token.line, iter).unwrap();
                    },
                    Token::Semicolon => {
                        nodes.push(Node::String(str.to_string()));
                    },
                    Token::Plus => {
                        iter.next();
                        let next_str = &iter.next().unwrap().token;

                        match next_str {
                            Token::String(next_str) => {
                                nodes.push(Node::CombineStr { lhs: (Box::new(Node::String(str.to_string()))), rhs: (Box::new(Node::String(next_str.to_string()))) });
                            },

                            _ => {
                                return Err(SyntaxError::new(format!("Cannot combine number and other type using '+'"), token.line));
                            }
                        }
                    }

                    _ => {
                        return Err(SyntaxError::new(format!("Expected semicolon"), token.line));
                    }
                }
            },
            Token::Other(_) => {
                // This is where tokens that don't fall under other token sections go

            },
            Token::Repeat => {
                // have to check the next tokens to see the repeat count
                // More advanced loops can use a proper scope when going into Paren but I won't for this
                let left_paren = iter.next().unwrap();
                let value = iter.next().unwrap();
                let right_paren = iter.next().unwrap();

                // Error checks
                if matches!(left_paren.token, Token::LeftParen) {
                    return Err(SyntaxError::new(format!("Left paren expected after repeat keyword"), left_paren.line));
                }
                else if matches!(right_paren.token, Token::RightParen) {
                    return Err(SyntaxError::new(format!("Right paren expected after repeat and left paren"), right_paren.line));
                }
                
                match value.token {
                    Token::Number(num) => {
                        next_scope = NextScope::RepeatState { repeat_count: (num) };
                        // Next scope is managed in Token::LeftBracket
                    },

                    _ => {
                        return Err(SyntaxError::new(format!("Expected number for repeat arg, however got something else"), token.line));
                    }
                }
            },
            Token::Bool(bool) => {
                nodes.push(Node::Bool(*bool));
            },
            Token::Plus => {
                
            },
            Token::Dash => {

            },
            Token::Star => {

            },
            Token::Slash => {

            },
            Token::LeftParen => {
                
            },
            Token::RightParen => {
                
            },
            Token::LeftBracket => {
                nodes.push(create_next_scope(iter, &next_scope, token.line).unwrap());
            },
            Token::RightBracket => {
                // This occurs when the function is called inside a left bracket
                if matches!(end_token, Token::RightBracket) {
                    
                }
            },
            Token::Dot => {

            },
            Token::Semicolon => {
                println!("NON-FATAL ERROR - Two semicolons in a row or semicolon was not properly consumed in parser.rs (Latter is fault of language creator)");
            },
            Token::Equal => {

            },
            Token::Eof => {
                if matches!(end_token, Token::Eof) {
                    nodes.push(Node::Eof);
                }
                else {
                    return Err(SyntaxError::new(format!("Did not close section"), token.line));
                }
            },
            Token::While => todo!(),
            Token::For => todo!(),
            Token::If => todo!(),
            Token::Elseif => todo!(),
            Token::Else => todo!(),
        }
    }
    todo!();
}

fn set_variable(variable_name: String, line: u64, iter: &mut Peekable<Iter<Tokens>>) -> Result<Vec<Node>, SyntaxError> {
    let mut nodes: Vec<Node> = Vec::new();

    // Variable support goes here
    iter.next(); // This is the equal sign
    let variable_value = iter.next().unwrap();
    let op_or_end = iter.peek().unwrap(); // Used to check if it is an operation or a semicolon
    match &variable_value.token {
        Token::String(value_str) => {
            match op_or_end.token {
                Token::Semicolon => {
                    nodes.push(Node::SetVariable { name: variable_name.to_string(), value: Box::new(Node::String(value_str.to_string())) });
                    return Ok(nodes);
                },
                Token::Plus => {
                    todo!();
                },

                _ => {
                    return Err(SyntaxError::new(format!(
                        "Excepted semicolon or operator after String, got unexpected result instead"), 
                        // Using variable_value.line as that is where the error when writing the code would be located
                        variable_value.line
                    )); 
                }
            }
        },
        Token::Number(_num) => {
            todo!();
        }

        _ => {
            return Err(SyntaxError::new(format!("That cannot be stored as a variable"), line));
        }
    } 
}

fn create_next_scope(iter: &mut Peekable<Iter<Tokens>>, next_scope: &NextScope, line: u64) -> Result<Node, SyntaxError> {
    // Possible optimisation here, instead of putting the nodes into new_nodes have it directly edit the nodes
    let new_nodes = put_into_nodes(iter, Token::RightBracket).unwrap();

    // Depending on which the next scope will be what affect what the next scope 
    match next_scope {
        NextScope::Default => {
            return Err(SyntaxError::new(format!("Unset scopes are not supported"), line));
        },
        NextScope::IfState { check: _ } => todo!(),
        NextScope::ElseIfState { check: _ } => todo!(),
        NextScope::ElseState => todo!(),
        NextScope::WhileState { check: _ } => todo!(),
        NextScope::RepeatState { repeat_count } => {
            return Ok(Node::Repeat { count: *repeat_count, nodes: new_nodes });
        },
        NextScope::VariableSetState => todo!(),
        NextScope::FunctionCreateState => {
            todo!();
        },
    }
}

// fn examine_numbers(iter: &mut Peekable<Iter<Tokens>>, ) -> Result<Vec<Node>, SyntaxError> {
//     let nodes = 

//     match iter.peek().unwrap().token {
//         Token::Plus => {
//             iter.next(); // this is equal to the plus
//             let next_value = &iter.next().unwrap().token;

//             match next_value {
//                 Token::Number(next_num) => {
//                     nodes.push(Node::BinaryExpr { op: (Operator::Plus), lhs: (Box::new(Node::Int(*num))), rhs: (Box::new(Node::Int(*next_num))) });
//                 },

//                 _ => {
//                     return Err(SyntaxError::new(format!("Cannot combine Number and other type together using '+'"), token.line));
//                 }
//             }
//         },
//         Token::Dash => {
//             iter.next(); // this is equal to the minus
//             let next_value = &iter.next().unwrap().token;

//             match next_value {
//                 Token::Number(next_num) => {
//                     nodes.push(Node::BinaryExpr { op: (Operator::Minus), lhs: (Box::new(Node::Int(*num))), rhs: (Box::new(Node::Int(*next_num))) });
//                 }
                
//                 _ => {
//                     return Err(SyntaxError::new(format!("Cannot combine Number and other type together using '-'"), token.line));
//                 }
//             }
//         },
//         Token::Star => {
//             iter.next();
//             let next_value = &iter.next().unwrap().token;

//             match next_value {
//                 Token::Number(next_num) => {
//                     nodes.push(Node::BinaryExpr { op: (Operator::Minus), lhs: (Box::new(Node::Int(*num))), rhs: (Box::new(Node::Int(*next_num))) });
//                 },

//                 _ => {
//                     return Err(SyntaxError::new(format!("Cannot combine Number and other types together using '*'"), token.line));
//                 }
//             }
//         },
//         Token::Slash => {
//             iter.next();
//             let next_value = &iter.next().unwrap().token;

//             match next_value {
//                 Token::Number(next_num) => {
//                     nodes.push(Node::BinaryExpr { op: (Operator::Divide), lhs: (Box::new(Node::Int(*num))), rhs: (Box::new(Node::Int(*next_num))) });
//                 },

//                 _ => {
//                     return Err(SyntaxError::new(format!("Cannot combine Number and other types together using '/'"), token.line));
//                 }
//             }
//         },

//         _ => {
//             nodes.push(Node::Int(*num));
//         }
//     }

//     todo!();
// }