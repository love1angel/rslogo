// just save as f32 string
pub fn is_literal(s: &str) -> Option<String> {
    if s.len() <= 1 {
        return None;
    }
    if !s.starts_with('"') {
        return None;
    }

    let s = &s[1..];
    match s {
        "TRUE" => return Some(1.0.to_string()),
        "FALSE" => return Some(0.0.to_string()),
        _ => {
            if let Ok(val) = s.parse::<f32>() {
                return Some(val.to_string());
            }
        }
    }
    None
}
