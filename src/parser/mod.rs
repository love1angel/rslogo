mod condition;
mod expression;

use std::str::FromStr;

use crate::ast::{ASTNode, FunName};
use crate::error::fatal_error;
use crate::lexer::{self, Lexer, Token, TokenType};
use crate::parser::expression::is_expression;
use crate::{error, Manager};

pub fn parse_as_number<T: FromStr>(s: &str) -> Option<T> {
    if let Ok(number) = s.parse::<T>() {
        Some(number)
    } else {
        None
    }
}

fn sequence_handing(name: FunName, tokens: &Vec<Token>) -> ASTNode {
    if tokens.len() < 2 {
        fatal_error(error::LogoError::NoEnoughArguments(tokens[0].souce.clone()));
    }
    assert!(tokens.len() >= 2);
    let mut joined_string = String::new();

    for i in 1..tokens.len() {
        if let Some(str) = is_expression(&tokens[i]) {
            if i != 1 {
                joined_string.push(' ');
            }
            joined_string.push_str(str);
        } else {
            fatal_error(error::LogoError::NotAexpression(tokens[i].souce.clone()));
            panic!("not a expresssion");
        }
    }
    ASTNode::FunctionCall(name, Some(vec![joined_string]))
}

fn plus_and_handling(tokens: &Vec<Token>) -> ASTNode {
    if tokens.len() < 2 {
        fatal_error(error::LogoError::NoEnoughArguments(tokens[0].souce.clone()));
    }
    if tokens[1].token_type != TokenType::Variable {
        fatal_error(error::LogoError::UnDefinedVariable(tokens[1].souce.clone()));
    }
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

pub struct Parser<'a, 'b>
where
    'a: 'b,
{
    depth: Vec<usize>,
    root: crate::parser::ASTNode,
    token_source: &'a mut Box<dyn Lexer>,
    runtime: &'b mut Manager,
    function_table: &'b mut std::collections::HashMap<String, Vec<ASTNode>>,
}

impl<'a, 'b> Parser<'a, 'b>
where
    'a: 'b,
{
    pub fn new(
        token_source: &'a mut Box<dyn Lexer>,
        runtime: &'b mut Manager,
        function_table: &'b mut std::collections::HashMap<String, Vec<ASTNode>>,
    ) -> Self {
        Parser {
            depth: Vec::new(),
            root: ASTNode::Sequence(Vec::new()),
            token_source,
            runtime,
            function_table,
        }
    }

    pub fn get_root(self) -> ASTNode {
        self.root
    }

    pub fn run(&mut self) {
        while let Some(tokens) = self.token_source.next_line_token() {
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
                // maybe user defined function
                // compile time resolve
                if self.runtime.func_vars.contains_key(&tokens[0].souce) {
                    let argument_size = self
                        .runtime
                        .func_vars
                        .get(&tokens[0].souce)
                        .expect("impossible")
                        .len();
                    if argument_size == 0 {
                        self.handle_user_defined_fn_call(&tokens[0].souce, &argument_size, None)
                    } else {
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
                        self.handle_user_defined_fn_call(
                            &tokens[0].souce,
                            &argument_size,
                            Some(joined_string),
                        )
                    }
                } else {
                    panic!("unknow error, {}", tokens[0].souce);
                }
            }
            TokenType::Keyword(key) => match key {
                lexer::keyword::Keyword::TRUE => todo!(),
                lexer::keyword::Keyword::FALSE => todo!(),
                lexer::keyword::Keyword::PENUP => {
                    assert!(tokens.len() == 1);
                    ASTNode::FunctionCall(FunName::PenUp, None)
                }
                lexer::keyword::Keyword::PENDOWN => {
                    if tokens.len() != 1 {
                        error::fatal_error(error::LogoError::TooManyArguments(
                            self.token_source.get_current_line_number(),
                            tokens[0].souce.clone(),
                        ));
                    }
                    ASTNode::FunctionCall(FunName::PenDown, None)
                }
                lexer::keyword::Keyword::FORWARD => sequence_handing(FunName::Foreward, &tokens),
                lexer::keyword::Keyword::BACK => sequence_handing(FunName::Back, &tokens),
                lexer::keyword::Keyword::LEFT => sequence_handing(FunName::Left, &tokens),
                lexer::keyword::Keyword::RIGHT => sequence_handing(FunName::Right, &tokens),
                lexer::keyword::Keyword::SETPENCOLOR => {
                    sequence_handing(FunName::SetColor, &tokens)
                }
                lexer::keyword::Keyword::TURN => sequence_handing(FunName::Turn, &tokens),
                lexer::keyword::Keyword::SETHEADING => {
                    sequence_handing(FunName::SetHeading, &tokens)
                }
                lexer::keyword::Keyword::SETX => sequence_handing(FunName::SetXCoordinate, &tokens),
                lexer::keyword::Keyword::SETY => sequence_handing(FunName::SetYCoordinate, &tokens),

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
                lexer::keyword::Keyword::FBegin => self.parse_function(&tokens),
                lexer::keyword::Keyword::FEnd => {
                    fatal_error(error::LogoError::FunctionDefineFailed(
                        self.token_source.get_current_line_number(),
                        "not define a function, but meet END".to_string(),
                    ));
                    todo!()
                }
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
                fatal_error(error::LogoError::UnvalidIfOrWhile(
                    self.token_source.get_current_line_number(),
                    "not meet ]".to_string(),
                ));
                panic!("not found ]");
            }
        }
        block
    }

    pub fn parse_if_while(&mut self, tokens: &Vec<Token>) -> Option<ASTNode> {
        if tokens[tokens.len() - 1].token_type != TokenType::LSBracket {
            fatal_error(error::LogoError::UnvalidIfOrWhile(
                self.token_source.get_current_line_number(),
                "not meet [".to_string(),
            ));
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

    // @todo
    pub fn handle_user_defined_fn_call(
        &mut self,
        name: &str,
        _: &usize,
        expression: Option<String>,
    ) -> ASTNode {
        ASTNode::CustomFunction(name.to_owned(), expression)
    }

    pub fn parse_function(&mut self, tokens: &Vec<Token>) -> ASTNode {
        assert!(tokens.len() >= 2);

        let mut block = Vec::new();
        let mut size = 0;
        let mut vars_name = Vec::new();
        for idx in 2..tokens.len() {
            if tokens[idx].token_type != TokenType::Variable {
                panic!("argumentnot a variable fine");
            } else {
                vars_name.push(tokens[idx].souce.clone());
                size = size + 1;
            }
        }

        let func_name = &tokens[1].souce;
        self.runtime.func_vars.insert(func_name.clone(), vars_name);

        loop {
            if let Some(tokens) = self.token_source.next_line_token() {
                if tokens.len() == 1
                    && tokens[0].token_type == TokenType::Keyword(lexer::keyword::Keyword::FEnd)
                {
                    break;
                } else {
                    block.push(self.handle_token(tokens));
                }
            } else {
                fatal_error(error::LogoError::FunctionDefineFailed(
                    self.token_source.get_current_line_number(),
                    "not found function define END".to_string(),
                ));
            }
        }
        self.function_table.insert(func_name.to_string(), block);

        ASTNode::Sequence(Vec::new())
    }
}
