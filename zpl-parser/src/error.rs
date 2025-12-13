use std::{error::Error, fmt::Display};

use zpl_tokenizer::Span;

#[derive(Debug, Clone)]
pub enum ParseError {
    MissingRequiredParameter {
        command: &'static str,
        parameter: &'static str,
        span: Span,
    },

    InvalidNumber {
        value: String,
        span: Span,
    },

    InvalidOrientation,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingRequiredParameter {
                command,
                parameter,
                span,
            } => {
                write!(
                    f,
                    "Command ^{} missing required parameter '{}' at position {}-{}",
                    command, parameter, span.start, span.end
                )
            }
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Error for ParseError {}
