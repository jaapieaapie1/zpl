//! Anonymizes ZPL for bug reports. Every letter and digit of field data (^FD, ^FV),
//! comments (^FX), serial numbers (^SN) and stray text between commands is replaced by a
//! random one of the same kind: a digit by a digit, an uppercase letter by an uppercase
//! letter, a letter of another script by a letter near it in the same script. Length,
//! spaces and punctuation stay, so the label lays out as before and its barcodes still
//! encode, but it no longer says anything. Commands and their parameters stay as they are,
//! and so do graphics (^GF, ~DG, ~DY, ~DU, ~DT): an image cannot be scrambled, so
//! [`Anonymized::graphics`] counts them for the reporter to check.
//!
//! What is kept so the label still renders the same way:
//! - ^FH hex escapes stay escapes: escaped letters and digits are replaced and escaped again.
//! - QR codes (^BQ) keep their switches (`QA,`, `MM,N`, `MM,B0012`, mixed mode `D…,`).
//! - Code 128 (^BC) keeps its invocation codes (`>;`, `>8`, ...), and barcodes keep GS1
//!   application identifiers in brackets (`(01)`).
//! - ^FC clock placeholders (`%Y`) and Data Matrix escapes (^BX quality 200, `~1`, `~d065`).
//! - EAN-13, EAN-8, UPC-A and GS1 DataBar data get a correct check digit again.
//! - RFID data written in hex (^RF with format H) stays hex.
//!
//! The replacement of a word (a run of letters and digits) depends only on the word and the
//! key, so a value that appears in several places, such as an order number in text and in a
//! barcode, gets the same replacement everywhere in one report. Without the key the
//! original cannot be worked out, so the key must not be kept with the report.

use sha2::{Digest, Sha256};
use zpl_tokenizer::{CommandPrefix, Token, Tokenizer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Anonymized {
    pub zpl: String,
    /// Fields, comments and other pieces of text that were scrambled.
    pub scrambled: usize,
    /// Graphics, stored images and fonts, which were left as they are.
    pub graphics: usize,
}

/// `zpl` with its text and barcode data replaced, using `key` (random, and not kept) for
/// the replacements.
pub fn anonymize(zpl: &str, key: &[u8; 32]) -> Anonymized {
    let words = Words { key };
    let mut out = Output { source: zpl, text: String::with_capacity(zpl.len()), copied: 0 };
    let mut field = Field::default();
    let (mut scrambled, mut graphics) = (0, 0);
    for token in Tokenizer::new(zpl) {
        match token {
            Token::StartToken | Token::EndToken | Token::FieldSeparator => field = Field::default(),
            Token::FieldData { data, .. } => {
                out.replace(data, field.scramble(data, &words));
                scrambled += 1;
            }
            Token::Unknown { content, .. } => {
                out.replace(content, Field::default().scramble(content, &words));
                scrambled += 1;
            }
            Token::Command { prefix, name, params, .. } => {
                let name = name.to_ascii_uppercase();
                let first_char = |i: usize| params.get(i).and_then(|p| p.trim().chars().next());
                match (prefix, name.as_str()) {
                    // Lowercase ^fs, ^fd and ^fv are not special to the tokenizer.
                    (CommandPrefix::Caret, "FS") => field = Field::default(),
                    (CommandPrefix::Caret, "FD" | "FV") => {
                        for p in &params {
                            out.replace(p, field.scramble(p, &words));
                        }
                        scrambled += 1;
                    }
                    (CommandPrefix::Caret, "FX") => {
                        for p in &params {
                            out.replace(p, Field::default().scramble(p, &words));
                        }
                        scrambled += 1;
                    }
                    // ^SN's first parameter is the starting value, the rest are settings.
                    (CommandPrefix::Caret, "SN") => {
                        if let Some(p) = params.first() {
                            out.replace(p, Field::default().scramble(p, &words));
                            scrambled += 1;
                        }
                    }
                    (CommandPrefix::Caret, "FH") => field.hex = Some(first_char(0).unwrap_or('_')),
                    (CommandPrefix::Caret, "FC") => {
                        field.clock = (0..3).filter_map(first_char).collect();
                        if field.clock.is_empty() {
                            field.clock.push('%');
                        }
                    }
                    (CommandPrefix::Caret, "RF") => field.hex_data = first_char(1).is_some_and(|f| f.eq_ignore_ascii_case(&'H')),
                    (CommandPrefix::Caret, "BX") => {
                        field.barcode = Some(name.clone());
                        if params.get(2).is_some_and(|q| q.trim() == "200") {
                            field.escape = Some(first_char(6).unwrap_or('~'));
                        }
                    }
                    (CommandPrefix::Caret, "BY") => {}
                    (CommandPrefix::Caret, n) if n.len() == 2 && n.starts_with('B') => field.barcode = Some(name.clone()),
                    (CommandPrefix::Caret, "GF") | (CommandPrefix::Tilde, "DG" | "DY" | "DU" | "DT") => graphics += 1,
                    _ => {}
                }
            }
        }
    }
    Anonymized { zpl: out.finish(), scrambled, graphics }
}

/// The anonymized ZPL: the source, with slices of it replaced in order.
struct Output<'a> {
    source: &'a str,
    text: String,
    /// Bytes of `source` copied or replaced so far.
    copied: usize,
}

impl Output<'_> {
    /// Replaces `slice`, which must be part of `source` after everything replaced so far.
    fn replace(&mut self, slice: &str, with: String) {
        let start = slice.as_ptr() as usize - self.source.as_ptr() as usize;
        debug_assert!(start >= self.copied && start + slice.len() <= self.source.len());
        self.text.push_str(&self.source[self.copied..start]);
        self.text.push_str(&with);
        self.copied = start + slice.len();
    }

    fn finish(mut self) -> String {
        self.text.push_str(&self.source[self.copied..]);
        self.text
    }
}

/// What the commands of the current field (up to ^FS) say about its data.
#[derive(Default)]
struct Field {
    /// The barcode command, e.g. `BC`; `None` for text.
    barcode: Option<String>,
    /// The ^FH hex escape indicator.
    hex: Option<char>,
    /// The ^FC clock indicators.
    clock: Vec<char>,
    /// The ^BX escape character (quality 200).
    escape: Option<char>,
    /// RFID data in hex.
    hex_data: bool,
}

impl Field {
    fn scramble(&self, data: &str, words: &Words) -> String {
        let barcode = self.barcode.as_deref();
        let mut out = String::with_capacity(data.len());
        let switches = if barcode == Some("BQ") { qr_switches(data) } else { 0 };
        out.push_str(&data[..switches]);
        let chars: Vec<char> = data[switches..].chars().collect();
        let mut word = String::new();
        let flush = |word: &mut String, out: &mut String| {
            out.push_str(&words.scramble(word, self.hex_data));
            word.clear();
        };
        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];
            // A run of ^FH escapes is decoded as a whole: one UTF-8 character can take several.
            if let Some(indicator) = self.hex.filter(|&h| h == c && hex_escape(&chars, i)) {
                flush(&mut word, &mut out);
                let lower = chars[i + 1..i + 3].iter().any(char::is_ascii_lowercase);
                let mut bytes = Vec::new();
                while i < chars.len() && chars[i] == indicator && hex_escape(&chars, i) {
                    bytes.push((hex_value(chars[i + 1]) << 4) | hex_value(chars[i + 2]));
                    i += 3;
                }
                for b in words.scramble_bytes(&bytes) {
                    out.push(indicator);
                    out.push_str(&if lower { format!("{b:02x}") } else { format!("{b:02X}") });
                }
                continue;
            }
            let keep = if self.clock.contains(&c) || barcode == Some("BC") && c == '>' {
                2
            } else if Some(c) == self.escape {
                // `~dNNN` is a character by its decimal code.
                if chars.get(i + 1).is_some_and(|d| d.eq_ignore_ascii_case(&'d')) { 5 } else { 2 }
            } else if barcode.is_some() && c == '(' {
                application_identifier(&chars, i)
            } else {
                0
            };
            if keep > 0 {
                flush(&mut word, &mut out);
                out.extend(&chars[i..(i + keep).min(chars.len())]);
                i += keep;
                continue;
            }
            if c.is_alphanumeric() {
                word.push(c);
            } else {
                flush(&mut word, &mut out);
                out.push(c);
            }
            i += 1;
        }
        flush(&mut word, &mut out);
        let digits = match barcode {
            Some("BE") => 13,
            Some("B8") => 8,
            Some("BU") => 12,
            Some("BR") => 14,
            _ => 0,
        };
        if digits > 0 && out.len() == digits && out.bytes().all(|b| b.is_ascii_digit()) {
            let check = gs1_check_digit(&out.as_bytes()[..digits - 1]);
            out.replace_range(digits - 1.., &check.to_string());
        }
        out
    }
}

/// Length in bytes of the switches in front of ^BQ data, as png-renderer's `qr::parse`
/// reads them: mixed mode `Diijjxx,`, then `<ec><input>,`, and with manual input the
/// character mode (`N`, `A`, `K`, or `B` and a four digit byte count).
fn qr_switches(data: &str) -> usize {
    let b = data.as_bytes();
    let mut n = if b.len() > 7 && b[0] == b'D' && b[7] == b',' { 8 } else { 0 };
    if b.len() >= n + 3 && b[n + 2] == b',' {
        let manual = b[n + 1] == b'M';
        n += 3;
        if manual {
            match b.get(n) {
                Some(b'N' | b'A' | b'K') => n += 1,
                Some(b'B') if b.len() >= n + 5 && b[n + 1..n + 5].iter().all(u8::is_ascii_digit) => n += 5,
                _ => {}
            }
        }
    }
    if data.is_char_boundary(n) { n } else { 0 }
}

/// Length of a GS1 application identifier in brackets at `i`, such as `(01)`, else 0.
fn application_identifier(chars: &[char], i: usize) -> usize {
    let digits = chars[i + 1..].iter().take_while(|c| c.is_ascii_digit()).count();
    if (2..=4).contains(&digits) && chars.get(i + 1 + digits) == Some(&')') { digits + 2 } else { 0 }
}

fn hex_escape(chars: &[char], i: usize) -> bool {
    i + 2 < chars.len() && chars[i + 1].is_ascii_hexdigit() && chars[i + 2].is_ascii_hexdigit()
}

fn hex_value(c: char) -> u8 {
    c.to_digit(16).unwrap_or(0) as u8
}

/// The GS1 check digit for `digits` (without it): weights 3 and 1 from the right.
fn gs1_check_digit(digits: &[u8]) -> u8 {
    let sum: u32 = digits.iter().rev().enumerate().map(|(i, d)| (d - b'0') as u32 * if i % 2 == 0 { 3 } else { 1 }).sum();
    ((10 - sum % 10) % 10) as u8
}

/// Replacements for words, derived from the key and the word.
struct Words<'k> {
    key: &'k [u8; 32],
}

impl Words<'_> {
    fn stream(&self, kind: u8, input: &[u8]) -> Stream {
        let seed: [u8; 32] = Sha256::new().chain_update(self.key).chain_update([kind]).chain_update(input).finalize().into();
        Stream { seed, block: [0; 32], used: 32, counter: 0 }
    }

    fn scramble(&self, word: &str, hex: bool) -> String {
        if word.is_empty() {
            return String::new();
        }
        let mut stream = self.stream(b'w', word.as_bytes());
        word.chars().map(|c| if hex { hex_digit(c, &mut stream) } else { replace(c, &mut stream) }).collect()
    }

    /// Bytes from ^FH escapes: as UTF-8 text if they are, else byte by byte (only ASCII
    /// letters and digits are replaced then).
    fn scramble_bytes(&self, bytes: &[u8]) -> Vec<u8> {
        let mut stream = self.stream(b'h', bytes);
        match std::str::from_utf8(bytes) {
            Ok(text) => text.chars().map(|c| replace(c, &mut stream)).collect::<String>().into_bytes(),
            Err(_) => bytes.iter().map(|&b| if b.is_ascii_alphanumeric() { replace(b as char, &mut stream) as u8 } else { b }).collect(),
        }
    }
}

/// A random character of the same kind as `c`; anything but letters and digits stays.
fn replace(c: char, stream: &mut Stream) -> char {
    match c {
        '0'..='9' => (b'0' + stream.below(10) as u8) as char,
        'A'..='Z' => (b'A' + stream.below(26) as u8) as char,
        'a'..='z' => (b'a' + stream.below(26) as u8) as char,
        c if c.is_alphabetic() => same_script(c, stream),
        c if c.is_numeric() => (b'0' + stream.below(10) as u8) as char,
        c => c,
    }
}

/// A letter from the same block of 128 code points as `c` (so the same script, and as
/// many UTF-8 bytes) with the same case; an ASCII letter if a few tries find none.
fn same_script(c: char, stream: &mut Stream) -> char {
    let block = c as u32 & !0x7F;
    for _ in 0..32 {
        if let Some(r) = char::from_u32(block + stream.below(128))
            && r.is_alphabetic()
            && r.is_uppercase() == c.is_uppercase()
            && r.is_lowercase() == c.is_lowercase()
        {
            return r;
        }
    }
    let base = if c.is_uppercase() { b'A' } else { b'a' };
    (base + stream.below(26) as u8) as char
}

fn hex_digit(c: char, stream: &mut Stream) -> char {
    match c {
        'a'..='f' => (b'a' + stream.below(6) as u8) as char,
        'A'..='F' => (b'A' + stream.below(6) as u8) as char,
        c => replace(c, stream),
    }
}

/// Random numbers: SHA-256 of the seed and a counter.
struct Stream {
    seed: [u8; 32],
    block: [u8; 32],
    used: usize,
    counter: u64,
}

impl Stream {
    fn below(&mut self, n: u32) -> u32 {
        if self.used + 4 > self.block.len() {
            self.block = Sha256::new().chain_update(self.seed).chain_update(self.counter.to_le_bytes()).finalize().into();
            self.counter += 1;
            self.used = 0;
        }
        let bytes = [self.block[self.used], self.block[self.used + 1], self.block[self.used + 2], self.block[self.used + 3]];
        self.used += 4;
        u32::from_le_bytes(bytes) % n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY: [u8; 32] = [7; 32];

    fn run(zpl: &str) -> String {
        anonymize(zpl, &KEY).zpl
    }

    /// The field data of every ^FD in `zpl`.
    fn fields(zpl: &str) -> Vec<&str> {
        Tokenizer::new(zpl).filter_map(|t| if let Token::FieldData { data, .. } = t { Some(data) } else { None }).collect()
    }

    fn same_shape(a: &str, b: &str) -> bool {
        a.chars().count() == b.chars().count()
            && a.chars().zip(b.chars()).all(|(x, y)| {
                x.is_ascii_digit() == y.is_ascii_digit()
                    && x.is_uppercase() == y.is_uppercase()
                    && x.is_lowercase() == y.is_lowercase()
                    && (x.is_alphanumeric() || x == y)
            })
    }

    #[test]
    fn replaces_text_but_keeps_its_shape_and_the_commands() {
        let zpl = "^XA\n^FO50,60^A0N,30,30^FDJan de Vries, Kerkstraat 12^FS\n^FO50,100^FDtel. 06-12345678^FS\n^XZ";
        let out = run(zpl);
        assert_eq!(out.replace(|c: char| c.is_alphanumeric(), "x"), zpl.replace(|c: char| c.is_alphanumeric(), "x"));
        assert!(out.starts_with("^XA\n^FO50,60^A0N,30,30^FD"));
        for (a, b) in fields(zpl).iter().zip(fields(&out)) {
            assert!(same_shape(a, b), "{a} -> {b}");
            assert_ne!(*a, b);
        }
        assert!(!out.contains("Vries") && !out.contains("12345678"));
    }

    #[test]
    fn same_word_same_replacement_and_another_key_another() {
        let zpl = "^XA^FDOrder 4711^FS^BCN,50^FD4711^FS^XZ";
        let f = fields(&run(zpl)).into_iter().map(String::from).collect::<Vec<_>>();
        assert_eq!(&f[0][6..], f[1]);
        assert_ne!(run(zpl), anonymize(zpl, &[8; 32]).zpl);
        assert_eq!(run(zpl), run(zpl));
    }

    #[test]
    fn comments_serials_and_stray_text() {
        let out = run("^XA^FXcustomer Piet^FS^SN0001234,1,Y^FS^XZ\nPiet");
        assert!(!out.contains("Piet") && !out.contains("customer") && !out.contains("1234"));
        assert!(out.contains(",1,Y^FS"));
    }

    #[test]
    fn hex_escapes_stay_valid() {
        let out = run("^XA^FH^FDM_c3_bcller_5F_7E^FS^FH\\^FD\\41B^FS^XZ");
        let f = fields(&out);
        // ü (two bytes) stays a two byte letter, _5F and _7E are not letters.
        let decoded = |s: &str, ind: char| {
            let mut bytes = Vec::new();
            let mut chars = s.chars().peekable();
            while let Some(c) = chars.next() {
                if c == ind {
                    let h: String = chars.by_ref().take(2).collect();
                    bytes.push(u8::from_str_radix(&h, 16).unwrap());
                } else {
                    bytes.extend(c.to_string().bytes());
                }
            }
            String::from_utf8(bytes).unwrap()
        };
        let a = decoded(f[0], '_');
        assert_eq!(a.chars().count(), 8, "{a}");
        assert!(a.ends_with("_~") && !a.starts_with("Müller"));
        assert!(a.chars().nth(1).is_some_and(|c| !c.is_ascii() && c.is_lowercase()), "{a}");
        assert!(f[0][1..].starts_with("_c"), "{}", f[0]);
        assert_eq!(decoded(f[1], '\\').len(), 2);
    }

    #[test]
    fn qr_switches_stay() {
        let out = run("^XA^BQN,2,4^FDQA,https://example.com/track/ABC123^FS^BQN,2,4^FDMM,B0005hello^FS^BQ^FDD03048F,LM,N0123^FS^XZ");
        let f = fields(&out);
        assert!(f[0].starts_with("QA,") && f[0].contains("://") && !f[0].contains("example"));
        assert!(f[1].starts_with("MM,B0005") && !f[1].ends_with("hello"));
        assert!(f[2].starts_with("D03048F,LM,N") && f[2].len() == 16);
    }

    #[test]
    fn code128_invocations_and_application_identifiers_stay() {
        let f = run("^XA^BCN,50,Y,N,N,D^FD>;>8(01)12345678901231(10)AB^FS^XZ");
        let f = fields(&f)[0];
        assert!(f.starts_with(">;>8(01)") && f.contains("(10)") && !f.contains("12345678901231"));
    }

    #[test]
    fn check_digits_are_correct_again() {
        let out = run("^XA^BEN,50^FD5901234123457^FS^B8^FD96385074^FS^XZ");
        for f in fields(&out) {
            let d = f.as_bytes();
            assert_eq!(gs1_check_digit(&d[..d.len() - 1]), d[d.len() - 1] - b'0', "{f}");
        }
        assert!(!out.contains("5901234123457"));
    }

    #[test]
    fn graphics_stay_and_are_counted() {
        let zpl = "^XA^FO0,0^GFA,8,8,1,FFAA5500FFAA5500^FS~DGR:LOGO.GRF,2,1,FF00^XZ";
        let a = anonymize(zpl, &KEY);
        assert_eq!(a.zpl, zpl);
        assert_eq!(a.graphics, 2);
    }

    #[test]
    fn clock_and_data_matrix_escapes_stay() {
        let out = run("^XA^FC%^FDPrinted %Y-%m-%d by Ann^FS^BXN,5,200^FD~1ABC~d065^FS^XZ");
        let f = fields(&out);
        assert!(f[0].contains(" %Y-%m-%d ") && !f[0].contains("Ann"));
        assert!(f[1].starts_with("~1") && f[1].ends_with("~d065") && !f[1].contains("ABC"));
    }

    #[test]
    fn other_scripts_stay_in_their_script() {
        let out = run("^XA^CI28^FDИван Ελένη^FS^XZ");
        let f = fields(&out)[0];
        assert!(f.chars().take(4).all(|c| ('\u{400}'..'\u{480}').contains(&c)), "{f}");
        assert!(f.chars().nth(0).unwrap().is_uppercase() && f.chars().nth(1).unwrap().is_lowercase());
        assert!(!f.contains("Иван"));
    }

    #[test]
    fn changed_prefixes() {
        let out = run("^XA^CC#\n#FO1,1#FDSecret#FS#XZ");
        assert!(out.starts_with("^XA^CC#\n#FO1,1#FD") && !out.contains("Secret"));
    }

    #[test]
    fn empty_and_unfinished_fields() {
        assert_eq!(run("^XA^FD^FS^FD"), "^XA^FD^FS^FD");
        assert_eq!(run(""), "");
        assert_eq!(run("^XA^BQN^FDQA^FS^BQ^FDD0^FS^FH^FD_4^FS^XZ").len(), "^XA^BQN^FDQA^FS^BQ^FDD0^FS^FH^FD_4^FS^XZ".len());
    }

    #[test]
    fn rfid_hex_stays_hex() {
        let out = run("^XA^RFW,H^FD3005FB63AC1F3681EC880468^FS^XZ");
        let f = fields(&out)[0];
        assert!(f.chars().all(|c| c.is_ascii_hexdigit()) && f != "3005FB63AC1F3681EC880468");
    }
}
