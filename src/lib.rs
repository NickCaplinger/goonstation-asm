use logos::Logos;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum AssemblerError {
    #[error("Expected operand")]
    ExpectedOperand,
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

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

pub fn assemble(input: &str) -> Result<String, AssemblerError> {
    let lexer = Token::lexer(input);

    let mut output = String::new();

    let mut expecting_operand = false;
    for token in lexer {
        dbg!(&token);

        // If we're expecting an operand, make sure this token is one
        if expecting_operand {
            match token {
                Token::Operand(_) => {
                    expecting_operand = false;
                }
                _ => {
                    // TODO
                    return Err(AssemblerError::ExpectedOperand);
                }
            }
        }

        // Push the token representation to the output
        if let Some(token_repr) = get_token_representation(&token) {
            output.push(token_repr);
        }

        match token {
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
            | Token::Jump => expecting_operand = true,
            _ => expecting_operand = false,
        }
    }

    if expecting_operand {
        // TODO
        return Err(AssemblerError::ExpectedOperand);
    }

    Ok(output)
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
        Token::Operand(operand) => {
            Some(format!("{:X}", operand).chars().next().unwrap())
        },
        Token::Error => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let bin = assemble("OEN 0 \nSTO 0");
        assert_eq!(bin, Ok(String::from("B080")));
    }
}
