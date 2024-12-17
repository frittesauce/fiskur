use std::{ops::Range, usize};

use toml::Value;

#[derive(Debug, Default)]
pub struct Location {
    lines: Range<usize>,
    columns: Range<usize>,
}

#[derive(Debug, Default)]
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
    Enum {
        name: String,
        fields: Vec<String>
    },
    Struct {
        name: String,
        fields: Vec<(String, String)>
    },
    Function {
        identifier: String,
        parameters: Vec<Parameter>,
        return_type: String,
        scope: Vec<NodeInfo>,
    },
    Scope(Vec<NodeInfo>),

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
    IfStatement {
        expression: ExprInfo,
        body: Vec<NodeInfo>,
        elseif: Vec<(ExprInfo, Vec<NodeInfo>)>,
        else_body: Option<Vec<NodeInfo>>,
    },
    Call(ExprInfo),
    Return(Option<ExprInfo>),
    NameSpace {
        public: bool,
        static_path: String
    }
}
#[derive(Debug)]
pub struct NodeInfo {
    location: Location,
    node: Node,
}
impl NodeInfo {
    pub fn void() -> Self {
        Self {
            location: Location {
                columns: 0..0,
                lines: 0..0
            },
            node: Node::Unkown,
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    GetVariable(String),
    Field(Box<ExprInfo>, String),
    Index(String, Box<ExprInfo>),
    Value(Value),
    Call(Path, Vec<ExprInfo>)
}

#[derive(Debug)]
pub struct ExprInfo {
    location: Location,
    expr: Expr,
}
