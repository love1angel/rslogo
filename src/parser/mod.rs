mod condition;
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

fn sequence_handing(name: FunName, tokens: &Vec<Token>) -> ASTNode {
    assert!(tokens.len() >= 2);
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
    ASTNode::FunctionCall(name, Some(vec![joined_string]))
}

fn plus_and_handling(tokens: &Vec<Token>) -> ASTNode {
    assert!(tokens.len() >= 2 && tokens[1].token_type == TokenType::Variable);
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
    ASTNode::PlusAnd(tokens[1].souce.to_string(), joined_string)
}

fn define_handing(tokens: &Vec<Token>) -> ASTNode {
    assert!(tokens.len() >= 2 && tokens[1].token_type == TokenType::Variable);
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
    ASTNode::Define(tokens[1].souce.to_string(), joined_string)
}

pub struct Parser<'a> {
    depth: Vec<usize>,
    root: crate::parser::ASTNode,
    current: Vec<Token>,
    token_source: &'a mut Box<dyn Lexer>,
}

impl<'a> Parser<'a> {
    pub fn new(token_source: &'a mut Box<dyn Lexer>) -> Self {
        Parser {
            depth: Vec::new(),
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
            let new_node = self.handle_token(tokens);
            match &mut self.root {
                ASTNode::Sequence(vec) => {
                    vec.push(new_node);
                }
                _ => {
                    panic!("Attempted to push into a non-Sequence variant of ASTNode");
                }
            }
        }
    }

    fn handle_token(&mut self, tokens: Vec<Token>) -> ASTNode {
        match &tokens[0].token_type {
            TokenType::Invalid => {
                panic!("unknow error, {}", tokens[0].souce);
            }
            TokenType::Keyword(key) => match key {
                lexer::keyword::Keyword::TRUE => todo!(),
                lexer::keyword::Keyword::FALSE => todo!(),
                lexer::keyword::Keyword::PENUP => {
                    assert!(tokens.len() == 1);
                    ASTNode::FunctionCall(FunName::pen_up, None)
                }
                lexer::keyword::Keyword::PENDOWN => {
                    assert!(tokens.len() == 1);
                    ASTNode::FunctionCall(FunName::pen_down, None)
                }
                lexer::keyword::Keyword::FORWARD => sequence_handing(FunName::foreward, &tokens),
                lexer::keyword::Keyword::BACK => sequence_handing(FunName::back, &tokens),
                lexer::keyword::Keyword::LEFT => sequence_handing(FunName::left, &tokens),
                lexer::keyword::Keyword::RIGHT => sequence_handing(FunName::right, &tokens),
                lexer::keyword::Keyword::SETPENCOLOR => {
                    sequence_handing(FunName::set_color, &tokens)
                }
                lexer::keyword::Keyword::TURN => sequence_handing(FunName::turn, &tokens),
                lexer::keyword::Keyword::SETHEADING => {
                    sequence_handing(FunName::set_heading, &tokens)
                }
                lexer::keyword::Keyword::SETX => {
                    sequence_handing(FunName::set_x_coordinate, &tokens)
                }
                lexer::keyword::Keyword::SETY => {
                    sequence_handing(FunName::set_y_coordinate, &tokens)
                }

                lexer::keyword::Keyword::MAKE => define_handing(&tokens),
                lexer::keyword::Keyword::ADDASSIGN => plus_and_handling(&tokens),
                lexer::keyword::Keyword::IF => {
                    if let Some(new_node) = self.parse_if_while(&tokens) {
                        new_node
                    } else {
                        panic!("no new root");
                    }
                }
                lexer::keyword::Keyword::WHILE => {
                    if let Some(new_node) = self.parse_if_while(&tokens) {
                        new_node
                    } else {
                        panic!("no new root");
                    }
                }
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

    fn parse_block(&mut self) -> Vec<ASTNode> {
        let mut block = Vec::new();
        loop {
            if let Some(tokens) = self.token_source.next_line_token() {
                if tokens.len() >= 3 && tokens[tokens.len() - 1].token_type == TokenType::LSBracket
                {
                    // 进入一个新的深度
                    self.depth.push(self.depth.len() + 1);
                }
                if tokens.len() == 1 && tokens[0].token_type == TokenType::RSBracket {
                    // 退出当前深度
                    self.depth.pop();
                    break;
                    // if self.depth.is_empty() {
                    //     // 当前语句块结束
                    //     break;
                    // }
                }
                // 如果当前行包含 IF 或 WHILE，递归调用解析方法
                if tokens.len() >= 3
                    && tokens[0].token_type == TokenType::Keyword(lexer::keyword::Keyword::IF)
                    || tokens.len() >= 3
                        && tokens[0].token_type
                            == TokenType::Keyword(lexer::keyword::Keyword::WHILE)
                {
                    if let Some(node) = self.parse_if_while(&tokens) {
                        block.push(node);
                    } else {
                        panic!("unvalid if or while");
                    }
                } else {
                    // 否则解析表达式并加入语句块
                    let expression = self.handle_token(tokens);
                    block.push(expression);
                }
            } else {
                panic!("not found ]");
            }
        }
        block
    }

    pub fn parse_if_while(&mut self, tokens: &Vec<Token>) -> Option<ASTNode> {
        if tokens[tokens.len() - 1].token_type != TokenType::LSBracket {
            panic!("unvalid if or while");
        }
        // assert!()
        if tokens[0].token_type == TokenType::Keyword(lexer::keyword::Keyword::IF) {
            let expression = Self::parse_expression(&tokens);
            // 继续解析条件执行的语句块，可能包含内部的 IF 和 WHILE
            let block = self.parse_block();
            Some(ASTNode::If(Box::new(expression), block))
        } else if tokens[0].token_type == TokenType::Keyword(lexer::keyword::Keyword::WHILE) {
            // 解析 WHILE 语句
            let expression = Self::parse_expression(&tokens);
            // 继续解析条件循环执行的语句块，可能包含内部的 IF 和 WHILE
            let block = self.parse_block();
            Some(ASTNode::While(Box::new(expression), block))
        } else {
            // 无法读取更多行，返回 None
            None
        }
    }

    fn parse_expression(tokens: &Vec<Token>) -> ASTNode {
        let mut joined_string = String::new();

        for i in 1..tokens.len() - 1 {
            if let Some(str) = is_expression(&tokens[i]) {
                if i != 0 {
                    joined_string.push(' ');
                }
                joined_string.push_str(str);
            } else {
                panic!("not a expresssion");
            }
        }
        ASTNode::Expersion(joined_string)
    }
}
