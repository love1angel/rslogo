use thiserror::Error;

// 定义一个错误类型
#[derive(Error, Debug, Clone)]
pub enum LogoError {
    #[error("TooManyArguments line number: {0}, command: {1}")]
    TooManyArguments(usize, String),

    #[error("unexpected token: {0}")]
    UnExpectedToken(String),

    #[error("UnDefined Variable: {0}")]
    UnDefinedVariable(String),

    #[error("UnvalidIfOrWhile: line number: {0}, reason: {1}")]
    UnvalidIfOrWhile(usize, String),

    #[error("function define error: line number: {0}, reason: {1}")]
    FunctionDefineFailed(usize, String),

    #[error("NoEnoughArguments for command: {0}")]
    NoEnoughArguments(String),

    #[error("Not a expression for string: {0}")]
    NotAexpression(String),
}

pub fn fatal_error(error: LogoError) {
    eprintln!("failed to build since: {}", error);
    std::process::exit(1);
}
