pub fn is_keyword_operator(s: &str) -> bool {
    match s {
        "EQ" => true,
        "NE" => true,
        "GT" => true,
        "LT" => true,
        "AND" => true,
        "OR" => true,
        "+" => true,
        "-" => true,
        "*" => true,
        "/" => true,
        _ => false,
    }
}
