use super::{Node, Statement, Expression};
use crate::{Token, KeyWords};


struct Program {
    statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].string()
        } else {
            "".to_string()
        }
    }

    fn string(&self) -> String {
        let mut out = String::new();
        for statement in &self.statements {
            out += statement.string().as_str();
        }
        out
    }
}

struct LetStatement {
    token: Token,
    name: Identifier,
    value: Box<dyn Expression>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out += self.token.literal.as_str();
        out += " ";
        out += self.name.string().as_str();
        out += " = ";
        out += self.value.string().as_str();
        out += ";";
        out
    }
}

impl Statement for LetStatement {}


struct ReturnStatement {
    token: Token,
    returntype: Option<Box<dyn Expression>>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    } 

    fn string(&self) -> String {
        let mut out = self.token_literal();
        out += " ";
        if self.returntype.is_some() {
            out += self.returntype.as_ref().unwrap().string().as_str();
        }
        out += ";";
        out        
    }
}

impl Statement for ReturnStatement {}

struct ExpressionStatement {
    token: Token,
    expression: Option<Box<dyn Expression>>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        if self.expression.is_some() {
            self.expression.as_ref().unwrap().string();
        }
        " ".to_string()
    }
}

impl Statement for ExpressionStatement {}

struct BlockStatement {
    token: Token,
    statements: Vec<Box<dyn Statement>>,
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();

        for statement in &self.statements {
            out += statement.string().as_str();
        }

        out
    }
}

impl Statement for BlockStatement {}

struct Identifier {
    token: Token,
    value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.value.clone()
    }
}

impl Expression for Identifier {}

struct Boolean {
    token: Token,
    value: bool,
}

impl Node for Boolean {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.value.to_string()
    }
}

impl Expression for Boolean {}


struct IntegerLiteral {
    token: Token,
    value: usize,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.value.to_string()
    }
}

impl Expression for IntegerLiteral {}

struct PrefixExpression {
    token: Token,
    operator: String,
    right: Box<dyn Expression>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out += "(";
        out += self.operator.as_str();
        out += self.right.string().as_str();
        out += ")";
        out   
    }
}

impl Expression for PrefixExpression {}

struct IfExpression {
    token: Token,
    condition: Box<dyn Expression>,
    consequence: BlockStatement,
    alternative: Option<BlockStatement>,
}

impl Node for IfExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out += "if";
        out += self.condition.string().as_str();
        out += " ";
        out += self.consequence.string().as_str();

        if self.alternative.is_some() {
            out +=  "else";
            out += self.alternative.as_ref().unwrap().string().as_str();
        }
        out
    }
}

impl Expression for IfExpression {}



struct FuntionLiteral {
    token: Token,
    parameters: Vec<Identifier>,
    body: BlockStatement,
}

impl Node for FuntionLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = self.token_literal();
        // let mut paras = String::new();
        // for para in &self.parameters {
        //     paras += para.string().as_str();
        // }
        out += "(";

        self.parameters.iter().for_each(|para| {
            out += para.string().as_str();
        });

        out += ")";
        out += self.body.string().as_str();
        out
    }
}

impl Expression for FuntionLiteral {}

struct CallExpression {
    token: Token,
    function: Box<dyn Expression>,
    arguments: Vec<Box<dyn Expression>>,
}

impl Node for CallExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();

        out += self.function.string().as_str();
        out += "(";
        self.arguments.iter().for_each(|arg| {
            let mut arg = arg.string();
            arg.push_str(", ");
            out += arg.as_str();
        });

        out += ")";

        out
    }
}

impl Expression for CallExpression {}

#[test]
fn ast_test() {
    let program = Program {
        statements: vec![
            Box::new(
                LetStatement {
                    token: Token::new(KeyWords::LET, "let".to_string()),
                    name: Identifier {
                        token: Token::new(KeyWords::IDENT, "five".to_string()),
                        value: "five".to_string(),
                    },
                    value: Box::new(
                        Identifier {
                            token: Token::new(KeyWords::IDENT, "ten".to_string()),
                            value: "ten".to_string(),
                        }
                    )
                }
            )
        ]
    };

    assert_eq!(program.string(), "let five = ten;".to_string());
}