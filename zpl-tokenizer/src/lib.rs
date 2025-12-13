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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'a> {
    StartToken,

    EndToken,

    Command {
        prefix: CommandPrefix,
        name: &'a str,
        params: Vec<&'a str>,
        span: Span,
    },

    FieldSeparator,

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

    #[inline]
    fn peek_n(&self, n: usize) -> Option<&'a str> {
        let remaining = &self.input[self.position..];
        if remaining.len() >= n {
            Some(&remaining[..n])
        } else {
            None
        }
    }

    #[inline]
    fn consume(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.position += c.len_utf8();
        Some(c)
    }

    #[inline]
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.consume();
            } else {
                break;
            }
        }
    }

    #[inline]
    fn is_eof(&self) -> bool {
        self.position >= self.input.len()
    }

    fn parse_command_name(&mut self) -> Option<&'a str> {
        let start = self.position;

        if let Some(first_char) = self.peek()
            && first_char.eq_ignore_ascii_case(&'A')
        {
            self.consume();
            return Some(&self.input[start..self.position]);
        }

        while let Some(c) = self.peek() {
            if c.is_ascii_alphabetic() {
                self.consume();
            } else {
                break;
            }
        }

        if self.position > start {
            Some(&self.input[start..self.position])
        } else {
            None
        }
    }

    fn parse_parameters(&mut self) -> Vec<&'a str> {
        let mut params = Vec::new();

        loop {
            self.skip_whitespace();

            let start = self.position;
            let mut param_len = 0;

            while let Some(c) = self.peek() {
                match c {
                    ',' => {
                        break;
                    }
                    '^' | '~' => {
                        if param_len > 0 {
                            params.push(&self.input[start..self.position]);
                        }
                        return params;
                    }
                    c if c.is_whitespace() => {
                        break;
                    }
                    _ => {
                        self.consume();
                        param_len += 1;
                    }
                }
            }

            if param_len > 0 {
                params.push(&self.input[start..self.position]);
            }

            self.skip_whitespace();

            if self.peek() == Some(',') {
                self.consume();
                continue;
            } else {
                break;
            }
        }

        params
    }

    fn parse_field_data(&mut self) -> Option<&'a str> {
        let start = self.position;

        while !self.is_eof() {
            if self.peek_n(3) == Some("^FS") {
                let data = &self.input[start..self.position];
                return Some(data);
            }
            self.consume();
        }

        if self.position > start {
            Some(&self.input[start..])
        } else {
            None
        }
    }

    pub fn next_token(&mut self) -> Option<Token<'a>> {
        self.skip_whitespace();

        if self.is_eof() {
            return None;
        }

        let start = self.position;

        let prefix = match self.peek()? {
            '^' => {
                self.consume();
                CommandPrefix::Caret
            }
            '~' => {
                self.consume();
                CommandPrefix::Tilde
            }
            _ => {
                while !self.is_eof() {
                    if matches!(self.peek(), Some('^') | Some('~')) {
                        break;
                    }
                    self.consume();
                }

                return Some(Token::Unknown {
                    content: &self.input[start..self.position],
                    span: Span {
                        start,
                        end: self.position,
                    },
                });
            }
        };

        let name = self.parse_command_name()?;

        let token = match (prefix, name) {
            (CommandPrefix::Caret, "XA") => Token::StartToken,
            (CommandPrefix::Caret, "XZ") => Token::EndToken,
            (CommandPrefix::Caret, "FS") => Token::FieldSeparator,

            (CommandPrefix::Caret, "FD") => {
                if let Some(data) = self.parse_field_data() {
                    Token::FieldData {
                        data,
                        span: Span {
                            start,
                            end: self.position,
                        },
                    }
                } else {
                    Token::Command {
                        prefix,
                        name,
                        params: vec![],
                        span: Span {
                            start,
                            end: self.position,
                        },
                    }
                }
            }

            _ => {
                let params = self.parse_parameters();

                Token::Command {
                    prefix,
                    name,
                    params,
                    span: Span {
                        start,
                        end: self.position,
                    },
                }
            }
        };

        Some(token)
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_zpl() {
        let testzpl = include_str!("../testzpls/zpl1.zpl").to_string();

        let tokenizer = Tokenizer::new(&testzpl);

        let mut buffer = String::new();

        for token in tokenizer {
            match token {
                Token::StartToken => {
                    buffer += "^XA";
                }
                Token::EndToken => {
                    buffer += "^XZ";
                }
                Token::FieldSeparator => {
                    buffer += "^FS";
                }
                Token::Command {
                    prefix,
                    name,
                    params,
                    ..
                } => {
                    let prefix = match prefix {
                        CommandPrefix::Caret => "^",
                        CommandPrefix::Tilde => "~",
                    };

                    let params = params.join("");

                    buffer += &format!("{}{}{}", prefix, name, params);
                }
                Token::FieldData { data, .. } => {
                    buffer += &format!("^FD{}", data);
                }
                Token::Unknown { content, .. } => {
                    buffer += content;
                }
            };
        }

        let original = testzpl.replace("\n", "").replace(" ", "").replace(",", "");
        let ours = buffer.replace("\n", "").replace(" ", "").replace(",", "");

        assert_eq!(ours, original);
    }

    #[test]
    fn test_start_and_end_tokens() {
        let input = "^XA^XZ";
        let tokens: Vec<Token> = Tokenizer::new(input).collect();

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::StartToken);
        assert_eq!(tokens[1], Token::EndToken);
    }

    #[test]
    fn test_field_separator() {
        let input = "^FS";
        let tokens: Vec<Token> = Tokenizer::new(input).collect();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::FieldSeparator);
    }

    #[test]
    fn test_simple_command_no_params() {
        let input = "^FX";
        let tokens: Vec<Token> = Tokenizer::new(input).collect();

        assert_eq!(tokens.len(), 1);
        match &tokens[0] {
            Token::Command {
                prefix,
                name,
                params,
                ..
            } => {
                assert_eq!(*prefix, CommandPrefix::Caret);
                assert_eq!(*name, "FX");
                assert_eq!(params.len(), 0);
            }
            _ => panic!("Expected Command token"),
        }
    }

    #[test]
    fn test_command_with_single_param() {
        let input = "^LT120";
        let tokens: Vec<Token> = Tokenizer::new(input).collect();

        assert_eq!(tokens.len(), 1);
        match &tokens[0] {
            Token::Command {
                prefix,
                name,
                params,
                ..
            } => {
                assert_eq!(*prefix, CommandPrefix::Caret);
                assert_eq!(*name, "LT");
                assert_eq!(params, &vec!["120"]);
            }
            _ => panic!("Expected Command token"),
        }
    }

    #[test]
    fn test_command_with_multiple_params() {
        let input = "^FO50,173";
        let tokens: Vec<Token> = Tokenizer::new(input).collect();

        assert_eq!(tokens.len(), 1);
        match &tokens[0] {
            Token::Command {
                prefix,
                name,
                params,
                ..
            } => {
                assert_eq!(*prefix, CommandPrefix::Caret);
                assert_eq!(*name, "FO");
                assert_eq!(params, &vec!["50", "173"]);
            }
            _ => panic!("Expected Command token"),
        }
    }

    #[test]
    fn test_command_with_many_params() {
        let input = "^FB542,1,0,N,0";
        let tokens: Vec<Token> = Tokenizer::new(input).collect();

        assert_eq!(tokens.len(), 1);
        match &tokens[0] {
            Token::Command {
                prefix,
                name,
                params,
                ..
            } => {
                assert_eq!(*prefix, CommandPrefix::Caret);
                assert_eq!(*name, "FB");
                assert_eq!(params, &vec!["542", "1", "0", "N", "0"]);
            }
            _ => panic!("Expected Command token"),
        }
    }

    #[test]
    fn test_tilde_prefix_command() {
        let input = "~SD15";
        let tokens: Vec<Token> = Tokenizer::new(input).collect();

        assert_eq!(tokens.len(), 1);
        match &tokens[0] {
            Token::Command {
                prefix,
                name,
                params,
                ..
            } => {
                assert_eq!(*prefix, CommandPrefix::Tilde);
                assert_eq!(*name, "SD");
                assert_eq!(params, &vec!["15"]);
            }
            _ => panic!("Expected Command token"),
        }
    }

    #[test]
    fn test_field_data_basic() {
        let input = "^FDHello World^FS";
        let tokens: Vec<Token> = Tokenizer::new(input).collect();

        assert_eq!(tokens.len(), 2);
        match &tokens[0] {
            Token::FieldData { data, .. } => {
                assert_eq!(*data, "Hello World");
            }
            _ => panic!("Expected FieldData token"),
        }
        assert_eq!(tokens[1], Token::FieldSeparator);
    }

    #[test]
    fn test_field_data_empty() {
        let input = "^FD^FS";
        let tokens: Vec<Token> = Tokenizer::new(input).collect();

        assert_eq!(tokens.len(), 2);
        match &tokens[0] {
            Token::FieldData { data, .. } => {
                assert_eq!(*data, "");
            }
            _ => panic!("Expected FieldData token"),
        }
        assert_eq!(tokens[1], Token::FieldSeparator);
    }

    #[test]
    fn test_field_data_with_special_chars() {
        let input = "^FDPRODUCT A x 1, PRODUCT B x 4\\&^FS";
        let tokens: Vec<Token> = Tokenizer::new(input).collect();

        assert_eq!(tokens.len(), 2);
        match &tokens[0] {
            Token::FieldData { data, .. } => {
                assert_eq!(*data, "PRODUCT A x 1, PRODUCT B x 4\\&");
            }
            _ => panic!("Expected FieldData token"),
        }
    }

    #[test]
    fn test_field_data_without_terminator() {
        let input = "^FDUnterminated data";
        let tokens: Vec<Token> = Tokenizer::new(input).collect();

        assert_eq!(tokens.len(), 1);
        match &tokens[0] {
            Token::FieldData { data, .. } => {
                assert_eq!(*data, "Unterminated data");
            }
            _ => panic!("Expected FieldData token"),
        }
    }

    #[test]
    fn test_unknown_content() {
        let input = "Random text before ZPL^XA^XZ";
        let tokens: Vec<Token> = Tokenizer::new(input).collect();

        assert_eq!(tokens.len(), 3);
        match &tokens[0] {
            Token::Unknown { content, .. } => {
                assert_eq!(*content, "Random text before ZPL");
            }
            _ => panic!("Expected Unknown token"),
        }
        assert_eq!(tokens[1], Token::StartToken);
        assert_eq!(tokens[2], Token::EndToken);
    }

    #[test]
    fn test_whitespace_handling() {
        let input = "  ^XA  ^LT120  ^XZ  ";
        let tokens: Vec<Token> = Tokenizer::new(input).collect();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::StartToken);
        match &tokens[1] {
            Token::Command { name, params, .. } => {
                assert_eq!(*name, "LT");
                assert_eq!(params, &vec!["120"]);
            }
            _ => panic!("Expected Command token"),
        }
        assert_eq!(tokens[2], Token::EndToken);
    }

    #[test]
    fn test_params_with_whitespace() {
        let input = "^FO 50 , 173";
        let tokens: Vec<Token> = Tokenizer::new(input).collect();

        assert_eq!(tokens.len(), 1);
        match &tokens[0] {
            Token::Command { params, .. } => {
                assert_eq!(params, &vec!["50", "173"]);
            }
            _ => panic!("Expected Command token"),
        }
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let tokens: Vec<Token> = Tokenizer::new(input).collect();

        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_only_whitespace() {
        let input = "   \n\t  ";
        let tokens: Vec<Token> = Tokenizer::new(input).collect();

        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_complex_label_sequence() {
        let input = "^XA^CFB,25^FO50,100^FDTest Label^FS^XZ";
        let tokens: Vec<Token> = Tokenizer::new(input).collect();

        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0], Token::StartToken);

        match &tokens[1] {
            Token::Command { name, params, .. } => {
                assert_eq!(*name, "CF");
                assert_eq!(params, &vec!["B", "25"]);
            }
            _ => panic!("Expected CF command"),
        }

        match &tokens[2] {
            Token::Command { name, params, .. } => {
                assert_eq!(*name, "FO");
                assert_eq!(params, &vec!["50", "100"]);
            }
            _ => panic!("Expected FO command"),
        }

        match &tokens[3] {
            Token::FieldData { data, .. } => {
                assert_eq!(*data, "Test Label");
            }
            _ => panic!("Expected FieldData"),
        }

        assert_eq!(tokens[4], Token::FieldSeparator);
        assert_eq!(tokens[5], Token::EndToken);
    }

    #[test]
    fn test_span_tracking() {
        let input = "^XA^LT120";
        let tokens: Vec<Token> = Tokenizer::new(input).collect();

        assert_eq!(tokens.len(), 2);

        match &tokens[1] {
            Token::Command { span, .. } => {
                assert_eq!(span.start, 3);
                assert_eq!(span.end, 9);
            }
            _ => panic!("Expected Command token"),
        }
    }

    #[test]
    fn test_mixed_prefixes() {
        let input = "^XA~SD15^LT100~CC^XZ";
        let tokens: Vec<Token> = Tokenizer::new(input).collect();

        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0], Token::StartToken);

        match &tokens[1] {
            Token::Command { prefix, name, .. } => {
                assert_eq!(*prefix, CommandPrefix::Tilde);
                assert_eq!(*name, "SD");
            }
            _ => panic!("Expected ~SD command"),
        }

        match &tokens[2] {
            Token::Command { prefix, name, .. } => {
                assert_eq!(*prefix, CommandPrefix::Caret);
                assert_eq!(*name, "LT");
            }
            _ => panic!("Expected ^LT command"),
        }

        match &tokens[3] {
            Token::Command { prefix, name, .. } => {
                assert_eq!(*prefix, CommandPrefix::Tilde);
                assert_eq!(*name, "CC");
            }
            _ => panic!("Expected ~CC command"),
        }

        assert_eq!(tokens[4], Token::EndToken);
    }

    #[test]
    fn test_consecutive_field_data() {
        let input = "^FDFirst^FS^FDSecond^FS";
        let tokens: Vec<Token> = Tokenizer::new(input).collect();

        assert_eq!(tokens.len(), 4);

        match &tokens[0] {
            Token::FieldData { data, .. } => {
                assert_eq!(*data, "First");
            }
            _ => panic!("Expected first FieldData"),
        }

        assert_eq!(tokens[1], Token::FieldSeparator);

        match &tokens[2] {
            Token::FieldData { data, .. } => {
                assert_eq!(*data, "Second");
            }
            _ => panic!("Expected second FieldData"),
        }

        assert_eq!(tokens[3], Token::FieldSeparator);
    }
}
