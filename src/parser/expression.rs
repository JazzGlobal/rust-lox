use crate::scanner::token::Token;


#[derive(Debug)]
pub enum Expr {
    BINARY_EXPR(BinaryExpr),
    UNARY_EXPR(UnaryExpr),
    LITERAL_EXPR(LiteralExpr),
    GROUP_EXPR(GroupingExpr),
}

// pub fn handle_expr(expr: Expr) -> String {
//     match expr {
//         Expr::BINARY_EXPR(binary_expr) => parenthesize(
//             binary_expr.operator.lexeme,
//             vec![binary_expr.left, binary_expr.right],
//         ),
//         Expr::UNARY_EXPR(unary_expr) => {
//             parenthesize(unary_expr.operator.lexeme, vec![unary_expr.right])
//         }
//         Expr::LITERAL_EXPR(literal_expr) => {
//             return match literal_expr.value {
//                 LoxType::LoxString(opt) => match opt {
//                     None => "nil".to_string(),
//                     Some(value) => value,
//                 },
//                 LoxType::LoxBoolean(opt) => match opt {
//                     None => "nil".to_string(),
//                     Some(value) => value.to_string(),
//                 },
//                 LoxType::LoxNumber(opt) => match opt {
//                     Some(value) => value.to_string(),
//                     None => "nil".to_string(),
//                 },
//                 LoxType::LoxNil(opt) => match opt {
//                     _Some => "nil".to_string(),
//                     None => "nil".to_string(),
//                 },
//             }
//         }
//         Expr::GROUP_EXPR(group_expr) => {
//             parenthesize("group".to_string(), vec![group_expr.expression])
//         }
//     }
// }

// fn parenthesize(name: String, exprs: Vec<Box<Expr>>) -> String {
//     print!("({}", name);
//     for expr in exprs {
//         let x = *expr;
//         print!(" ");
//         print!("{}", &handle_expr(x));
//     }
//     print!(")");
//     "".to_string()
// }

#[derive(Debug)]
pub enum LoxType {
    LoxString(Option<String>),
    LoxBoolean(Option<bool>),
    LoxNumber(Option<f64>),
    LoxNil,
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub(crate) left: Box<Expr>,
    pub(crate) operator: Token,
    pub(crate) right: Box<Expr>,
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub(crate) operator: Token,
    pub(crate) right: Box<Expr>,
}

#[derive(Debug)]
pub struct LiteralExpr {
    pub(crate) value: LoxType,
}

#[derive(Debug)]
pub struct GroupingExpr {
    pub(crate) expression: Box<Expr>,
}
