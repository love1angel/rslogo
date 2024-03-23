use std::str::FromStr;

use crate::ast::{ASTNode, FunName};
use crate::lexer::{self, Lexer, Token, TokenType};

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

fn is_expression(token: &Token) -> Option<&str> {
    // expression: procudure query literal
    // operator
    match &token.token_type {
        TokenType::Invalid => {
            panic!("not a expression");
        }
        TokenType::Keyword(key) => {
            match key {
                lexer::keyword::Keyword::EQ => Some(&token.souce),
                lexer::keyword::Keyword::NE => Some(&token.souce),
                lexer::keyword::Keyword::GT => Some(&token.souce),
                lexer::keyword::Keyword::LT => Some(&token.souce),
                lexer::keyword::Keyword::AND => Some(&token.souce),
                lexer::keyword::Keyword::OR => Some(&token.souce),
                lexer::keyword::Keyword::Plus => Some(&token.souce),
                lexer::keyword::Keyword::Minus => Some(&token.souce),
                lexer::keyword::Keyword::Multipliy => Some(&token.souce),
                lexer::keyword::Keyword::Divide => Some(&token.souce),
                _ => {
                    // panic!("unknown key");
                    None
                }
            }
        }
        TokenType::Float(f) => Some(f),
        TokenType::Variable => None,
        TokenType::Procedure => Some(&token.souce),
        TokenType::Query(_) => Some(&token.souce),
        TokenType::LRrace => None,
        TokenType::RBrace => None,
        TokenType::LSBracket => None,
        TokenType::RSBracket => None,
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
            Parser::handle_Token(tokens, &mut self.root);
        }

        loop {
            if let Some(tokens) = self.token_source.next_line_token() {
            } else {
                break;
            }
        }
    }

    fn handle_Token(tokens: Vec<Token>, node: &mut ASTNode) {
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

                lexer::keyword::Keyword::MAKE => todo!(),
                lexer::keyword::Keyword::ADDASSIGN => todo!(),
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
