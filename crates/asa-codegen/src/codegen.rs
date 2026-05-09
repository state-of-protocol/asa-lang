// PELAKSANAAN SEMENTARA: Pengkompil ini ditulis dalam Rust.
// Ia akan digantikan dengan pengkompil ASA yang ditulis dalam ASA
// sebaik sahaja bahasa ini matang. Lihat SUPREMACY.txt

use asa_ast::*;
use std::io::Write;

pub fn generate(program: &Program, output: &mut impl Write) -> std::io::Result<()> {
    writeln!(output, "#include <stdio.h>")?;
    writeln!(output, "#include <stdlib.h>")?;
    writeln!(output, "#include <string.h>")?;
    writeln!(output)?;
    for func in &program.functions {
        gen_function(func, output)?;
    }
    Ok(())
}

fn gen_function(func: &Function, output: &mut impl Write) -> std::io::Result<()> {
    write!(output, "void {}(", func.name)?;
    for (i, (param, _)) in func.params.iter().enumerate() {
        if i > 0 { write!(output, ", ")?; }
        write!(output, "int {}", param)?;
    }
    writeln!(output, ") {{")?;
    for stmt in &func.body {
        gen_statement(stmt, output, 1)?;
    }
    writeln!(output, "}}")?;
    Ok(())
}

fn gen_statement(stmt: &Statement, output: &mut impl Write, indent: usize) -> std::io::Result<()> {
    let i = "    ".repeat(indent);
    match stmt {
        Statement::Let { name, init } => {
            write!(output, "{}int {} = ", i, name)?;
            gen_expr(init, output)?;
            writeln!(output, ";")?;
        }
        Statement::Assign { name, value } => {
            write!(output, "{}{} = ", i, name)?;
            gen_expr(value, output)?;
            writeln!(output, ";")?;
        }
        Statement::Expression(expr) => {
            write!(output, "{}", i)?;
            gen_expr(expr, output)?;
            writeln!(output, ";")?;
        }
        Statement::If { condition, then_branch, else_branch } => {
            write!(output, "{}if (", i)?;
            gen_expr(condition, output)?;
            writeln!(output, ") {{")?;
            for s in then_branch {
                gen_statement(s, output, indent+1)?;
            }
            if let Some(else_block) = else_branch {
                writeln!(output, "{}}} else {{", i)?;
                for s in else_block {
                    gen_statement(s, output, indent+1)?;
                }
            }
            writeln!(output, "{}}}", i)?;
        }
        Statement::For { var, start, end, body } => {
            write!(output, "{}for (int {} = ", i, var)?;
            gen_expr(start, output)?;
            write!(output, "; {} < ", var)?;
            gen_expr(end, output)?;
            writeln!(output, "; {}++) {{", var)?;
            for s in body {
                gen_statement(s, output, indent+1)?;
            }
            writeln!(output, "{}}}", i)?;
        }
        Statement::While { condition, body } => {
            write!(output, "{}while (", i)?;
            gen_expr(condition, output)?;
            writeln!(output, ") {{")?;
            for s in body {
                gen_statement(s, output, indent+1)?;
            }
            writeln!(output, "{}}}", i)?;
        }
        Statement::Return(expr) => {
            write!(output, "{}return", i)?;
            if let Some(e) = expr {
                write!(output, " ")?;
                gen_expr(e, output)?;
            }
            writeln!(output, ";")?;
        }
    }
    Ok(())
}

fn gen_expr(expr: &Expression, output: &mut impl Write) -> std::io::Result<()> {
    match expr {
        Expression::Ident(name) => write!(output, "{}", name),
        Expression::IntLiteral(val) => write!(output, "{}", val),
        Expression::StringLiteral(s) => write!(output, "\"{}\"", s),
        Expression::Binary { left, op, right } => {
            gen_expr(left, output)?;
            let op_str = match op {
                BinaryOp::Add => " + ",
                BinaryOp::Sub => " - ",
                BinaryOp::Mul => " * ",
                BinaryOp::Div => " / ",
                BinaryOp::Eq => " == ",
                BinaryOp::Neq => " != ",
                BinaryOp::Lt => " < ",
                BinaryOp::Gt => " > ",
                BinaryOp::Le => " <= ",
                BinaryOp::Ge => " >= ",
            };
            write!(output, "{}", op_str)?;
            gen_expr(right, output)
        }
        Expression::Unary { op, expr } => {
            match op {
                UnaryOp::Neg => write!(output, "-")?,
            }
            gen_expr(expr, output)
        }
        Expression::Call { func, args } => {
            if func == "println" {
                write!(output, "printf(")?;
                let mut format_parts: Vec<String> = Vec::new();
                let mut arg_exprs: Vec<&Expression> = Vec::new();
                for arg in args {
                    match arg {
                        Expression::StringLiteral(s) => {
                            format_parts.push(s.clone());
                        }
                        _ => {
                            format_parts.push("%d".to_string());
                            arg_exprs.push(arg);
                        }
                    }
                }
                let format_str = format!("\"{}\\n\"", format_parts.join(""));
                write!(output, "{}", format_str)?;
                for expr in arg_exprs {
                    write!(output, ", ")?;
                    gen_expr(expr, output)?;
                }
                write!(output, ")")?;
            } else {
                write!(output, "{}(", func)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { write!(output, ", ")?; }
                    gen_expr(arg, output)?;
                }
                write!(output, ")")?;
            }
            Ok(())
        }
    }
}
