#[derive(Debug)]
pub enum FunName {
    // fn pen_up(&mut self);
    PenUp,
    // fn pen_down(&mut self);
    PenDown,

    // fn foreward(&mut self, pixel: Pixel);
    Foreward,
    // fn back(&mut self, pixel: Pixel);
    Back,
    // fn left(&mut self, pixel: Pixel);
    Left,
    // fn right(&mut self, pixel: Pixel);
    Right,

    // fn set_color(&mut self, color: Color);
    SetColor,

    // fn turn(&mut self, degree: Degree);
    Turn,
    // fn set_heading(&mut self, degree: Degree);
    SetHeading,

    // fn set_x_coordinate(&mut self, pixel: Pixel);
    SetXCoordinate,
    // fn set_y_coordinate(&mut self, pixel: Pixel);
    SetYCoordinate,
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

// pub fn mangling_fn_name_without_argument(fun_name: &str) -> String {
//     let mut ret = String::from("_ZF");
//     ret.push_str("__");
//     ret.push_str(fun_name);
//     ret
// }

// pub fn mangling_fn_name(fun_name: &str, argument_size: &usize) -> String {
//     let mut ret = String::from("_ZF");
//     ret.push_str(fun_name);
//     ret.push_str("__");
//     ret.push_str(&argument_size.to_string());
//     ret
// }

// pub fn unmangling_fn_name(mangled_name: &str) -> Option<(String, usize)> {
//     // 检查名称是否以 "_ZF" 开头
//     if mangled_name.starts_with("_ZF") {
//         // 分割名称和参数数量部分
//         let parts: Vec<&str> = mangled_name.split("__").collect();
//         if parts.len() == 3 {
//             // 提取函数名称和参数数量
//             let fun_name = parts[1].to_string();
//             if let Ok(argument_size) = parts[2].parse::<usize>() {
//                 return Some((fun_name, argument_size));
//             }
//         }
//     }
//     None
// }

// pub fn mangling_fn_var_name(fun_name: &str, var: &str) -> String {
//     let mut ret = String::from(fun_name);
//     ret.push_str("__");
//     ret.push_str(var);
//     ret
// }

#[derive(Debug)]
pub enum ASTNode {
    Sequence(Vec<ASTNode>),

    // function name, argument
    FunctionCall(FunName, Option<Vec<String>>),

    PlusAnd(String, String),
    Define(String, String),
    // literal queries
    Expersion(String),
    If(Box<ASTNode>, Vec<ASTNode>),
    While(Box<ASTNode>, Vec<ASTNode>),

    // function name
    // expressions for arguments
    CustomFunction(String, Option<String>),
}
