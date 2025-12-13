use zpl_tokenizer::{CommandPrefix, Span, Token, Tokenizer};

use crate::{
    command::{Command, Justification},
    error::ParseError,
};

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    peeked: Option<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(zpl: &'a str) -> Self {
        Self {
            tokenizer: Tokenizer::new(zpl),
            peeked: None,
        }
    }

    pub fn parse(mut self) -> Result<Vec<Command>, ParseError> {
        let mut commands = Vec::new();

        while let Some(command) = self.parse_next()? {
            commands.push(command);
        }

        Ok(commands)
    }

    #[inline]
    fn next_token(&mut self) -> Option<Token<'a>> {
        if let Some(token) = self.peeked.take() {
            Some(token)
        } else {
            self.tokenizer.next()
        }
    }

    #[inline]
    fn peek_token(&mut self) -> Option<&Token<'a>> {
        if self.peeked.is_none() {
            self.peeked = self.tokenizer.next();
        }

        self.peeked.as_ref()
    }

    pub fn parse_next(&mut self) -> Result<Option<Command>, ParseError> {
        let token = match self.next_token() {
            Some(t) => t,
            None => return Ok(None),
        };

        let command = match token {
            Token::StartToken => Command::StartFormat,
            Token::EndToken => Command::EndFormat,
            Token::FieldSeparator => Command::FieldSeperator,
            Token::FieldData { data, .. } => Command::FielData {
                data: data.to_string(),
            },

            Token::Command {
                prefix,
                name,
                params,
                span,
            } => self.parse_command(prefix, name, &params, span)?,
        };

        Ok(())
    }

    fn parse_command(
        &self,
        prefix: CommandPrefix,
        name: &str,
        params: &[&str],
        span: Span,
    ) -> Result<Command, ParseError> {
        let name_upper = name.to_uppercase();

        match (prefix, name_upper.as_str()) {
            (CommandPrefix::Caret, "FO") => self.parse_field_origin(params, span),
            (CommandPrefix::Caret, "FT") => self.parse_field_typeset(params, span),
            (CommandPrefix::Caret, "LH") => self.parse_label_home(params, span),

            (CommandPrefix::Caret, "A") | (CommandPrefix::Caret, name) if name.starts_with("A") => {
                self.parse_font(name, params, span);
            }
        }
    }

    fn parse_field_origin(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^FOx,y[,justification]

        let x = self.parse_u32_param(params, 0, "x", span)?;
        let y = self.parse_u32_param(params, 0, "y", span)?;

        let justification = if params.len() >= 3 {
            Some(self.parse_justification(params[2], span)?)
        } else {
            None
        };

        Ok(Command::FieldOrigin {
            x,
            y,
            justification,
        })
    }

    fn parse_field_typeset(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^FTx,y

        let x = self.parse_u32_param(params, 0, "x", span)?;
        let y = self.parse_u32_param(params, 0, "y", span)?;

        let justification = if params.len() >= 3 {
            Some(self.parse_justification(params[2], span)?)
        } else {
            None
        };

        Ok(Command::FieldTypeset {
            x,
            y,
            justification,
        })
    }

    fn parse_label_home(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^LHx,y

        let x = self.parse_u32_param(params, 0, "x", span)?;
        let y = self.parse_u32_param(params, 0, "y", span)?;

        Ok(Command::LabelHome { x, y })
    }

    fn parse_justification(&self, s: &str, _span: Span) -> Result<Justification, ParseError> {
        match s.to_uppercase().as_str() {
            "0" | "L" => Ok(Justification::Left),
            "1" | "R" => Ok(Justification::Right),
            "2" | "A" => Ok(Justification::Auto),
            _ => Ok(Justification::Left),
        }
    }

    fn parse_u32_param(
        &self,
        params: &[&str],
        index: usize,
        name: &'static str,
        span: Span,
    ) -> Result<u32, ParseError> {
        if index >= params.len() {
            return Err(ParseError::MissingRequiredParameter {
                command: "",
                parameter: name,
                span,
            });
        }

        self.parse_u32(params[index], span)
    }

    fn parse_optional_u32(
        &self,
        params: &[&str],
        index: usize,
        span: Span,
    ) -> Result<Option<u32>, ParseError> {
        if index >= params.len() || params[index].is_empty() {
            return Ok(None);
        }

        Ok(Some(self.parse_u32(params[index], span)?))
    }

    #[inline]
    fn parse_u32(&self, s: &str, span: Span) -> Result<u32, ParseError> {
        s.parse::<u32>().map_err(|_| ParseError::InvalidNumber {
            value: s.to_string(),
            span,
        })
    }
}
