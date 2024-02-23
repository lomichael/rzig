use log::debug;
use crate::lexer::Token;

#[derive(Debug)]
pub enum ASTNode {
    Program(Vec<FunctionDeclaration>),
}

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration {
    pub name: String,
    pub return_type: String,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    IntegerLiteral(i32),
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    // get token
    fn peek(&self) -> &Token {
        &self.tokens[self.position]
    }

    // get token and advance position 
    fn next_token(&mut self) -> Option<Token> {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    // begin parsing from start rule
    pub fn parse_program(&mut self) -> ASTNode {
        let mut functions = Vec::new();
        while self.position < self.tokens.len() {
            if let Some(function_declaration) = self.parse_function_declaration() {
                functions.push(function_declaration);
            }
        }
		debug!("Parsed functions: {:?}", functions);
        ASTNode::Program(functions)
    }

    fn parse_function_declaration(&mut self) -> Option<FunctionDeclaration> {
        // function declaration starting tokens (e.g. 'export fn')
        match (self.next_token()?, self.next_token()?) {
            (Token::KEYWORDexport, Token::KEYWORDfn) => (),
            _ => return None,
        }

        // get the function identifier (e.g. 'main')
        let name = if let Token::IDENTIFIER(name) = self.next_token()? {
            name
        } else {
            return None;
        };

        // empty parameter list (e.g. '()')
        self.expect_token(Token::LPAREN)?;
        self.expect_token(Token::RPAREN)?;

        // get the return type (e.g. 'i32')
        let return_type = if let Token::KEYWORDreturntype(type_expr) = self.next_token()? {
            type_expr
        } else {
            return None;
        };

        // call the parse handler for the statement block (e.g. '{ return 42; }')
        self.expect_token(Token::LBRACKET)?;
        let body = self.parse_statement_block()?;
        self.expect_token(Token::RBRACKET)?;

        // return the ASTNode with the filled in info
        Some(FunctionDeclaration {
            name,
            return_type,
            body,
        })
    }

    fn parse_statement_block(&mut self) -> Option<Vec<Statement>> {
        let mut statements = Vec::new();

        // call the parse handler for the statement
        while let Some(statement) = self.parse_statement() {
            statements.push(statement);
            if matches!(self.peek(), Token::RBRACKET) {
                break;
            }
        }
        Some(statements)
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.next_token()? {
            Token::KEYWORDreturn => {
                let expression = self.parse_expression()?;
                self.expect_token(Token::SEMICOLON)?;
                Some(Statement::Return(expression))
            },
            _ => None,
        }
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        match self.next_token()? {
            Token::INTEGER(value) => Some(Expression::IntegerLiteral(value)),
            _ => None,
        }
    }

    fn expect_token(&mut self, expected: Token) -> Option<()> {
        if self.next_token()? == expected {
            Some(())
        } else {
            None
        }
    }
}


