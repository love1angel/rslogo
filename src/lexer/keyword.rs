#[derive(Debug, PartialEq)]
pub enum Keyword {
    TRUE,
    FALSE,

    PENUP,
    PENDOWN,
    FORWARD,
    BACK,
    LEFT,
    RIGHT,
    SETPENCOLOR,
    TURN,
    SETHEADING,
    SETX,
    SETY,

    MAKE,
    ADDASSIGN,

    IF,
    WHILE,

    EQ,
    NE,
    GT,
    LT,
    AND,
    OR,
    Plus,
    Minus,
    Multipliy,
    Divide,

    FBegin,
    FEnd,
}

pub fn is_keyword(s: &str) -> Option<Keyword> {
    match s {
        "TRUE" => Some(Keyword::TRUE),
        "FALSE" => Some(Keyword::FALSE),

        "PENUP" => Some(Keyword::PENUP),
        "PENDOWN" => Some(Keyword::PENDOWN),
        "FORWARD" => Some(Keyword::FORWARD),
        "BACK" => Some(Keyword::BACK),
        "LEFT" => Some(Keyword::LEFT),
        "RIGHT" => Some(Keyword::RIGHT),
        "SETPENCOLOR" => Some(Keyword::SETPENCOLOR),
        "TURN" => Some(Keyword::TURN),
        "SETHEADING" => Some(Keyword::SETHEADING),
        "SETX" => Some(Keyword::SETX),
        "SETY" => Some(Keyword::SETY),

        "MAKE" => Some(Keyword::MAKE),
        "ADDASSIGN" => Some(Keyword::ADDASSIGN),

        "IF" => Some(Keyword::IF),
        "WHILE" => Some(Keyword::WHILE),

        "EQ" => Some(Keyword::EQ),
        "NE" => Some(Keyword::NE),
        "GT" => Some(Keyword::GT),
        "LT" => Some(Keyword::LT),
        "AND" => Some(Keyword::AND),
        "OR" => Some(Keyword::OR),
        "+" => Some(Keyword::Plus),
        "-" => Some(Keyword::Minus),
        "*" => Some(Keyword::Multipliy),
        "/" => Some(Keyword::Divide),

        "TO" => Some(Keyword::FBegin),
        "END" => Some(Keyword::FEnd),

        _ => None,
    }
}
