use regex::Regex;

fn main() {
    let expr = "\"hello";
    if let Some(()) = is_variable(&expr) {
        println!("111");
    }
}

pub fn is_variable(s: &str) -> Option<()> {
    let re = Regex::new(r#"^"[a-zA-Z_][a-zA-Z0-9_]*"$"#).unwrap();

    if let Some(_) = keyword::is_keyword(s) {
        return None;
    }
    if re.is_match(s) {
        return Some(());
    }

    None
}
