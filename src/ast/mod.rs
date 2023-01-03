mod node;

pub trait Node {
    fn token_literal(&self) -> String;

    fn string(&self) -> String;
}

pub trait Expression : Node {
    // fn expression_node(&self);
}

pub trait Statement : Node {
    // fn statement_node(&self);
}