mod ast;
mod executor;
mod lexer;
mod parser;

use std::f32::consts::E;

use ast::{ASTNode, FunName};
use clap::Parser as clapParser;

use crate::parser::parse_as_number;

#[derive(clapParser)]
struct Args {
    /// Path to a file
    file_path: std::path::PathBuf,

    /// Path to an svg or png image
    image_path: std::path::PathBuf,

    /// Height
    height: u32,

    /// Width
    width: u32,
}

struct Manager {
    variables: std::collections::HashMap<String, String>,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            variables: std::collections::HashMap::new(),
        }
    }

    fn dfs(&mut self, root: &mut ASTNode, executor: &mut Box<dyn executor::Executor>) {
        match root {
            ASTNode::Sequence(root) => {
                for node in root {
                    self.dfs(node, executor);
                }
            }
            ASTNode::FunctionCall(FunName, args) => match FunName {
                FunName::pen_up => executor.pen_up(),
                FunName::pen_down => executor.pen_down(),
                FunName::foreward => {
                    if let Some(args) = args {
                        assert!(args.len() == 1);
                        if let Some(v) = self.evaluate_prefix(&args[0], executor) {
                            executor.foreward(v);
                        } else {
                            panic!("not f32");
                        }
                    } else {
                        panic!("few argument");
                    }
                }
                FunName::back => {
                    if let Some(args) = args {
                        assert!(args.len() == 1);
                        if let Some(v) = self.evaluate_prefix(&args[0], executor) {
                            executor.back(v);
                        } else {
                            panic!("not f32");
                        }
                    } else {
                        panic!("few argument");
                    }
                }
                FunName::left => {
                    if let Some(args) = args {
                        assert!(args.len() == 1);
                        if let Some(v) = self.evaluate_prefix(&args[0], executor) {
                            executor.left(v);
                        } else {
                            panic!("not f32");
                        }
                    } else {
                        panic!("few argument");
                    }
                }
                FunName::right => {
                    if let Some(args) = args {
                        assert!(args.len() == 1);
                        if let Some(v) = self.evaluate_prefix(&args[0], executor) {
                            executor.right(v);
                        } else {
                            panic!("not f32");
                        }
                    } else {
                        panic!("few argument");
                    }
                }
                FunName::set_color => {
                    if let Some(args) = args {
                        assert!(args.len() == 1);
                        println!("{:?}", args);
                        if let Some(v) = self.evaluate_prefix(&args[0], executor) {
                            executor.set_color(v as u32);
                        } else {
                            panic!("not u32");
                        }
                    } else {
                        panic!("few argument");
                    }
                }
                FunName::turn => {
                    if let Some(args) = args {
                        assert!(args.len() == 1);
                        if let Some(v) = self.evaluate_prefix(&args[0], executor) {
                            executor.turn(v as i32);
                        } else {
                            panic!("not i32");
                        }
                    } else {
                        panic!("few argument");
                    }
                }
                FunName::set_heading => {
                    if let Some(args) = args {
                        assert!(args.len() == 1);
                        if let Some(v) = self.evaluate_prefix(&args[0], executor) {
                            executor.set_heading(v as i32);
                        } else {
                            panic!("not i32");
                        }
                    } else {
                        panic!("few argument");
                    }
                }
                FunName::set_x_coordinate => {
                    if let Some(args) = args {
                        assert!(args.len() == 1);
                        if let Some(v) = self.evaluate_prefix(&args[0], executor) {
                            executor.set_x_coordinate(v);
                        } else {
                            panic!("not f32");
                        }
                    } else {
                        panic!("few argument");
                    }
                }
                FunName::set_y_coordinate => {
                    if let Some(args) = args {
                        assert!(args.len() == 1);
                        if let Some(v) = self.evaluate_prefix(&args[0], executor) {
                            executor.set_y_coordinate(v);
                        } else {
                            panic!("not f32");
                        }
                    } else {
                        panic!("few argument");
                    }
                }
            },
            ASTNode::Define(name, expressions) => {
                if let Some(v) = self.evaluate_prefix(expressions, executor) {
                    self.variables.insert(name.clone(), v.to_string());
                } else {
                    panic!("failed to obtain expression val");
                }
            }
            ASTNode::PlusAnd(name, expressions) => {
                if let Some(v) = self.evaluate_prefix(expressions, executor) {
                    if let Some(old) = self.variables.get(name) {
                        let old = parse_as_number::<f32>(old).expect("error parse");
                        self.variables.insert(name.clone(), (v + old).to_string());
                    } else {
                        panic!("not defined variable for {}", name);
                    }
                } else {
                    panic!("failed to obtain expression val");
                }
            }
            _ => {
                panic!("Attempted to push into a non-Sequence variant of ASTNode");
            }
            _ => {
                panic!("Attempted to push into a non-Sequence variant of ASTNode");
            }
        }
    }

    pub fn get_variable_val(&self, name: &str) -> Option<f32> {
        for (k, v) in &self.variables {
            if k == name {
                if let Some(v) = parse_as_number::<f32>(v) {
                    return Some(v);
                } else {
                    panic!("cannot parse");
                }
            }
        }
        None
    }

    pub fn evaluate_prefix(
        &self,
        expressions: &str,
        executor: &mut Box<dyn executor::Executor>,
    ) -> Option<f32> {
        let mut stack: Vec<f32> = Vec::new();

        for expression in expressions.split_whitespace().into_iter().rev() {
            match expression {
                // query
                "XCOR" => stack.push(executor.get_x_coordinate()),
                "YCOR" => stack.push(executor.get_y_coordinate()),
                "HEADING" => stack.push(executor.get_heading() as f32),
                "COLOR" => stack.push(executor.get_color() as f32),
                "EQ" => {
                    if let (Some(op1), Some(op2)) = (stack.pop(), stack.pop()) {
                        stack.push(if op1 == op2 { 1.0 } else { 0.0 })
                    } else {
                        panic!("no enough opcode");
                    }
                }
                "NE" => {
                    if let (Some(op1), Some(op2)) = (stack.pop(), stack.pop()) {
                        stack.push(if op1 != op2 { 1.0 } else { 0.0 });
                    } else {
                        panic!("no enough opcode");
                    }
                }
                "GT" => {
                    if let (Some(op1), Some(op2)) = (stack.pop(), stack.pop()) {
                        stack.push(if op1 >= op2 { 1.0 } else { 0.0 });
                    } else {
                        panic!("no enough opcode");
                    }
                }
                "LT" => {
                    if let (Some(op1), Some(op2)) = (stack.pop(), stack.pop()) {
                        stack.push(if op1 <= op2 { 1.0 } else { 0.0 });
                    } else {
                        panic!("no enough opcode");
                    }
                }
                "AND" => {
                    if let (Some(op1), Some(op2)) = (stack.pop(), stack.pop()) {
                        stack.push(if op1 != 0.0 && op2 != 0.0 { 1.0 } else { 0.0 });
                    } else {
                        panic!("no enough opcode");
                    }
                }
                "OR" => {
                    if let (Some(op1), Some(op2)) = (stack.pop(), stack.pop()) {
                        stack.push(if op1 != 0.0 || op2 != 0.0 { 1.0 } else { 0.0 });
                    } else {
                        panic!("no enough opcode");
                    }
                }
                "+" => {
                    if let (Some(op1), Some(op2)) = (stack.pop(), stack.pop()) {
                        stack.push(op1 + op2);
                    } else {
                        panic!("no enough opcode");
                    }
                }
                "-" => {
                    if let (Some(op1), Some(op2)) = (stack.pop(), stack.pop()) {
                        stack.push(op1 - op2);
                    } else {
                        panic!("no enough opcode");
                    }
                }
                "*" => {
                    if let (Some(op1), Some(op2)) = (stack.pop(), stack.pop()) {
                        stack.push(op1 * op2);
                    } else {
                        panic!("no enough opcode");
                    }
                }
                "/" => {
                    if let (Some(op1), Some(op2)) = (stack.pop(), stack.pop()) {
                        if op2 == 0.0 {
                            panic!("divide by 0");
                        }
                        stack.push(op1 / op2);
                    } else {
                        panic!("no enough opcode");
                    }
                }

                // variable name
                _ => {
                    // literas
                    if let Some(val) = parse_as_number::<f32>(expression) {
                        stack.push(val);
                    } else {
                        println!("variable name: {}", expression);
                        if let Some(v) = self.get_variable_val(expression) {
                            stack.push(v);
                        } else {
                            // not defined variable;
                            debug_assert!(false);
                        }
                    }
                }
            }
        }

        stack.pop()
    }
}

fn main() -> Result<(), ()> {
    let args: Args = Args::parse();

    // Access the parsed arguments
    let file_path = args.file_path;
    let image_path = args.image_path;
    let height = args.height;
    let width = args.width;

    match file_path.extension().map(|s| s.to_str()).flatten() {
        Some("lg") => {}
        _ => {
            eprintln!("source file extension not supported");
            return Err(());
        }
    }

    let mut lexer = lexer::LexerFactory::create_lexer(&file_path);
    let mut parser = parser::Parser::new(&mut lexer);
    parser.run();
    let mut root: &mut ASTNode = parser.get_root();
    let mut executor = executor::ExecutorFactory::create_turtle(width, height, image_path);

    let mut manger: Manager = Manager::new();
    manger.dfs(root, &mut executor);
    Ok(())
}
