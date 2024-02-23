use crate::parser::{ASTNode, FunctionDeclaration, Statement, Expression};

pub struct CodeGen;

impl CodeGen {
    pub fn generate(ast: &ASTNode) -> String {
        match ast {
            ASTNode::Program(functions) => {
                let mut asm = String::new();
                for function in functions {
                    asm.push_str(&Self::generate_function(function));
                }
                asm
            },
            _ => panic!("Unsupported AST node at the program root."),
        }
    }

    fn generate_function(function: &FunctionDeclaration) -> String {
        let mut asm = format!("global {}\n{}:\n", function.name, function.name);
        for statement in &function.body {
            asm.push_str(&Self::generate_statement(statement));
        }
        asm
    }

    fn generate_statement(statement: &Statement) -> String {
        match statement {
            Statement::Return(expr) => Self::generate_expression(expr) + "\tret\n",
            _ => panic!("Unsupported statement."),
        }
    }

    fn generate_expression(expression: &Expression) -> String {
        match expression {
            Expression::IntegerLiteral(value) => format!("\tmov eax, {}\n", value),
            _ => panic!("Unsupported expression."),
        }
    }
}
