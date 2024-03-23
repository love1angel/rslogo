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

fn dfs(root: &mut ASTNode, executor: &mut Box<dyn executor::Executor>) {
    match root {
        ASTNode::Sequence(root) => {
            for node in root {
                dfs(node, executor);
            }
        }
        ASTNode::FunctionCall(FunName, args) => match FunName {
            FunName::pen_up => executor.pen_up(),
            FunName::pen_down => executor.pen_down(),
            FunName::foreward => {
                if let Some(args) = args {
                    assert!(args.len() == 1);
                    if let Some(v) = parse_as_number::<f32>(&args[0]) {
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
                    if let Some(v) = parse_as_number::<f32>(&args[0]) {
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
                    if let Some(v) = parse_as_number::<f32>(&args[0]) {
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
                    if let Some(v) = parse_as_number::<f32>(&args[0]) {
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
                    if let Some(v) = parse_as_number::<u32>(&args[0]) {
                        executor.set_color(v);
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
                    if let Some(v) = parse_as_number::<i32>(&args[0]) {
                        executor.turn(v);
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
                    if let Some(v) = parse_as_number::<i32>(&args[0]) {
                        executor.set_heading(v);
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
                    if let Some(v) = parse_as_number::<f32>(&args[0]) {
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
                    if let Some(v) = parse_as_number::<f32>(&args[0]) {
                        executor.set_y_coordinate(v);
                    } else {
                        panic!("not f32");
                    }
                } else {
                    panic!("few argument");
                }
            }
        },
        _ => {
            panic!("Attempted to push into a non-Sequence variant of ASTNode");
        }
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
    let mut executor = executor::ExecutorFactory::create_turtle(100, 100, image_path);
    dfs(root, &mut executor);
    Ok(())
}
