use crate::parser::Node;

use std::iter::Peekable;
use std::slice::Iter;

pub struct RuntimeError {
    message: String,
    line: u64,
}

impl RuntimeError {
    pub fn new(message: String, line: u64) -> Self {
        RuntimeError {
            message,
            line
        }
    }
}


// I don't think I need to seperate interpret from interpreter as I do not think I will need to call recursion
fn interpret(instructions: Vec<Node>) {
    let iter: Peekable<Iter<Node>> = instructions.iter().peekable();

    for instruction in iter {
        match instruction {
            Node::Int(_) => todo!(),
            Node::String(_) => todo!(),
            Node::Bool(_) => todo!(),
            Node::UnaryExpr { op: _, child: _ } => todo!(),
            Node::BinaryExpr { op: _, lhs: _, rhs: _ } => todo!(),
            Node::CombineStr { lhs: _, rhs: _ } => todo!(),
            Node::Repeat { count: _, nodes: _ } => todo!(),
            Node::Eof => todo!(),
            Node::SetVariable { name: _, value: _ } => todo!(),
            Node::Function { name: _, nodes: _ } => todo!(),
            Node::Compare { comparator, lhs, rhs } => todo!(),
        }
    }
}