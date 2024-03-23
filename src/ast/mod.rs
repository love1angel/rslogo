#[derive(Debug)]
pub enum FunName {
    // fn pen_up(&mut self);
    pen_up,
    // fn pen_down(&mut self);
    pen_down,

    // fn foreward(&mut self, pixel: Pixel);
    foreward,
    // fn back(&mut self, pixel: Pixel);
    back,
    // fn left(&mut self, pixel: Pixel);
    left,
    // fn right(&mut self, pixel: Pixel);
    right,

    // fn set_color(&mut self, color: Color);
    set_color,

    // fn turn(&mut self, degree: Degree);
    turn,
    // fn set_heading(&mut self, degree: Degree);
    set_heading,

    // fn set_x_coordinate(&mut self, pixel: Pixel);
    set_x_coordinate,
    // fn set_y_coordinate(&mut self, pixel: Pixel);
    set_y_coordinate,
}

// in expression
// pub enum Query {
//     // fn get_x_coordinate(&self) -> Pixel;
//     get_x_coordinate,
//     // fn get_y_coordinate(&self) -> Pixel;
//     get_y_coordinate,

//     // fn get_heading(&self) -> Direction;
//     get_heading,
//     // fn get_color(&self) -> Color;
//     get_color,
// }

#[derive(Debug)]
pub enum ASTNode {
    Sequence(Vec<ASTNode>),

    // function name, argument
    FunctionCall(FunName, Option<Vec<String>>),

    PlusAnd(String, String),
    Define(String, String),
    // literal queries
    Expersion(Vec<ASTNode>),
    If(Box<ASTNode>, Vec<ASTNode>),
    While(Box<ASTNode>, Box<ASTNode>),
}
