use zpl_tokenizer::{Token, Tokenizer};

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    peeked: Option<Token<'a>>,
}
