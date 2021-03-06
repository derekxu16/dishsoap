use super::super::{Parser, Token};
use super::{Block, Node, Parsable};

#[derive(Debug, PartialEq, Eq)]
pub struct IfStatement {
    condition: Box<Node>,
    if_block: Box<Node>,
    else_block: Box<Option<Node>>,
}

impl IfStatement {
    pub fn new(condition: Node, if_block: Node, else_block: Option<Node>) -> Node {
        Node::IfStatement(IfStatement {
            condition: Box::new(condition),
            if_block: Box::new(if_block),
            else_block: Box::new(else_block),
        })
    }
}

impl Parsable for IfStatement {
    fn parse(parser: &mut Parser) -> Node {
        parser.lexer.consume(Token::IfKeyword);
        parser.lexer.consume(Token::ParenOpen);
        let condition = parser.parse_expression(0);
        if condition.is_none() {
            panic!("Compilation error");
        }
        parser.lexer.consume(Token::ParenClose);

        let if_block = Block::parse(parser);
        let else_block = match parser.lexer.peek() {
            Some(Token::ElseKeyword) => {
                parser.lexer.consume(Token::ElseKeyword);
                Some(Block::parse(parser))
            }
            _ => None,
        };

        IfStatement::new(condition.unwrap(), if_block, else_block)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct WhileStatement {}

#[derive(Debug, PartialEq, Eq)]
pub struct ForStatement {}

#[derive(Debug, PartialEq, Eq)]
pub struct ReturnStatement {
    pub expression: Box<Node>,
}

impl ReturnStatement {
    pub fn new(expression: Node) -> Node {
        Node::ReturnStatement(ReturnStatement {
            expression: Box::new(expression),
        })
    }
}

impl Parsable for ReturnStatement {
    fn parse(parser: &mut Parser) -> Node {
        parser.lexer.consume(Token::ReturnKeyword);

        let expression = match parser.parse_expression(0) {
            Some(e) => ReturnStatement::new(e),
            _ => {
                panic!("Compilation error");
            }
        };
        parser.lexer.consume(Token::Semicolon);

        expression
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct VariableDeclarationStatement {
    pub variable: Box<Node>,
}

impl VariableDeclarationStatement {
    pub fn new(variable: Node) -> Node {
        Node::VariableDeclarationStatement(VariableDeclarationStatement {
            variable: Box::new(variable),
        })
    }
}

impl Parsable for VariableDeclarationStatement {
    fn parse(parser: &mut Parser) -> Node {
        parser.lexer.consume(Token::LetKeyword);

        let variable_declaration =
            Node::VariableDeclarationStatement(VariableDeclarationStatement {
                variable: Box::new(parser.parse_variable_like().unwrap()),
            });
        parser.lexer.consume(Token::Semicolon);

        variable_declaration
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FunctionDeclarationStatement {
    pub identifier: Box<Node>,
    pub return_type: Box<Node>,
    pub parameters: Vec<Box<Node>>,
    pub body: Box<Node>,
}

impl FunctionDeclarationStatement {
    pub fn new(
        identifier: Node,
        return_type: Node,
        parameters: Vec<Box<Node>>,
        body: Node,
    ) -> Node {
        Node::FunctionDeclarationStatement(FunctionDeclarationStatement {
            identifier: Box::new(identifier),
            return_type: Box::new(return_type),
            parameters,
            body: Box::new(body),
        })
    }

    fn parse_parameters(parser: &mut Parser) -> Vec<Box<Node>> {
        let mut parameters: Vec<Box<Node>> = Vec::new();
        parser.lexer.consume(Token::ParenOpen);
        loop {
            if parser.lexer.peek() == Some(Token::ParenClose) {
                parser.lexer.consume(Token::ParenClose);
                break;
            }
            let parameter = parser.parse_variable_like();
            if parameter.is_none() {
                panic!("Compilation error");
            }
            parameters.push(Box::new(parameter.unwrap()));

            if parser.lexer.peek() == Some(Token::Comma) {
                parser.lexer.consume(Token::Comma);
            }
        }
        parameters
    }
}

impl Parsable for FunctionDeclarationStatement {
    fn parse(parser: &mut Parser) -> Node {
        parser.lexer.consume(Token::FuncKeyword);

        let identifier: Option<Node> = parser.parse_identifier();
        if identifier.is_none() {
            panic!("Compilation error");
        }

        let parameters = Self::parse_parameters(parser);

        parser.lexer.consume(Token::Colon);

        let return_type: Option<Node> = parser.parse_type();
        if return_type.is_none() {
            panic!("Compilation error: expected type.")
        }

        FunctionDeclarationStatement::new(
            identifier.unwrap(),
            return_type.unwrap(),
            parameters,
            Block::parse(parser),
        )
    }
}
