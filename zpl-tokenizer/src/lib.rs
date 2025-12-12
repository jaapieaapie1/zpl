#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandPrefix {
    Caret,
    Tilde,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token<'a> {
    StartToken,

    EndToken,

    Command {
        prefix: CommandPrefix,
        name: &'a str,
        params: Vec<&'a str>,
        span: Span,
    },

    FieldSeperator,

    FieldData {
        data: &'a str,
        span: Span,
    },

    Unknown {
        content: &'a str,
        span: Span,
    },
}

pub struct Tokenizer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, position: 0 }
    }

    #[inline]
    fn peek(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }
}
