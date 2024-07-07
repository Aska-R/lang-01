// Reimplementation of the parser that isn't terrible implemented
use crate::lexer::{Tokens, Token}; 
use crate::interpreter::{Variable, Function};

// Tokens is a struct with a Token and a line number
use std::iter::Peekable;
use std::slice::Iter;

#[cfg(test)]
mod tests;

/// Operators are used for operations (+, -, *, /)
#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Plus,
    Minus, 
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Comparator {
    Equal,      // ==
    NotEqual,   // !=
    More,       // > 
    Less,       // <
    EqualMore,  // => or >=
    EqualLess,  // =< or <=
}


/// Node is the struct used for the instructions which will be eventually be interpreted.
#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    // Values
    Int(i64),
    String(String), 
    Bool(bool),
    // Set Variables
    SetVariable {
        var: Variable,
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
    // Comparisions
    IfBinaryCompare {
        comparator: Comparator,
        lhs: Box<Node>,
        rhs: Box<Node>,
        nodes: Vec<Node>,
    },
    IfUnaryCompare {
        expected: bool,
        actual: Box<Node>,
        nodes: Vec<Node>,
    },
    IfElseBinaryCompare {
        comparator: Comparator,
        lhs: Box<Node>,
        rhs: Box<Node>,
        nodes: Vec<Node>,
    },
    IfElseUnaryCompare {
        expected: bool,
        actual: Box<Node>,
        nodes: Vec<Node>
    },
    Else {
        nodes: Vec<Node>
    },
    // Other
    Repeat {
        count: i64,
        nodes: Vec<Node>
    },
    Function {
        name: String,
        nodes: Vec<Node>
    },
    Eof
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

    put_into_nodes(&mut iter, Token::Eof)
}

fn put_into_nodes(iter: &mut Peekable<Iter<Tokens>>, end_token: Token) -> Result<Vec<Node>, SyntaxError> {
    let mut nodes: Vec<Node> = Vec::new();

    while let Some(token) = iter.next() {
        match &token.token {
            Token::Number(num) => {
                nodes.append(&mut examine_numbers(iter, num, token.line).unwrap());
            },
            Token::String(str) => {
                nodes.append(&mut examine_string(iter, str, token.line).unwrap());
            },
            Token::Other(_) => {
                // This is where tokens that don't fall under other token sections go
            },
            Token::Repeat => {
                nodes.push(create_repeat(iter, token.line).unwrap());
            },
            Token::For => todo!(),
            Token::If => todo!(),
            Token::Elseif => todo!(),
            Token::Else => todo!(),
            Token::While => todo!(),
            Token::Bool(bool) => {
                nodes.push(Node::Bool(*bool));
            },
            Token::RightParen => {
                
            },
            Token::LeftBracket => {
                // I'm going to start managing next_scope in the repeat tokens now
                // nodes.push(create_next_scope(iter, &next_scope, token.line).unwrap());

                return Err(SyntaxError::new(
                    "{ token found in unexpected location".to_string(),
                    token.line
                ));
            },
            
            // END OF SCOPES -----------------------------------------------------------------------
            Token::RightBracket => {
                // This occurs when the function is called inside a left bracket
                if matches!(end_token, Token::RightBracket) {
                    // return to previous scope
                    return Ok(nodes)
                }
            },
            Token::Eof => {
                if matches!(end_token, Token::Eof) {
                    nodes.push(Node::Eof);
                }
                else {
                    return Err(SyntaxError::new("Did not close section".to_string(), token.line));
                }
            },
            // -------------------------------------------------------------------------------------
            
            // Error handling ----------------------------------------------------------------------
            Token::Plus => {
                return Err(SyntaxError::new(
                    "+ token found in unexpected location, this error could also be the result of programming language creator's error".to_string(),
                    token.line
                ));
            },
            Token::Dash => {
                return Err(SyntaxError::new(
                    "- token found in unexpected location, this error could also be the result of programming language creator's error".to_string(),
                    token.line
                ));
            },
            Token::Star => {
                return Err(SyntaxError::new(
                    "* token found in unexpected location, this error could also be the result of programming language creator's error".to_string(),
                    token.line
                ));
            },
            Token::Slash => {
                return Err(SyntaxError::new(
                    "/ token found in unexpected location, this error could also be the result of programming language creator's error".to_string(),
                    token.line
                ));
            },
            Token::Dot => {
                return Err(SyntaxError::new(
                    "Dot token found in unexpected location, this error could also be the result of programming language creator's error".to_string(),
                    token.line
                ));
            },
            Token::Semicolon => {
                println!("NON-FATAL ERROR - Two semicolons in a row or semicolon was not properly consumed in parser.rs (Latter is fault of language creator)");
            },
            Token::Equal => {
                return Err(SyntaxError::new(
                    "Equal token found in unexpected location, this error could also be the result of programming language creator's error".to_string(),
                    token.line
                ));
            },
            Token::LeftParen => {
                return Err(SyntaxError::new(
                    "} token found in unexpected location, this error could also be the result of programming language creator's error".to_string(),
                    token.line
                )); 
            },
            // Comparision tokens
            Token::NotEqual => {
                if matches!(end_token, Token::Comparator) {
                    return Ok(nodes);
                }

                return Err(SyntaxError::new(
                    "!= token found in unexpected location, this error could also be the result of programming language creator's error".to_string(),
                    token.line
                )); 
            },
            Token::More => {
                if matches!(end_token, Token::Comparator) {
                    return Ok(nodes);
                }

                return Err(SyntaxError::new(
                    "> token found in unexpected location, this error could also be the result of programming language creator's error".to_string(),
                    token.line
                )); 
            },
            Token::Less => {
                if matches!(end_token, Token::Comparator) {
                    return Ok(nodes);
                }

                return Err(SyntaxError::new(
                    "< token found in unexpected location, this error could also be the result of programming language creator's error".to_string(),
                    token.line
                )); 
            },
            Token::EqualMore => {
                if matches!(end_token, Token::Comparator) {
                    return Ok(nodes);
                }

                return Err(SyntaxError::new(
                    ">= or => token found in unexpected location, this error could also be the result of programming language creator's error".to_string(),
                    token.line
                )); 
            },
            Token::EqualLess => {
                if matches!(end_token, Token::Comparator) {
                    return Ok(nodes);
                }

                return Err(SyntaxError::new(
                    "<= or =< token found in unexpected location, this error could also be the result of programming language creator's error".to_string(),
                    token.line
                )); 
            },
            Token::Comparator => {
                return Err(SyntaxError::new("Comparator token found, this is impossible".to_string(), token.line));
            },
            // -------------------------------------------------------------------------------------
        }
    }
    
    return Err(SyntaxError::new(
        "End token not found".to_string(),
        0
    ))
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
                    Ok(nodes)
                },
                Token::Plus => {
                    todo!();
                },

                _ => {
                    Err(SyntaxError::new("Excepted semicolon or operator after String, got unexpected result instead".to_string(), 
                        // Using variable_value.line as that is where the error when writing the code would be located
                        variable_value.line
                    ))
                }
            }
        },
        Token::Number(_num) => {
            todo!();
        }

        _ => {
            Err(SyntaxError::new("That cannot be stored as a variable".to_string(), line))
        }
    } 
}

fn examine_numbers(iter: &mut Peekable<Iter<Tokens>>, num: &i64, line: u64) -> Result<Vec<Node>, SyntaxError> {
    let mut nodes: Vec<Node> =  Vec::new();

    match iter.peek().unwrap().token {
        Token::Plus => {
            iter.next(); // this is equal to the plus
            let next_value = &iter.next().unwrap().token;

            match next_value {
                Token::Number(next_num) => {
                    nodes.push(Node::BinaryExpr { 
                        op: (Operator::Plus), 
                        lhs: (Box::new(Node::Int(*num))), 
                        rhs: (Box::new(Node::Int(*next_num))) 
                    });
                },

                _ => {
                    return Err(SyntaxError::new(
                        "Cannot combine Number and other type together using '+'".to_string(),
                        line
                    ));
                }
            }
        },
        Token::Dash => {
            iter.next(); // this is equal to the minus
            let next_value = &iter.next().unwrap().token;

            match next_value {
                Token::Number(next_num) => {
                    nodes.push(Node::BinaryExpr { 
                        op: (Operator::Minus), 
                        lhs: (Box::new(Node::Int(*num))), 
                        rhs: (Box::new(Node::Int(*next_num))) 
                    });
                }
                
                _ => {
                    return Err(SyntaxError::new(
                        "Cannot combine Number and other type together using '-'".to_string(),
                        line
                    ));
                }
            }
        },
        Token::Star => {
            iter.next();
            let next_value = &iter.next().unwrap().token;

            match next_value {
                Token::Number(next_num) => {
                    nodes.push(Node::BinaryExpr { 
                        op: (Operator::Minus), 
                        lhs: (Box::new(Node::Int(*num))), 
                        rhs: (Box::new(Node::Int(*next_num))) 
                    });
                },

                _ => {
                    return Err(SyntaxError::new(
                        "Cannot combine Number and other types together using '*'".to_string(), 
                        line
                    ));
                }
            }
        },
        Token::Slash => {
            iter.next();
            let next_value = &iter.next().unwrap().token;

            match next_value {
                Token::Number(next_num) => {
                    nodes.push(Node::BinaryExpr { 
                        op: (Operator::Divide), 
                        lhs: (Box::new(Node::Int(*num))), 
                        rhs: (Box::new(Node::Int(*next_num))) 
                    });
                },

                _ => {
                    return Err(SyntaxError::new(
                        "Cannot combine Number and other types together using '/'".to_string(), 
                        line
                    ));
                }
            }
        },

        // Just number
        Token::Semicolon => {
            nodes.push(Node::Int(*num));
        },

        // Error handling
        Token::Number(_) => {
            return Err(SyntaxError::new(
                "Two number tokens found in a row, make sure there isn't whitespace etc. between numbers".to_string(),
                line
            ));
        },

        _ => {
            return Err(SyntaxError::new(
                "Unexpected token found after number".to_string(),
                line
            ));
        }
    }

    Ok(nodes)
}

fn examine_string(iter: &mut Peekable<Iter<Tokens>>, str: &String, line: u64) -> Result<Vec<Node>, SyntaxError> {
    let mut nodes = Vec::new();

    match iter.peek().unwrap().token {
        Token::Equal => {
            // Variable support goes here
            set_variable(str.to_string(), line, iter).unwrap();
        },
        Token::Semicolon => {
            nodes.push(Node::String(str.to_string()));
        },
        Token::Plus => {
            iter.next();
            let next_str = &iter.next().unwrap().token;

            match next_str {
                Token::String(next_str) => {
                    nodes.push(Node::CombineStr { 
                        lhs: (Box::new(Node::String(str.to_string()))), 
                        rhs: (Box::new(Node::String(next_str.to_string()))) 
                    });
                },

                _ => {
                    return Err(SyntaxError::new(
                        "Cannot combine number and other type using '+'".to_string(), 
                        line
                    ));
                }
            }
        }

        _ => {
            return Err(SyntaxError::new("Expected semicolon".to_string(), line));
        }
    } 

    Ok(nodes)
}

fn create_repeat(iter: &mut Peekable<Iter<Tokens>>, line: u64) -> Result<Node, SyntaxError> {
    // have to check the next tokens to see the repeat count
    // More advanced loops can use a proper scope when going into Paren but I won't for this
    let left_paren = iter.next().unwrap();
    let value = iter.next().unwrap();
    let right_paren = iter.next().unwrap();

    // Error checks
    if !matches!(left_paren.token, Token::LeftParen) {
        return Err(SyntaxError::new("Left paren expected after repeat keyword".to_string(), left_paren.line));
    }
    else if !matches!(right_paren.token, Token::RightParen) {
        return Err(SyntaxError::new("Right paren expected after repeat and left paren".to_string(), right_paren.line));
    }
    
    match value.token {
        Token::Number(num) => {
            iter.next();
            let new_nodes = put_into_nodes(iter, Token::RightBracket).unwrap();
            return Ok(Node::Repeat { count: num, nodes: new_nodes });
        },

        _ => {
            Err(SyntaxError::new("Expected number for repeat arg, however got something else".to_string(), line))
        }
    }
}

fn create_if(iter: &mut Peekable<Iter<Tokens>>, line: u64) -> Result<Node, SyntaxError> {
    // Makes sure that the 
    if !matches!(iter.next().unwrap().token, Token::LeftParen) {
        return Err(SyntaxError::new(
            "If statement requires ( around comparision".to_string(),
            line
        ));
    }

    // I think I have to go in a loop to fully process whatever is on the lhs
    // let lhs_nodes = put_into_nodes(iter, Token::Comparator).unwrap();
    // Now need to work out the token that resulted in put_into_nodes ending
    
    // If rhs is { then lhs has to be a bool
    // I've realised that if I was to put a bool in without any comparator this wouldnt work
    // so I'm goign to work on this later.
        
    todo!();
}

fn create_else(iter: &mut Peekable<Iter<Tokens>>, line: u64) -> Result<Node, SyntaxError> {
    if !matches!(iter.next().unwrap().token, Token::LeftBracket) {
        return Err(SyntaxError::new(
            "Expected { got different token instead".to_string(),
            line
        ));
    }

    let new_nodes = put_into_nodes(iter, Token::RightBracket).unwrap();
    return Ok(Node::Else { nodes: (new_nodes) });
}