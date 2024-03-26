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

    #[error("function define error: line number: {0}, command: {1}, reason: {2}")]
    FunctionDefineFailed(usize, String, String),

    #[error("NoEnoughArguments for command: {0}")]
    NoEnoughArguments(String),

    #[error("Command: {0} need a expression, but current not a expression for string: {1}")]
    NotAexpression(String, String),
}

pub fn fatal_error(error: LogoError) {
    match error {
        LogoError::TooManyArguments(_, _) => {}
        LogoError::UnExpectedToken(_) => todo!(),
        LogoError::UnDefinedVariable(_) => todo!(),
        LogoError::UnvalidIfOrWhile(_, _) => todo!(),
        LogoError::FunctionDefineFailed(_, _, _) => todo!(),
        LogoError::NoEnoughArguments(_) => todo!(),
        LogoError::NotAexpression(_, _) => todo!(),
    }
    eprintln!("failed to build since: {:?}", error);
    std::process::exit(1);
}
