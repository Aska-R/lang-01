use crate::parser::Node;

use std::iter::Peekable;
use std::slice::Iter;

#[cfg(test)]
mod tests;


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

#[derive(Debug, PartialEq, Clone)]
pub enum Variable {
    Int {
        name: String,
        num: i64
    },
    String {
        name: String,
        str: String
    },
    Null,
}

impl Variable {
    // I should work out how to use self but with vectors at some point
    fn get_variable(variables: Vec<Variable>, find_name: String) -> Result<Variable, RuntimeError> {
        for variable in variables {
            match variable {
                Variable::Int { ref name, num: _  } => {
                    if *name == find_name {
                        return Ok(variable);
                    }
                },
                Variable::String { ref name, str: _ } => {
                    if *name == find_name {
                        return Ok(variable);
                    }
                },

                // Error handling
                Variable::Null => {
                    return Err(RuntimeError::new(
                        "Null variable has been saved to variable storage".to_string()
                    ));
                }
            }
        }

        return Ok(Variable::Null);
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    name: String,
    nodes: Vec<Node>,
    args: Vec<Variable>
}

impl Function {
    fn new(name: String, nodes: Vec<Node>, args: Vec<Variable>) -> Function {
        return Function { name, nodes, args };
    }

    fn null() -> Function {
        let nodes: Vec<Node> = Vec::new();
        let args: Vec<Variable> = Vec::new();
        
        return Function { name: "null".to_string(), nodes, args}
    }

    fn get_function(functions: Vec<Function>, find_name: String) -> Function {
        for function in functions {
            if function.name == find_name {
                return function;
            }
        }

        return Self::null();
    }

    pub fn run_function(mut functions: Vec<Function>, function_name: String) -> Result<(), RuntimeError> {
        let function: Function = Function::get_function(functions.clone(), function_name);

        if function.name == "null".to_string() {
            return Err(RuntimeError::new("Failed to find function".to_string()));
        }

        let mut variables: Vec<Variable> = function.args;
        // variables here would be the arguments passed into the function
        return Ok(interpret(function.nodes, &mut variables, &mut functions)?);
    }   

    pub fn create_function(functions: &mut Vec<Function>, function_name: String, function_nodes: Vec<Node>, args: Vec<Variable>) {
        let function: Function = Function { name: function_name, nodes: function_nodes, args};

        functions.push(function);
    }
}


// I don't think I need to seperate interpret from interpreter as I do not think I will need to call recursion
pub fn interpret(instructions: Vec<Node>, variables: &mut Vec<Variable>, functions: &mut Vec<Function>) -> Result<(), RuntimeError> {
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
            Node::SetVariable { var: _ } => {
                //variables.push(var.clone());
            },
            // Run function
            Node::Function { name, args: _ } => {
                // Error prone line
                Function::run_function(functions.to_vec(), name.to_string()).unwrap();
            },
            // Define function
            Node::DefineFunction { name: _, nodes: _, args: _ } => {
                todo!();
            },

            // Comparisons
            Node::IfBinaryCompare { comparator: _, lhs: _, rhs: _, nodes: _ } => todo!(),
            Node::IfUnaryCompare { expected: _, actual: _, nodes: _ } => todo!(),
            Node::IfElseBinaryCompare { comparator: _, lhs: _, rhs: _, nodes: _ } => todo!(),
            Node::IfElseUnaryCompare { expected: _, actual: _, nodes: _ } => todo!(),
            
            // EOF
            Node::Eof => return Ok(()),
            
            // Built-in functions
            Node::Print { str } => println!("{str}"),

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
    
    return Ok(());
}

