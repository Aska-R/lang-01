use crate::parser::Node;

use std::iter::Peekable;
use std::slice::Iter;

#[derive(Debug)]
pub struct RuntimeError {
    message: String
}

impl RuntimeError {
    pub fn new(message: String) -> Self {
        RuntimeError {
            message
        }
    }
}

pub enum Variable {
    Int {
        num: i64
    },
}

pub enum Function {

}


// I don't think I need to seperate interpret from interpreter as I do not think I will need to call recursion
fn interpret(instructions: Vec<Node>, variables: &Vec<Variable>, functions: &Vec<Function>) -> Result<(), RuntimeError> {
    let iter: Peekable<Iter<Node>> = instructions.iter().peekable();
    

    for instruction in iter {
        match instruction {
            // Loops
            Node::Repeat { count, nodes } => {
                for _ in 0..*count {
                    interpret(nodes.to_vec(), variables, functions).unwrap();
                }
            },

            // Statements
            Node::SetVariable { name: _, value: _ } => todo!(),
            Node::Function { name: _, nodes: _ } => todo!(),

            // Comparisons
            Node::IfBinaryCompare { comparator: _, lhs: _, rhs: _, nodes } => todo!(),
            Node::IfUnaryCompare { expected: _, actual: _, nodes } => todo!(),
            Node::IfElseBinaryCompare { comparator, lhs, rhs, nodes } => todo!(),
            Node::IfElseUnaryCompare { expected: _, actual: _, nodes } => todo!(),
            
            // EOF
            Node::Eof => return Ok(()),
            
            // Error handling ----------------------------------------------------------------------
            Node::Else { nodes: _ } => {
                return Err(RuntimeError::new(
                    "Else node found in unexpected locaiton".to_string()
                ));
            },

            Node::Int(_) => todo!(),
            Node::String(_) => todo!(),
            Node::Bool(_) => todo!(),
            Node::UnaryExpr { op: _, child: _ } => todo!(),
            Node::BinaryExpr { op: _, lhs: _, rhs: _ } => todo!(),
            Node::CombineStr { lhs: _, rhs: _ } => todo!(),
            // -------------------------------------------------------------------------------------
        }
    }

    todo!();
}