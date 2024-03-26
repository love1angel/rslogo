mod ast;
mod error;
mod executor;
mod lexer;
mod parser;

use ast::{ASTNode, FunName};
use clap::Parser as clapParser;

use crate::{error::fatal_error, error::LogoError, parser::parse_as_number};

#[derive(clapParser)]
struct Args {
    /// Path to a file
    file_path: std::path::PathBuf,

    /// Path to an svg or png image
    image_path: std::path::PathBuf,

    /// Height
    height: u32,
    width: u32,
}

struct Manager {
    variables: std::collections::HashMap<String, String>,

    pub func_vars: std::collections::HashMap<String, Vec<String>>,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            variables: std::collections::HashMap::new(),
            func_vars: std::collections::HashMap::new(),
        }
    }

    fn dfs(
        &mut self,
        root: &ASTNode,
        executor: &mut Box<dyn executor::Executor>,
        runtime: &std::collections::HashMap<String, Vec<ASTNode>>,
    ) {
        match root {
            ASTNode::Sequence(root) => {
                for node in root {
                    self.dfs(node, executor, runtime);
                }
            }
            ASTNode::FunctionCall(fun_name, args) => match fun_name {
                FunName::PenUp => executor.pen_up(),
                FunName::PenDown => executor.pen_down(),
                FunName::Foreward => {
                    if let Some(args) = args {
                        let mut stack = Vec::new();
                        if let Some(v) = self.evaluate_prefix(&mut stack, &args[0], executor) {
                            if stack.len() != 0 {
                                let mut str = String::new();
                                for i in 0..args.len() {
                                    str.push_str(&args[i]);
                                    str.push(' ');
                                }
                                fatal_error(LogoError::NotAexpression("FOREWARD".to_string(), str));
                            }
                            assert!(args.len() == 1);
                            executor.foreward(v);
                        } else {
                            panic!("not f32");
                        }
                    } else {
                        panic!("few argument");
                    }
                }
                FunName::Back => {
                    if let Some(args) = args {
                        let mut stack = Vec::new();
                        if let Some(v) = self.evaluate_prefix(&mut stack, &args[0], executor) {
                            if stack.len() != 0 {
                                let mut str = String::new();
                                for i in 0..args.len() {
                                    str.push_str(&args[i]);
                                    str.push(' ');
                                }
                                fatal_error(LogoError::NotAexpression("BACK".to_string(), str));
                            }
                            assert!(args.len() == 1);
                            executor.back(v);
                        } else {
                            panic!("not f32");
                        }
                    } else {
                        panic!("few argument");
                    }
                }
                FunName::Left => {
                    if let Some(args) = args {
                        let mut stack: Vec<f32> = Vec::new();
                        if let Some(v) = self.evaluate_prefix(&mut stack, &args[0], executor) {
                            if stack.len() != 0 {
                                let mut str = String::new();
                                for i in 0..args.len() {
                                    str.push_str(&args[i]);
                                    str.push(' ');
                                }
                                fatal_error(LogoError::NotAexpression("LEFT".to_string(), str));
                            }
                            assert!(args.len() == 1);
                            executor.left(v);
                        } else {
                            panic!("not f32");
                        }
                    } else {
                        panic!("few argument");
                    }
                }
                FunName::Right => {
                    if let Some(args) = args {
                        let mut stack = Vec::new();
                        if let Some(v) = self.evaluate_prefix(&mut stack, &args[0], executor) {
                            if stack.len() != 0 {
                                let mut str = String::new();
                                for i in 0..args.len() {
                                    str.push_str(&args[i]);
                                    str.push(' ');
                                }
                                fatal_error(LogoError::NotAexpression("RIGHT".to_string(), str));
                            }
                            assert!(args.len() == 1);
                            executor.right(v);
                        } else {
                            panic!("not f32");
                        }
                    } else {
                        panic!("few argument");
                    }
                }
                FunName::SetColor => {
                    if let Some(args) = args {
                        let mut stack = Vec::new();
                        if let Some(v) = self.evaluate_prefix(&mut stack, &args[0], executor) {
                            if stack.len() != 0 {
                                let mut str = String::new();
                                for i in 0..args.len() {
                                    str.push_str(&args[i]);
                                    str.push(' ');
                                }
                                fatal_error(LogoError::NotAexpression("SETCOLOR".to_string(), str));
                            }
                            assert!(args.len() == 1);
                            executor.set_color(v as u32);
                        } else {
                            panic!("not u32");
                        }
                    } else {
                        panic!("few argument");
                    }
                }
                FunName::Turn => {
                    if let Some(args) = args {
                        let mut stack = Vec::new();
                        if let Some(v) = self.evaluate_prefix(&mut stack, &args[0], executor) {
                            if stack.len() != 0 {
                                let mut str = String::new();
                                for i in 0..args.len() {
                                    str.push_str(&args[i]);
                                    str.push(' ');
                                }
                                fatal_error(LogoError::NotAexpression("TURN".to_string(), str));
                            }
                            assert!(args.len() == 1);
                            executor.turn(v as i32);
                        } else {
                            panic!("not i32");
                        }
                    } else {
                        panic!("few argument");
                    }
                }
                FunName::SetHeading => {
                    if let Some(args) = args {
                        let mut stack = Vec::new();
                        if let Some(v) = self.evaluate_prefix(&mut stack, &args[0], executor) {
                            if stack.len() != 0 {
                                let mut str = String::new();
                                for i in 0..args.len() {
                                    str.push_str(&args[i]);
                                    str.push(' ');
                                }
                                fatal_error(LogoError::NotAexpression(
                                    "SETHEADING".to_string(),
                                    str,
                                ));
                            }
                            assert!(args.len() == 1);
                            executor.set_heading(v as i32);
                        } else {
                            panic!("not i32");
                        }
                    } else {
                        panic!("few argument");
                    }
                }
                FunName::SetXCoordinate => {
                    if let Some(args) = args {
                        let mut stack = Vec::new();
                        if let Some(v) = self.evaluate_prefix(&mut stack, &args[0], executor) {
                            if stack.len() != 0 {
                                let mut str = String::new();
                                for i in 0..args.len() {
                                    str.push_str(&args[i]);
                                    str.push(' ');
                                }
                                fatal_error(LogoError::NotAexpression("SETX".to_string(), str));
                            }
                            assert!(args.len() == 1);
                            executor.set_x_coordinate(v);
                        } else {
                            panic!("not f32");
                        }
                    } else {
                        panic!("few argument");
                    }
                }
                FunName::SetYCoordinate => {
                    if let Some(args) = args {
                        let mut stack: Vec<f32> = Vec::new();
                        if let Some(v) = self.evaluate_prefix(&mut stack, &args[0], executor) {
                            if stack.len() != 0 {
                                let mut str = String::new();
                                for i in 0..args.len() {
                                    str.push_str(&args[i]);
                                    str.push(' ');
                                }
                                fatal_error(LogoError::NotAexpression("SETY".to_string(), str));
                            }
                            assert!(args.len() == 1);
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
                let mut stack: Vec<f32> = Vec::new();

                if let Some(v) = self.evaluate_prefix(&mut stack, expressions, executor) {
                    self.variables.insert(name.clone(), v.to_string());
                } else {
                    panic!("failed to obtain expression val");
                }
            }
            ASTNode::PlusAnd(name, expressions) => {
                let mut stack: Vec<f32> = Vec::new();

                if let Some(v) = self.evaluate_prefix(&mut stack, expressions, executor) {
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
            ASTNode::If(expression, block) => {
                if let ASTNode::Expersion(str) = &**expression {
                    // 在这里处理 expression 是 Expersion 的情况
                    let mut stack: Vec<f32> = Vec::new();

                    if let Some(val) = self.evaluate_prefix(&mut stack, &str, executor) {
                        if val != 0.0 {
                            // 执行 IF 语句块
                            for statement in block {
                                self.dfs(statement, executor, runtime);
                            }
                        }
                    }
                } else {
                    panic!("if doesn't meet expression");
                }
            }
            ASTNode::While(expression, block) => {
                if let ASTNode::Expersion(str) = &**expression {
                    loop {
                        let mut stack: Vec<f32> = Vec::new();

                        if let Some(val) = self.evaluate_prefix(&mut stack, &str, executor) {
                            if val != 0.0 {
                                // 执行 IF 语句块
                                for statement in &*block {
                                    self.dfs(statement, executor, runtime);
                                }
                            } else {
                                break;
                            }
                        }
                    }
                } else {
                    panic!("if doesn't meet expression");
                }
            }
            ASTNode::CustomFunction(func_name, expression) => {
                let argument_size = self.func_vars[func_name].len();

                if argument_size == 0 {
                    if let None = expression {
                        let statements = &runtime[func_name];
                        for node in statements {
                            self.dfs(&node, executor, runtime);
                        }
                    } else {
                        panic!("no arguments");
                    }
                } else {
                    if let Some(expression) = expression {
                        // calcatue parameter value;
                        let mut stack: Vec<f32> = Vec::new();
                        if let Some(val) = self.evaluate_prefix(&mut stack, &expression, executor) {
                            stack.push(val);
                        } else {
                            panic!("no enough para");
                        }

                        if stack.len() != argument_size {
                            panic!("no enough parameter value");
                        }

                        //
                        let mut odd: std::collections::HashMap<String, String> =
                            std::collections::HashMap::new(); // arguments

                        for var_name in self.func_vars[func_name].iter() {
                            if self.variables.contains_key(var_name) {
                                odd.insert(
                                    var_name.clone(),
                                    self.variables.get(var_name).expect("unreach").clone(),
                                );
                            }
                            self.variables.insert(
                                var_name.clone(),
                                stack.pop().expect("unreach").to_string(),
                            );
                        }
                        let statements = &runtime[func_name];
                        for node in statements {
                            self.dfs(&node, executor, runtime);
                        }

                        for var_name in self.func_vars[func_name].iter().rev() {
                            if odd.contains_key(var_name) {
                                self.variables
                                    .insert(var_name.clone(), odd[var_name].clone());
                            } else {
                                self.variables.remove(var_name);
                            }
                        }
                    }
                }
            }
            _ => {
                panic!("Attempted to push into a non-Sequence variant of ASTNode");
            }
        }
    }

    pub fn get_variable_val(&self, name: &str) -> Option<f32> {
        for (k, v) in &self.variables {
            if k == &name[1..] {
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
        stack: &mut Vec<f32>,
        expressions: &str,
        executor: &mut Box<dyn executor::Executor>,
    ) -> Option<f32> {
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
                        stack.push(if op1 > op2 { 1.0 } else { 0.0 });
                    } else {
                        panic!("no enough opcode");
                    }
                }
                "LT" => {
                    if let (Some(op1), Some(op2)) = (stack.pop(), stack.pop()) {
                        stack.push(if op1 < op2 { 1.0 } else { 0.0 });
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
                        // println!("variable name: {}", expression);
                        if let Some(v) = self.get_variable_val(expression) {
                            stack.push(v);
                        } else {
                            // not defined variable;
                            fatal_error(LogoError::UnDefinedVariable(expression.to_string()));
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

    let mut manger: Manager = Manager::new();

    let mut lexer = lexer::LexerFactory::create_lexer(&file_path);
    let mut function_table: std::collections::HashMap<String, Vec<ASTNode>> =
        std::collections::HashMap::new();
    let mut parser = parser::Parser::new(&mut lexer, &mut manger, &mut function_table);
    parser.run();

    let root: ASTNode = parser.get_root();
    let mut executor = executor::ExecutorFactory::create_turtle(width, height, image_path);

    manger.dfs(&root, &mut executor, &function_table);
    Ok(())
}
