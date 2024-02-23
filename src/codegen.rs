use log::info;
use crate::parser::{ASTNode, FunctionDeclaration, Statement, Expression};

pub struct CodeGen;

impl CodeGen {
	pub fn new() -> Self {
		CodeGen {}
	}

    pub fn generate(&self, ast: &ASTNode) -> String {
        match ast {
            ASTNode::Program(functions) => {
                let mut asm = String::new();
				// specifying the code section of assembly
				asm.push_str("\t.text\n");
				// directing assembler to use intel syntax x86
				asm.push_str("\t.intel_syntax noprefix\n");
                for function in functions {
                    asm.push_str(&self.generate_function(function));
                }
				info!("Generated assembly:\n{}", asm);
                asm
            },
        }
    }

    fn generate_function(&self, function: &FunctionDeclaration) -> String {
        let mut asm = format!(".globl {}\n{}:\n", function.name, function.name);
        for statement in &function.body {
            asm.push_str(&self.generate_statement(statement));
        }
		asm += "\tret\n";
        asm
    }

    fn generate_statement(&self, statement: &Statement) -> String {
        match statement {
            Statement::Return(expr) => self.generate_expression(expr),
        }
    }

    fn generate_expression(&self, expression: &Expression) -> String {
        match expression {
            Expression::IntegerLiteral(value) => format!("\tmov eax, {}\n", value),
        }
    }
}
