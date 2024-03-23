#[derive(Debug, PartialEq)]
pub enum Query {
    XCOR,
    YCOR,
    HEADING,
    COLOR,
}

pub fn is_query(s: &str) -> Option<Query> {
    match s {
        "XCOR" => Some(Query::XCOR),
        "YCOR" => Some(Query::YCOR),
        "HEADING" => Some(Query::HEADING),
        "COLOR" => Some(Query::COLOR),
        _ => None,
    }
}
