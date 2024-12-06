#[derive(Debug, Default)]
pub enum Node {
    #[default]
    Unkown,
    Break,
    Function {
        string: String
    }
}
