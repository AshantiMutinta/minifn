use crate::{
    common::error::{MiniFnError, ParserError},
    compiler::token::{LineOfCode, Token, TokenType},
};

//Syntax rules for minifn.
// We will implement a right to left(Manga) compiler they are rules.
#[derive(Debug)]
pub enum ParseCommand<'a> {
    Assign(&'a Token<'a>, &'a Token<'a>),
}

fn parse_line_of_code<'a>(line_of_code: &'a LineOfCode) -> Result<ParseCommand<'a>, MiniFnError> {
    let line_of_code = line_of_code.0.iter();
    line_of_code
        .clone()
        .last()
        .map(|token| match token.token_type {
            TokenType::IntDigit => {
                let equals_sign =
                    line_of_code
                        .clone()
                        .last()
                        .ok_or(MiniFnError::new_parsing_error(
                            ParserError::UnexpectedSymbol,
                        ))?;
                if equals_sign.symbol != "=" {
                    return Err(MiniFnError::new_parsing_error(
                        ParserError::UnexpectedSymbol,
                    ));
                }
                let variable = line_of_code.last().ok_or(MiniFnError::new_parsing_error(
                    ParserError::UnexpectedSymbol,
                ))?;
                match variable.token_type {
                    TokenType::Text => (),
                    _ => {
                        return Err(MiniFnError::new_parsing_error(
                            ParserError::UnexpectedSymbol,
                        ))
                    }
                };

                Ok(ParseCommand::Assign(&variable, token))
            }
            _ => Err(MiniFnError::new_parsing_error(
                ParserError::UnexpectedSymbol,
            )),
        })
        .unwrap_or(Err(MiniFnError::new_parsing_error(
            ParserError::UnexpectedSymbol,
        )))
}

pub fn parse_lines_of_code<'a>(
    lines: &'a Vec<LineOfCode>,
) -> Result<Vec<ParseCommand<'a>>, Vec<MiniFnError>> {
    let (parsed_lines, errors): (Vec<_>, Vec<_>) = lines
        .iter()
        .map(|s| parse_line_of_code(s))
        .partition(Result::is_ok);
    if !errors.is_empty() {
        Err(errors
            .into_iter()
            .map(|e| e.unwrap_err())
            .collect::<Vec<_>>())
    } else {
        Ok(parsed_lines
            .into_iter()
            .map(|e| e.unwrap())
            .collect::<Vec<_>>())
    }
}
