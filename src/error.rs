use thiserror::Error;

// 定义一个错误类型
#[derive(Error, Debug, Clone)]
pub enum LogoError {
    #[error("line number: {0}, command: {1}")]
    TooManyArguments(usize, String),

    #[error("line number: {0}, command: {1}")]
    NotAexpression(usize, String),

    #[error("Other error")]
    OtherError,
}

pub fn fatal_error(error: LogoError) {
    eprintln!("failed to build since: {}", error);
    std::process::exit(1);
}
