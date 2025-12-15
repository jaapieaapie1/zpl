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

    InvalidJustification {
        value: String,
        span: Span,
    },
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
            Self::InvalidJustification { value, span } => {
                write!(
                    f,
                    "Invalid justification received, gotten {} at position {}-{}",
                    value, span.start, span.end
                )
            }
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Error for ParseError {}
