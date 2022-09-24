#[derive(Debug)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub enum Node {
    Num(i32),
    UnaryOp {
        op: Operator,
        operand: Box<Node>,
    },
    BinOp {
        op: Operator,
        left: Box<Node>,
        right: Box<Node>
    }
}