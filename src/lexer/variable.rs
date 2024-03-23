use regex::Regex;

use crate::lexer::keyword;

// start with " , define a variable or a argument
pub fn is_variable(s: &str) -> Option<()> {
    let re = Regex::new(r#"\"[a-zA-Z_][a-zA-Z0-9_]*[^"]*"#).unwrap();

    if let Some(_) = keyword::is_keyword(s) {
        return None;
    }
    if re.is_match(s) {
        return Some(());
    }

    None
}
