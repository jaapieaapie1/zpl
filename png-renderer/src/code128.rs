use zpl_parser::command::{Code128Mode, Code128Subset};

/// Bar/space widths (in modules) for symbol values 0..=105. Each symbol is 11 modules.
const PATTERNS: [[u8; 6]; 106] = [
    [2, 1, 2, 2, 2, 2], [2, 2, 2, 1, 2, 2], [2, 2, 2, 2, 2, 1], [1, 2, 1, 2, 2, 3],
    [1, 2, 1, 3, 2, 2], [1, 3, 1, 2, 2, 2], [1, 2, 2, 2, 1, 3], [1, 2, 2, 3, 1, 2],
    [1, 3, 2, 2, 1, 2], [2, 2, 1, 2, 1, 3], [2, 2, 1, 3, 1, 2], [2, 3, 1, 2, 1, 2],
    [1, 1, 2, 2, 3, 2], [1, 2, 2, 1, 3, 2], [1, 2, 2, 2, 3, 1], [1, 1, 3, 2, 2, 2],
    [1, 2, 3, 1, 2, 2], [1, 2, 3, 2, 2, 1], [2, 2, 3, 2, 1, 1], [2, 2, 1, 1, 3, 2],
    [2, 2, 1, 2, 3, 1], [2, 1, 3, 2, 1, 2], [2, 2, 3, 1, 1, 2], [3, 1, 2, 1, 3, 1],
    [3, 1, 1, 2, 2, 2], [3, 2, 1, 1, 2, 2], [3, 2, 1, 2, 2, 1], [3, 1, 2, 2, 1, 2],
    [3, 2, 2, 1, 1, 2], [3, 2, 2, 2, 1, 1], [2, 1, 2, 1, 2, 3], [2, 1, 2, 3, 2, 1],
    [2, 3, 2, 1, 2, 1], [1, 1, 1, 3, 2, 3], [1, 3, 1, 1, 2, 3], [1, 3, 1, 3, 2, 1],
    [1, 1, 2, 3, 1, 3], [1, 3, 2, 1, 1, 3], [1, 3, 2, 3, 1, 1], [2, 1, 1, 3, 1, 3],
    [2, 3, 1, 1, 1, 3], [2, 3, 1, 3, 1, 1], [1, 1, 2, 1, 3, 3], [1, 1, 2, 3, 3, 1],
    [1, 3, 2, 1, 3, 1], [1, 1, 3, 1, 2, 3], [1, 1, 3, 3, 2, 1], [1, 3, 3, 1, 2, 1],
    [3, 1, 3, 1, 2, 1], [2, 1, 1, 3, 3, 1], [2, 3, 1, 1, 3, 1], [2, 1, 3, 1, 1, 3],
    [2, 1, 3, 3, 1, 1], [2, 1, 3, 1, 3, 1], [3, 1, 1, 1, 2, 3], [3, 1, 1, 3, 2, 1],
    [3, 3, 1, 1, 2, 1], [3, 1, 2, 1, 1, 3], [3, 1, 2, 3, 1, 1], [3, 3, 2, 1, 1, 1],
    [3, 1, 4, 1, 1, 1], [2, 2, 1, 4, 1, 1], [4, 3, 1, 1, 1, 1], [1, 1, 1, 2, 2, 4],
    [1, 1, 1, 4, 2, 2], [1, 2, 1, 1, 2, 4], [1, 2, 1, 4, 2, 1], [1, 4, 1, 1, 2, 2],
    [1, 4, 1, 2, 2, 1], [1, 1, 2, 2, 1, 4], [1, 1, 2, 4, 1, 2], [1, 2, 2, 1, 1, 4],
    [1, 2, 2, 4, 1, 1], [1, 4, 2, 1, 1, 2], [1, 4, 2, 2, 1, 1], [2, 4, 1, 2, 1, 1],
    [2, 2, 1, 1, 1, 4], [4, 1, 3, 1, 1, 1], [2, 4, 1, 1, 1, 2], [1, 3, 4, 1, 1, 1],
    [1, 1, 1, 2, 4, 2], [1, 2, 1, 1, 4, 2], [1, 2, 1, 2, 4, 1], [1, 1, 4, 2, 1, 2],
    [1, 2, 4, 1, 1, 2], [1, 2, 4, 2, 1, 1], [4, 1, 1, 2, 1, 2], [4, 2, 1, 1, 1, 2],
    [4, 2, 1, 2, 1, 1], [2, 1, 2, 1, 4, 1], [2, 1, 4, 1, 2, 1], [4, 1, 2, 1, 2, 1],
    [1, 1, 1, 1, 4, 3], [1, 1, 1, 3, 4, 1], [1, 3, 1, 1, 4, 1], [1, 1, 4, 1, 1, 3],
    [1, 1, 4, 3, 1, 1], [4, 1, 1, 1, 1, 3], [4, 1, 1, 3, 1, 1], [1, 1, 3, 1, 4, 1],
    [1, 1, 4, 1, 3, 1], [3, 1, 1, 1, 4, 1], [4, 1, 1, 1, 3, 1], [2, 1, 1, 4, 1, 2],
    [2, 1, 1, 2, 1, 4], [2, 1, 1, 2, 3, 2],
];

/// Stop pattern (value 106) including the terminating bar: 13 modules.
const STOP: [u8; 7] = [2, 3, 3, 1, 1, 1, 2];

const SHIFT: u8 = 98;
const CODE_C: u8 = 99;
const CODE_B: u8 = 100;
const CODE_A: u8 = 101;
const FNC1: u8 = 102;
const START_A: u8 = 103;
const START_B: u8 = 104;
const START_C: u8 = 105;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Code128 {
    /// Symbol values: start code, data, check character (stop not included).
    pub values: Vec<u8>,
    /// Human readable interpretation line (invocation codes removed).
    pub text: String,
}

impl Code128 {
    /// Bar/space widths in modules, starting with a bar, including the stop pattern.
    pub fn widths(&self) -> Vec<u8> {
        let mut widths: Vec<u8> = self.values.iter()
            .flat_map(|&v| PATTERNS[v as usize])
            .collect();
        widths.extend_from_slice(&STOP);
        widths
    }
}

/// Mode D interpretation line (Labelary): font 0 (Triumvirate) at `em` dots both ways,
/// scaled so the ink spans [`Gs1Line::FILL`] of the bar width, centred under (or over) the
/// bars. Below the bars the capitals start [`Gs1Line::GAP`] dots down; above them the
/// baseline sits [`Gs1Line::ABOVE`] em over the bars.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Gs1Line {
    pub em: f32,
    /// Pen position of the first character, in the unrotated symbol frame.
    pub x: f32,
    pub baseline: f32,
    /// Height the line adds to the symbol.
    pub area: u32,
    /// Top of the bars in the same frame.
    pub bars_y: u32,
}

impl Gs1Line {
    const FILL: f32 = 0.955;
    const GAP: f32 = 5.0;
    const ABOVE: f32 = 0.41;
    /// Cap height and descent of font 0, in em.
    const CAP: f32 = 0.75;
    const DESCENT: f32 = 0.294;

    /// Layout of `text` for bars `bars_w` x `height` dots; `None` for text without ink.
    pub fn new(text: &str, bars_w: u32, height: u32, above: bool) -> Option<Self> {
        use ab_glyph::{Font, FontRef};
        let font = FontRef::try_from_slice(crate::TRIUMVIRATE).ok()?;
        // Ink extent along the line, in font units, with the pen starting at 0.
        let (mut pen, mut left, mut right) = (0.0f32, f32::MAX, f32::MIN);
        for c in text.chars() {
            let glyph = font.glyph_id(c);
            if let Some(outline) = font.outline(glyph) {
                left = left.min(pen + outline.bounds.min.x);
                right = right.max(pen + outline.bounds.max.x);
            }
            pen += font.h_advance_unscaled(glyph);
        }
        if right <= left {
            return None;
        }
        let em = Self::FILL * bars_w as f32 * font.units_per_em().unwrap_or(1000.0) / (right - left);
        let unit = em / font.units_per_em().unwrap_or(1000.0);
        let x = (bars_w as f32 - (right - left) * unit) / 2.0 - left * unit;
        Some(if above {
            let area = (em * (Self::CAP + Self::ABOVE)).ceil() as u32;
            Self { em, x, baseline: area as f32 - em * Self::ABOVE, area, bars_y: area }
        } else {
            let area = (Self::GAP + em * (Self::CAP + Self::DESCENT)).ceil() as u32;
            Self { em, x, baseline: height as f32 + Self::GAP + em * Self::CAP, area, bars_y: 0 }
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Char(u8),
    Raw(u8),
    Start(Code128Subset),
}

pub fn encode(data: &str, mode: Code128Mode) -> Code128 {
    let mut values = match mode {
        Code128Mode::AutoMode => encode_invocation(data, None),
        Code128Mode::Subset(subset) => encode_invocation(data, Some(subset)),
        Code128Mode::Auto => encode_auto(data, false),
        Code128Mode::Gs1 => encode_gs1(data),
        Code128Mode::UccCase => encode_ucc_case(data),
    };
    let checksum = checksum(&values.0);
    values.0.push(checksum);
    Code128 { values: values.0, text: values.1 }
}

pub fn checksum(values: &[u8]) -> u8 {
    let sum: u32 = values.iter()
        .enumerate()
        .map(|(i, &v)| (i.max(1) as u32) * v as u32)
        .sum();
    (sum % 103) as u8
}

/// Splits ZPL field data into characters and `>` invocation codes.
fn tokenize(data: &str) -> Vec<Token> {
    let bytes: Vec<u8> = data.chars()
        .filter(|c| c.is_ascii())
        .map(|c| c as u8)
        .collect();
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'>' && i + 1 < bytes.len() {
            let token = match bytes[i + 1] {
                b'<' | b'0' => Some(Token::Char(b'>')),
                b'=' => Some(Token::Char(b'~')),
                b'1' => Some(Token::Raw(95)),
                b'2' => Some(Token::Raw(96)),
                b'3' => Some(Token::Raw(97)),
                b'4' => Some(Token::Raw(SHIFT)),
                b'5' => Some(Token::Raw(CODE_C)),
                b'6' => Some(Token::Raw(CODE_B)),
                b'7' => Some(Token::Raw(CODE_A)),
                b'8' => Some(Token::Raw(FNC1)),
                b'9' => Some(Token::Start(Code128Subset::A)),
                b':' => Some(Token::Start(Code128Subset::B)),
                b';' => Some(Token::Start(Code128Subset::C)),
                _ => None,
            };
            if let Some(token) = token {
                tokens.push(token);
                i += 2;
                continue;
            }
        }
        tokens.push(Token::Char(bytes[i]));
        i += 1;
    }

    tokens
}

fn start_value(subset: Code128Subset) -> u8 {
    match subset {
        Code128Subset::A => START_A,
        Code128Subset::B => START_B,
        Code128Subset::C => START_C,
    }
}

fn char_value(c: u8, subset: Code128Subset) -> Option<u8> {
    match subset {
        Code128Subset::A => match c {
            0..=31 => Some(c + 64),
            32..=95 => Some(c - 32),
            _ => None,
        },
        Code128Subset::B => match c {
            32..=127 => Some(c - 32),
            _ => None,
        },
        Code128Subset::C => None,
    }
}

fn is_digit_token(token: Option<&Token>) -> bool {
    matches!(token, Some(Token::Char(c)) if c.is_ascii_digit())
}

/// Zebra mode N: subset B by default, data may contain `>` invocation codes.
fn encode_invocation(data: &str, forced_start: Option<Code128Subset>) -> (Vec<u8>, String) {
    let tokens = tokenize(data);
    let mut i = 0;

    let mut subset = forced_start.unwrap_or(Code128Subset::B);
    if let Some(Token::Start(s)) = tokens.first() {
        subset = *s;
        i = 1;
    }

    let mut values = vec![start_value(subset)];
    let mut text = String::new();
    let mut shift = false;

    while i < tokens.len() {
        match tokens[i] {
            Token::Start(_) => {}
            Token::Raw(v) => match subset {
                Code128Subset::C => match v {
                    CODE_B => { values.push(v); subset = Code128Subset::B; }
                    CODE_A => { values.push(v); subset = Code128Subset::A; }
                    FNC1 => values.push(v),
                    _ => {}
                },
                _ => {
                    values.push(v);
                    match (v, subset) {
                        (CODE_C, _) => subset = Code128Subset::C,
                        (CODE_B, Code128Subset::A) => subset = Code128Subset::B,
                        (CODE_A, Code128Subset::B) => subset = Code128Subset::A,
                        (SHIFT, _) => shift = true,
                        _ => {}
                    }
                }
            },
            Token::Char(c) => match subset {
                Code128Subset::C => {
                    if c.is_ascii_digit() && is_digit_token(tokens.get(i + 1)) {
                        if let Some(Token::Char(d)) = tokens.get(i + 1) {
                            values.push((c - b'0') * 10 + (d - b'0'));
                            text.push(c as char);
                            text.push(*d as char);
                        }
                        i += 1;
                    } else {
                        // Odd digit or non-digit in subset C: fall back to subset B.
                        values.push(CODE_B);
                        subset = Code128Subset::B;
                        continue;
                    }
                }
                _ => {
                    let target = if shift {
                        shift = false;
                        if subset == Code128Subset::A { Code128Subset::B } else { Code128Subset::A }
                    } else {
                        subset
                    };
                    if let Some(v) = char_value(c, target) {
                        values.push(v);
                        text.push(c as char);
                    }
                }
            },
        }
        i += 1;
    }

    (values, text)
}

/// Zebra mode A: automatic subset selection, digit runs packed into subset C. With `gs1`
/// the symbol starts in subset C on any leading digit pair (application identifiers are at
/// least two digits), as Labelary does in mode D.
fn encode_auto(data: &str, gs1: bool) -> (Vec<u8>, String) {
    let tokens: Vec<Token> = tokenize(data)
        .into_iter()
        .filter_map(|t| match t {
            Token::Raw(FNC1) => Some(t),
            Token::Raw(_) | Token::Start(_) => None,
            Token::Char(_) => Some(t),
        })
        .collect();

    let digit_run = |from: usize| tokens[from..].iter()
        .take_while(|t| is_digit_token(Some(t)))
        .count();

    let best_ab = |from: usize| {
        for t in &tokens[from..] {
            if let Token::Char(c) = t {
                if *c < 32 { return Code128Subset::A; }
                if *c >= 96 { return Code128Subset::B; }
            }
        }
        Code128Subset::B
    };

    // Leading FNC1s do not count: the start character comes before them.
    let first = tokens.iter().take_while(|t| **t == Token::Raw(FNC1)).count();
    let first_run = digit_run(first);
    let mut subset = if first_run >= 4 || (first_run == 2 && tokens.len() == first + 2) || (gs1 && first_run >= 2) {
        Code128Subset::C
    } else {
        best_ab(0)
    };

    let mut values = vec![start_value(subset)];
    let mut text = String::new();
    let mut i = 0;

    while i < tokens.len() {
        if tokens[i] == Token::Raw(FNC1) {
            values.push(FNC1);
            i += 1;
            continue;
        }

        let c = match tokens[i] { Token::Char(c) => c, _ => { i += 1; continue; } };

        if subset == Code128Subset::C {
            if digit_run(i) >= 2 {
                if let Token::Char(d) = tokens[i + 1] {
                    values.push((c - b'0') * 10 + (d - b'0'));
                    text.push(c as char);
                    text.push(d as char);
                }
                i += 2;
            } else {
                subset = best_ab(i);
                values.push(if subset == Code128Subset::A { CODE_A } else { CODE_B });
            }
            continue;
        }

        let run = digit_run(i);
        if run >= 4 && (run % 2 == 0 || run >= 5) && (i + run == tokens.len() || run >= 6) {
            if run % 2 == 1 {
                if let Some(v) = char_value(c, subset) {
                    values.push(v);
                    text.push(c as char);
                }
                i += 1;
            }
            values.push(CODE_C);
            subset = Code128Subset::C;
            continue;
        }

        match char_value(c, subset) {
            Some(v) => values.push(v),
            None => {
                let other = if subset == Code128Subset::A { Code128Subset::B } else { Code128Subset::A };
                match char_value(c, other) {
                    Some(v) => {
                        values.push(if other == Code128Subset::A { CODE_A } else { CODE_B });
                        values.push(v);
                        subset = other;
                    }
                    None => { i += 1; continue; }
                }
            }
        }
        text.push(c as char);
        i += 1;
    }

    (values, text)
}

/// Zebra mode D: UCC/EAN-128. FNC1 follows the start character; parentheses and spaces are
/// stripped from the bars but kept in the interpretation line. Like Labelary, no FNC1
/// separates chained application identifiers, and no check digit is added.
fn encode_gs1(data: &str) -> (Vec<u8>, String) {
    let mut encoded = String::with_capacity(data.len() + 2);
    encoded.push_str(">8");
    encoded.extend(data.chars().filter(|c| !matches!(c, '(' | ')' | ' ')));
    let (values, _) = encode_auto(&encoded, true);
    (values, data.chars().filter(|&c| c.is_ascii() && c >= ' ').collect())
}

/// Zebra mode U: UCC case mode, 19 digits (padded / truncated) plus a mod-10 check digit.
fn encode_ucc_case(data: &str) -> (Vec<u8>, String) {
    let mut digits: Vec<u8> = data.bytes().filter(u8::is_ascii_digit).map(|d| d - b'0').take(19).collect();
    digits.resize(19, 0);

    let sum: u32 = digits.iter()
        .rev()
        .enumerate()
        .map(|(i, &d)| d as u32 * if i % 2 == 0 { 3 } else { 1 })
        .sum();
    digits.push(((10 - sum % 10) % 10) as u8);

    let mut values = vec![START_C, FNC1];
    values.extend(digits.chunks(2).map(|p| p[0] * 10 + p[1]));
    let text = digits.iter().map(|d| (b'0' + d) as char).collect();

    (values, text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_patterns_are_eleven_modules() {
        for (value, pattern) in PATTERNS.iter().enumerate() {
            let sum: u8 = pattern.iter().sum();
            assert_eq!(sum, 11, "value {}", value);
        }
        assert_eq!(STOP.iter().sum::<u8>(), 13);
    }

    #[test]
    fn mode_n_digits_stay_in_subset_b() {
        let code = encode("789878907890", Code128Mode::AutoMode);
        let mut expected = vec![START_B];
        expected.extend("789878907890".bytes().map(|c| c - 32));
        // (104 + sum(i * v)) mod 103
        expected.push(71);
        assert_eq!(code.values, expected);
        assert_eq!(code.text, "789878907890");

        let modules: u32 = code.widths().iter().map(|&w| w as u32).sum();
        assert_eq!(modules, 167);
    }

    #[test]
    fn mode_n_start_c_invocation() {
        let code = encode(">;123456", Code128Mode::AutoMode);
        assert_eq!(code.values, vec![START_C, 12, 34, 56, 44]);
        assert_eq!(code.text, "123456");
    }

    #[test]
    fn auto_mode_packs_digits_in_subset_c() {
        let code = encode("123456", Code128Mode::Auto);
        assert_eq!(code.values, vec![START_C, 12, 34, 56, 44]);

        let code = encode("AB1234", Code128Mode::Auto);
        assert_eq!(code.values[..4], [START_B, 33, 34, CODE_C]);
        assert_eq!(code.values[4..6], [12, 34]);
        assert_eq!(code.text, "AB1234");
    }

    #[test]
    fn invocation_codes_are_not_printed() {
        let code = encode(">:AB>5123456>6CD><", Code128Mode::AutoMode);
        assert_eq!(&code.values[..code.values.len() - 1],
                   &[START_B, 33, 34, CODE_C, 12, 34, 56, CODE_B, 35, 36, 30]);
        assert_eq!(code.text, "AB123456CD>");
    }

    #[test]
    fn mode_d_matches_labelary() {
        // Labelary's symbols: start C, FNC1, AIs packed in C, no FNC1 between chained AIs.
        let code = encode("(01)12345678901231(10)ABC123", Code128Mode::Gs1);
        assert_eq!(code.values[..code.values.len() - 1],
                   [START_C, FNC1, 1, 12, 34, 56, 78, 90, 12, 31, 10, CODE_B, 33, 34, 35, 17, 18, 19]);
        assert_eq!(code.text, "(01)12345678901231(10)ABC123");
        let code = encode("(10) A1", Code128Mode::Gs1);
        assert_eq!(code.values[..code.values.len() - 1], [START_C, FNC1, 10, CODE_B, 33, 17]);
        assert_eq!(code.text, "(10) A1");
        let code = encode("(10)AB(21)CD", Code128Mode::Gs1);
        assert_eq!(code.values[..code.values.len() - 1], [START_C, FNC1, 10, CODE_B, 33, 34, 18, 17, 35, 36]);
    }

    #[test]
    fn mode_d_line_spans_the_bars() {
        let below = Gs1Line::new("(10)A1", 180, 80, false).unwrap();
        let above = Gs1Line::new("(10)A1", 180, 80, true).unwrap();
        // Labelary prints this ~65-70 dots tall; ink 0.955 of the bars, centred.
        assert!((below.em - 70.0).abs() < 3.0, "{below:?}");
        assert_eq!(below.em, above.em);
        assert!(below.x.abs() < 5.0, "{below:?}");
        assert_eq!((below.bars_y, above.bars_y), (0, above.area));
        assert!(below.baseline > 85.0 && above.baseline < above.area as f32);
        assert_eq!(Gs1Line::new(" ", 180, 80, false), None);
    }

    #[test]
    fn widths_start_with_start_b_and_end_with_stop() {
        let widths = encode("A", Code128Mode::AutoMode).widths();
        assert_eq!(&widths[..6], &[2, 1, 1, 2, 1, 4]);
        assert_eq!(&widths[widths.len() - 7..], &STOP);
    }
}
