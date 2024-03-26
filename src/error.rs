use miette::{miette, Severity};
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
        LogoError::TooManyArguments(_, _) => {
            let report = miette!(
                severity = Severity::Error,
                help = "reduct argument",
                "{}",
                error,
            );
            eprintln!("failed to build since: {:?}", report);
        }
        LogoError::UnExpectedToken(_) => {
            let report = miette!(
                severity = Severity::Error,
                help = "please check the token",
                "{}",
                error,
            );
            eprintln!("failed to build since: {:?}", report);
        }
        LogoError::UnDefinedVariable(_) => {
            let report = miette!(
                severity = Severity::Error,
                help = "define variable first",
                "{}",
                error,
            );
            eprintln!("failed to build since: {:?}", report);
        }
        LogoError::UnvalidIfOrWhile(_, _) => {
            let report = miette!(
                severity = Severity::Error,
                help = "check if while style",
                "{}",
                error,
            );
            eprintln!("failed to build since: {:?}", report);
        }
        LogoError::FunctionDefineFailed(_, _, _) => {
            let report = miette!(
                severity = Severity::Error,
                help = "define function correctly",
                "{}",
                error,
            );
            eprintln!("failed to build since: {:?}", report);
        }
        LogoError::NoEnoughArguments(_) => {
            let report = miette!(
                severity = Severity::Error,
                help = "check argument number",
                "{}",
                error,
            );
            eprintln!("failed to build since: {:?}", report);
        }
        LogoError::NotAexpression(_, _) => {
            let report = miette!(
                severity = Severity::Error,
                help = "please give a expression calculable",
                "{}",
                error,
            );
            eprintln!("failed to build since: {:?}", report);
        }
    }
    std::process::exit(1);
}
