use std::fmt::{self, Display};

use crate::helper::DynError;

mod codegen;
mod evaluator;
mod parser;

#[derive(Debug)]
pub enum Instruction {
    Char(char),
    Match,
    Jump(usize),
    Split(usize, usize),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Char(c) => write!(f, "char {c}"),
            Instruction::Match => write!(f, "match"),
            Instruction::Jump(addr) => write!(f, "jump {:>04}", addr),
            Instruction::Split(addr1, addr2) => write!(f, "split {:>04}, {:>04}", addr1, addr2),
        }
    }
}

/// Test string by the provided regex.
/// Returns Ok(true) when the regex matches the string.
/// Returns Ok(false) when the regex does not match the string.
///
/// ## Example
/// ```
/// use regex;
/// regex::do_matching("*abc|(de|cd)+", "decddede", true);
/// ```
pub fn do_matching(expr: &str, line: &str) -> Result<bool, DynError> {
    let ast = parser::parse(expr)?;
    let code = codegen::get_code(&ast)?;
    let line = line.chars().collect::<Vec<char>>();
    Ok(evaluator::eval(&code, &line)?)
}
