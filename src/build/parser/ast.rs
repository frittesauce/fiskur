use std::{ops::Range, usize};

#[derive(Debug)]
pub struct Parameter {
    pub location: Location,
    pub mutable: bool,
    pub name: String,
    pub data_type: String,
}

#[derive(Debug, Default)]
pub enum Node {
    #[default]
    Unkown,
    Break,
    Function {
        identifier: String,
        parameters: Vec<Parameter>,
        return_type: String,
        scope: Vec<NodeInfo>,
    },
    SetVariable {
        identifier: String,
        value: Option<ExprInfo>,
    },
    DeclVar {
        identifier: String,
        mutable: bool,
        data_type: String,
        expression: Option<ExprInfo>,
    },
    Loop {
        condition: ExprInfo,
        body: Vec<NodeInfo>,
    },
    If {
        expression: ExprInfo,
        body: Vec<NodeInfo>,
        elseif: Vec<(ExprInfo, Vec<NodeInfo>)>,
        else_body: Option<Vec<NodeInfo>>,
    },
}

#[derive(Debug)]
pub struct Location {
    lines: Range<usize>,
    columns: Range<usize>,
}

#[derive(Debug)]
pub struct NodeInfo {
    location: Location,
    node: Node,
}

#[derive(Debug)]
pub enum Expr {}

#[derive(Debug)]
pub struct ExprInfo {
    location: Location,
    expr: Expr,
}
