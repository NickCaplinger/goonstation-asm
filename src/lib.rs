use logos::Logos;
use thiserror::Error;

// See Goonstation source code for more details: https://github.com/goonstation/goonstation/blob/master/code/modules/mechanics/MechanicMC14500.dm

const MAX_PROGRAM_LENGTH: usize = 128;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum AssemblerError {
    #[error("Expected operand")]
    ExpectedOperand,
    #[error("Exceeded max program length")]
    ExceededMaxLength,
}

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[token("NOP")]
    NoOp,

    #[token("LD")]
    Load,

    #[token("LDC")]
    LoadComplement,

    #[token("AND")]
    And,

    #[token("ANDC")]
    AndComplement,

    #[token("OR")]
    Or,

    #[token("ORC")]
    OrComplement,

    #[token("XNOR")]
    ExclusiveNor,

    #[token("STO")]
    Store,

    #[token("STOC")]
    StoreComplement,

    #[token("IEN")]
    InputEnable,

    #[token("OEN")]
    OutputEnable,

    #[token("JMP")]
    Jump,

    #[token("RTN")]
    Return,

    #[token("SKZ")]
    SkipIfZero,

    #[regex(r"[a-fA-F0-9]", |lex| u8::from_str_radix(lex.slice(), 16))]
    Operand(u8),

    #[regex(r";.*", logos::skip)]
    Comment,

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

pub struct Program {
    tokens: Vec<Token>,
}

impl Program {
    pub fn from_assembly(assembly: &str) -> Self {
        let lexer = Token::lexer(assembly);
        let tokens = lexer.collect();

        Self { tokens }
    }

    pub fn into_opcodes(&self) -> Result<String, AssemblerError> {

        if self.tokens.len() > MAX_PROGRAM_LENGTH {
            return Err(AssemblerError::ExceededMaxLength);
        }

        let mut output = String::new();

        let mut expecting_operand = false;
        for token in &self.tokens {
            dbg!(&token);

            // If we're expecting an operand, make sure this token is one
            if expecting_operand {
                match token {
                    Token::Operand(_) => {}
                    _ => {
                        return Err(AssemblerError::ExpectedOperand);
                    }
                }
            }

            // Push the token representation to the output
            if let Some(token_repr) = get_token_representation(token) {
                output.push(token_repr);
            }

            // Flag whether we're expecting an operand as the next token
            expecting_operand = does_token_require_operand(token);
        }

        if expecting_operand {
            return Err(AssemblerError::ExpectedOperand);
        }

        Ok(output)
    }
}

fn get_token_representation(token: &Token) -> Option<char> {
    match token {
        Token::NoOp => Some('0'),
        Token::Load => Some('1'),
        Token::LoadComplement => Some('2'),
        Token::And => Some('3'),
        Token::AndComplement => Some('4'),
        Token::Or => Some('5'),
        Token::OrComplement => Some('6'),
        Token::ExclusiveNor => Some('7'),
        Token::Store => Some('8'),
        Token::StoreComplement => Some('9'),
        Token::InputEnable => Some('A'),
        Token::OutputEnable => Some('B'),
        Token::Jump => Some('C'),
        Token::Return => Some('D'),
        Token::SkipIfZero => Some('E'),
        Token::Operand(operand) => Some(format!("{:X}", operand).chars().next().unwrap()),
        Token::Comment | Token::Error => None,
    }
}

fn does_token_require_operand(token: &Token) -> bool {
    matches!(
        token,
        Token::Load
            | Token::LoadComplement
            | Token::And
            | Token::AndComplement
            | Token::Or
            | Token::OrComplement
            | Token::ExclusiveNor
            | Token::Store
            | Token::StoreComplement
            | Token::InputEnable
            | Token::OutputEnable
            | Token::Jump
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_simple_program() {
        let program = Program::from_assembly("OEN 0 \nSTO 0");
        let bin = program.into_opcodes();
        assert_eq!(bin, Ok(String::from("B080")));
    }

    #[test]
    fn handles_non_zero_operands() {
        let program = Program::from_assembly("OEN 0\nSTO 0\nLD 7\nSTO F");
        let bin = program.into_opcodes();
        assert_eq!(bin, Ok(String::from("B080178F")));
    }

    #[test]
    fn handles_missing_final_operand() {
        let program = Program::from_assembly("OEN 0\nSTO 0\nLD 7\nSTO");
        let bin = program.into_opcodes();
        assert_eq!(bin, Err(AssemblerError::ExpectedOperand));
    }

    #[test]
    fn handles_missing_middle_operand() {
        let program = Program::from_assembly("OEN 0\nSTO \nLD 7\nSTO F");
        let bin = program.into_opcodes();
        assert_eq!(bin, Err(AssemblerError::ExpectedOperand));
    }

    #[test]
    fn handles_comments() {
        let program = Program::from_assembly("OEN 0 ;enable the output because RR is zero, so input 1 (!RR) is 1\nSTO 0 ;store the 0 from RR in output 1, so the unit outputs the signal \"0:0\"");
        let bin = program.into_opcodes();
        assert_eq!(bin, Ok(String::from("B080")));
    }
}
