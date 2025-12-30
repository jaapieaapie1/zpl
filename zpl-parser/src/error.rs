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

    InvalidYesNo {
        value: String,
        span: Span,
    },

    InvalidBlockJustification {
        value: String,
        span: Span,
    },

    InvalidLineColor {
        value: String,
        span: Span,
    },

    InvalidCompressionType {
        value: String,
        span: Span,
    },

    InvalidCode128Mode {
        value: String,
        span: Span,
    },

    InvalidZ64Data {
        message: String,
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
