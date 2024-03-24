use crate::lexer::{keyword, literal, query, token::Token, token_type::TokenType};

use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::PathBuf,
};

use super::{procedure, variable, Lexer};

pub struct LexerImpl {
    buf_reader: BufReader<File>,
    line_number: usize,
}

fn is_comment(s: &str) -> bool {
    match s {
        "//" => true,
        _ => false,
    }
}

impl LexerImpl {
    pub fn new(source: &PathBuf) -> Self {
        let file = fs::File::open(&source).expect("Failed to open source file");

        Self {
            buf_reader: BufReader::new(file),
            line_number: 0,
        }
    }
}

impl Lexer for LexerImpl {
    fn next_line_token(&mut self) -> Option<Vec<Token>> {
        let mut line = String::new();
        match self.buf_reader.read_line(&mut line) {
            // reach EOF
            Ok(0) => None,
            Ok(_) => {
                let words: Vec<Token> = line
                    .trim()
                    .split_whitespace()
                    .map(String::from)
                    .map(Self::distinguish)
                    .collect();

                if words.is_empty() {
                    self.line_number = self.line_number + 1;
                    if self.line_number == 22 {
                        // test
                        println!("11");
                    }
                    self.next_line_token()
                } else {
                    if is_comment(&words[0].souce.as_str()) {
                        self.line_number = self.line_number + 1;
                        return self.next_line_token();
                    }
                    self.line_number = self.line_number + 1;
                    Some(words)
                }
            }

            // failed to read line
            Err(e) => {
                panic!(
                    "Failed to read line: {}, error: {}",
                    self.line_number,
                    e.to_string()
                );
            }
        }
    }

    fn get_current_line_number(&self) -> usize {
        self.line_number
    }
}

impl LexerImpl {
    fn distinguish(s: String) -> Token {
        let mut token_type = TokenType::Invalid;
        match s.as_str() {
            "(" => token_type = TokenType::LRrace,
            ")" => token_type = TokenType::RBrace,
            "[" => token_type = TokenType::LSBracket,
            "]" => token_type = TokenType::RSBracket,

            _ => {
                // first LEVEL
                if let Some(k) = keyword::is_keyword(&s) {
                    return Token::new(s, TokenType::Keyword(k));
                }

                // query can be variable, high level than variable
                if let Some(query) = query::is_query(&s) {
                    return Token::new(s, TokenType::Query(query));
                }

                if let Some(str) = literal::is_literal(&s) {
                    return Token::new(String::new(), TokenType::Float(str));
                }

                if let Some(_) = variable::is_variable(&s) {
                    return Token::new(s.chars().skip(1).collect(), TokenType::Variable);
                }

                if let Some(_) = procedure::is_procedure(&s) {
                    return Token::new(s, TokenType::Procedure);
                }
            }
        }

        // debug_assert!(token_type != TokenType::Invalid);
        Token::new(s, token_type)
    }
}
