mod expression;

use std::str::FromStr;

use crate::ast::{ASTNode, FunName};
use crate::lexer::variable::is_variable;
use crate::lexer::{self, Lexer, Token, TokenType};
use crate::parser::expression::is_expression;

pub fn parse_as_number<T: FromStr>(s: &str) -> Option<T> {
    if let Ok(number) = s.parse::<T>() {
        Some(number)
    } else {
        None
    }
}

fn sequence_handing(name: FunName, tokens: &Vec<Token>, node: &mut ASTNode) {
    assert!(tokens.len() >= 2);
    match node {
        ASTNode::Sequence(vec) => {
            let mut joined_string = String::new();

            for i in 1..tokens.len() {
                if let Some(str) = is_expression(&tokens[i]) {
                    if i != 1 {
                        joined_string.push(' ');
                    }
                    joined_string.push_str(str);
                } else {
                    panic!("not a expresssion");
                }
            }

            vec.push(ASTNode::FunctionCall(name, Some(vec![joined_string])));
        }
        _ => {
            panic!("Attempted to push into a non-Sequence variant of ASTNode");
        }
    }
}

fn plus_and_handling(tokens: &Vec<Token>, node: &mut ASTNode) {
    assert!(tokens.len() >= 2 && tokens[1].token_type == TokenType::Variable);
    match node {
        ASTNode::Sequence(vec) => {
            let mut joined_string = String::new();

            for i in 2..tokens.len() {
                if let Some(str) = is_expression(&tokens[i]) {
                    if i != 2 {
                        joined_string.push(' ');
                    }
                    joined_string.push_str(str);
                } else {
                    panic!("not a expresssion");
                }
            }

            vec.push(ASTNode::PlusAnd(tokens[1].souce.to_string(), joined_string));
        }
        _ => {
            panic!("Attempted to push into a non-Sequence variant of ASTNode");
        }
    }
}

fn define_handing(tokens: &Vec<Token>, node: &mut ASTNode) {
    assert!(tokens.len() >= 2 && tokens[1].token_type == TokenType::Variable);
    match node {
        ASTNode::Sequence(vec) => {
            let mut joined_string = String::new();

            for i in 2..tokens.len() {
                if let Some(str) = is_expression(&tokens[i]) {
                    if i != 2 {
                        joined_string.push(' ');
                    }
                    joined_string.push_str(str);
                } else {
                    panic!("not a expresssion");
                }
            }

            vec.push(ASTNode::Define(tokens[1].souce.to_string(), joined_string));
        }
        _ => {
            panic!("Attempted to push into a non-Sequence variant of ASTNode");
        }
    }
}

pub struct Parser<'a> {
    root: crate::parser::ASTNode,
    current: Vec<Token>,
    token_source: &'a mut Box<dyn Lexer>,
}

impl<'a> Parser<'a> {
    pub fn new(token_source: &'a mut Box<dyn Lexer>) -> Self {
        Parser {
            root: ASTNode::Sequence(Vec::new()),
            current: Vec::new(),
            token_source,
        }
    }

    pub fn get_root(&mut self) -> &mut ASTNode {
        &mut self.root
    }

    pub fn run(&mut self) {
        while let Some(tokens) = self.token_source.next_line_token() {
            println!(
                "line: {}, Token are: {:?}",
                self.token_source.get_current_line_number(),
                tokens
            );
            Parser::handle_token(tokens, &mut self.root);
        }

        loop {
            if let Some(tokens) = self.token_source.next_line_token() {
            } else {
                break;
            }
        }
    }

    fn handle_token(tokens: Vec<Token>, node: &mut ASTNode) {
        match &tokens[0].token_type {
            TokenType::Invalid => {
                panic!("unknow error, {}", tokens[0].souce);
            }
            TokenType::Keyword(key) => match key {
                lexer::keyword::Keyword::TRUE => todo!(),
                lexer::keyword::Keyword::FALSE => todo!(),
                lexer::keyword::Keyword::PENUP => {
                    assert!(tokens.len() == 1);
                    match node {
                        ASTNode::Sequence(vec) => {
                            vec.push(ASTNode::FunctionCall(FunName::pen_up, None));
                        }
                        _ => {
                            panic!("Attempted to push into a non-Sequence variant of ASTNode");
                        }
                    }
                }
                lexer::keyword::Keyword::PENDOWN => {
                    assert!(tokens.len() == 1);
                    match node {
                        ASTNode::Sequence(vec) => {
                            vec.push(ASTNode::FunctionCall(FunName::pen_down, None));
                        }
                        _ => {
                            panic!("Attempted to push into a non-Sequence variant of ASTNode");
                        }
                    }
                }
                lexer::keyword::Keyword::FORWARD => {
                    sequence_handing(FunName::foreward, &tokens, node)
                }
                lexer::keyword::Keyword::BACK => sequence_handing(FunName::back, &tokens, node),
                lexer::keyword::Keyword::LEFT => sequence_handing(FunName::left, &tokens, node),
                lexer::keyword::Keyword::RIGHT => sequence_handing(FunName::right, &tokens, node),
                lexer::keyword::Keyword::SETPENCOLOR => {
                    sequence_handing(FunName::set_color, &tokens, node)
                }
                lexer::keyword::Keyword::TURN => sequence_handing(FunName::turn, &tokens, node),
                lexer::keyword::Keyword::SETHEADING => {
                    sequence_handing(FunName::set_heading, &tokens, node)
                }
                lexer::keyword::Keyword::SETX => {
                    sequence_handing(FunName::set_x_coordinate, &tokens, node)
                }
                lexer::keyword::Keyword::SETY => {
                    sequence_handing(FunName::set_y_coordinate, &tokens, node)
                }

                lexer::keyword::Keyword::MAKE => define_handing(&tokens, node),
                lexer::keyword::Keyword::ADDASSIGN => plus_and_handling(&tokens, node),
                lexer::keyword::Keyword::IF => todo!(),
                lexer::keyword::Keyword::WHILE => todo!(),
                lexer::keyword::Keyword::EQ => todo!(),
                lexer::keyword::Keyword::NE => todo!(),
                lexer::keyword::Keyword::GT => todo!(),
                lexer::keyword::Keyword::LT => todo!(),
                lexer::keyword::Keyword::AND => todo!(),
                lexer::keyword::Keyword::OR => todo!(),
                lexer::keyword::Keyword::Plus => todo!(),
                lexer::keyword::Keyword::Minus => todo!(),
                lexer::keyword::Keyword::Multipliy => todo!(),
                lexer::keyword::Keyword::Divide => todo!(),
                lexer::keyword::Keyword::FBegin => todo!(),
                lexer::keyword::Keyword::FEnd => todo!(),
            },
            TokenType::Float(_) => todo!(),
            TokenType::Variable => todo!(),
            TokenType::Procedure => todo!(),
            TokenType::Query(_) => todo!(),
            TokenType::LRrace => todo!(),
            TokenType::RBrace => todo!(),
            TokenType::LSBracket => todo!(),
            TokenType::RSBracket => todo!(),
        }
    }
}
