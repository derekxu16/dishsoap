use super::super::{Parser, Token};
use super::{Node, Parsable};

#[derive(Debug, PartialEq, Eq)]
pub struct IfStatement {}

impl Parsable for IfStatement {
    fn parse(parser: &mut Parser) -> Node {
        parser.lexer.consume(Token::IfKeyword);
        parser.lexer.consume(Token::ParenOpen);
        Node::IfStatement(IfStatement {})
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct WhileStatement {}

#[derive(Debug, PartialEq, Eq)]
pub struct ForStatement {}

#[derive(Debug, PartialEq, Eq)]
pub struct ReturnStatement {}

#[derive(Debug, PartialEq, Eq)]
pub struct VariableDeclarationStatement {
    pub variable: Box<Node>,
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
}

impl FunctionDeclarationStatement {
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

        parser.lexer.consume(Token::BraceOpen);
        parser.lexer.consume(Token::BraceClose);

        Node::FunctionDeclarationStatement(FunctionDeclarationStatement {
            identifier: Box::new(identifier.unwrap()),
            return_type: Box::new(return_type.unwrap()),
            parameters,
        })
    }
}
