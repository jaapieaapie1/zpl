//! 1D (linear) barcode encoders.
//!
//! Each encoder turns ZPL `^FD` data into a run of alternating bar/space [`Element`]s plus the
//! interpretation-line layout. Widths are abstract: module counts for module-based symbologies
//! (Code 93, EAN/UPC), or narrow/wide flags for ratio-based ones, which the renderer scales with
//! the `^BY` module width and wide-bar ratio (see [`LinearSymbol::element_dots`]).

/// A supported linear symbology and its encoding options.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Symbology {
    /// `^B3`; optional Mod 43 check character.
    Code39 { check_digit: bool },
    /// `^BA`; check characters C and K are always encoded, `check_digit` only shows them in the text.
    Code93 { check_digit: bool },
    /// `^B1`; 1 (C) or 2 (C and K) check digits, 0 for none.
    Code11 { check_digits: u8 },
    /// `^B2`; optional Mod 10 check digit, odd lengths get a leading zero.
    Interleaved2of5 { check_digit: bool },
    /// `^BJ`
    Standard2of5,
    /// `^BI`
    Industrial2of5,
    /// `^BK`; start/stop characters taken from the data when present, else `A`/`A`.
    /// Use [`encode_codabar`] to pass them explicitly.
    Codabar { check_digit: bool },
    /// `^BM`
    Msi { check: MsiCheck },
    /// `^BP`; the CRC is always encoded, `check_digit` only shows it in the text.
    Plessey { check_digit: bool },
    /// `^BE`
    Ean13,
    /// `^B8`
    Ean8,
    /// `^BU`
    UpcA,
    /// `^B9`
    UpcE,
    /// `^BS`; 2- or 5-digit UPC/EAN add-on.
    UpcEanExtension,
}

/// MSI check digit scheme (`^BM` parameter `e`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MsiCheck {
    None,
    Mod10,
    Mod10Mod10,
    Mod11Mod10,
}

/// One bar or space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Element {
    /// `true` = black bar, `false` = space.
    pub bar: bool,
    /// Module count (module-based) or 1 = narrow / 2 = wide (ratio-based).
    pub width: u8,
    /// EAN/UPC guard bar that extends below the normal bars into the text area.
    pub guard: bool,
}

/// A piece of interpretation-line text, centred in `start..end`.
#[derive(Debug, Clone, PartialEq)]
pub struct TextSegment {
    pub text: String,
    /// Horizontal extent the text is centred in, in module units from the symbol's left edge
    /// (start of first element). May be negative / exceed the symbol width (e.g. EAN-13 leading
    /// digit sits left of the symbol, UPC-A first/last digits sit outside). For ratio-based
    /// symbols units are nominal (wide = 2); use [`LinearSymbol::text_extent_dots`] to scale.
    pub start: f32,
    pub end: f32,
}

/// An encoded linear symbol.
#[derive(Debug, Clone, PartialEq)]
pub struct LinearSymbol {
    /// Alternating bar/space elements, starting with a bar.
    pub elements: Vec<Element>,
    /// `true`: width 2 = wide (scaled by the `^BY` ratio); `false`: width = module count.
    pub ratio_based: bool,
    /// Interpretation line layout; a single segment spanning the symbol for non-EAN/UPC.
    pub text: Vec<TextSegment>,
    /// EAN/UPC style layout (text sits between/under extended guards).
    pub has_guards: bool,
}

/// Why field data could not be encoded.
#[derive(Debug, Clone, PartialEq)]
pub enum EncodeError {
    InvalidCharacter(char),
    InvalidLength(usize),
    Empty,
}

impl LinearSymbol {
    /// Width of one element in dots: wide = round(module * ratio), at least module + 1.
    fn dots(&self, e: &Element, module: u32, ratio: f32) -> u32 {
        if self.ratio_based && e.width >= 2 {
            ((module as f32 * ratio).round() as u32).max(module + 1)
        } else {
            e.width as u32 * module
        }
    }

    /// Total width in dots for the given `^BY` module width and wide ratio.
    pub fn width_dots(&self, module: u32, ratio: f32) -> u32 {
        self.elements
            .iter()
            .map(|e| self.dots(e, module, ratio))
            .sum()
    }

    /// Element widths in dots, same rules as [`width_dots`](Self::width_dots).
    pub fn element_dots(
        &self,
        module: u32,
        ratio: f32,
    ) -> impl Iterator<Item = (Element, u32)> + '_ {
        self.elements
            .iter()
            .map(move |e| (*e, self.dots(e, module, ratio)))
    }

    /// Nominal width in units (sum of element widths, wide counted as 2).
    pub fn units(&self) -> u32 {
        self.elements.iter().map(|e| e.width as u32).sum()
    }

    /// A text segment's extent in dots from the symbol's left edge.
    pub fn text_extent_dots(&self, seg: &TextSegment, module: u32, ratio: f32) -> (f32, f32) {
        let scale = if self.ratio_based {
            self.width_dots(module, ratio) as f32 / self.units().max(1) as f32
        } else {
            module as f32
        };
        (seg.start * scale, seg.end * scale)
    }
}

/// Encode `data` with the given symbology.
pub fn encode(symbology: Symbology, data: &str) -> Result<LinearSymbol, EncodeError> {
    if data.is_empty() {
        return Err(EncodeError::Empty);
    }
    match symbology {
        Symbology::Code39 { check_digit } => code39(data, check_digit),
        Symbology::Code93 { check_digit } => code93(data, check_digit),
        Symbology::Code11 { check_digits } => code11(data, check_digits),
        Symbology::Interleaved2of5 { check_digit } => interleaved_2of5(data, check_digit),
        Symbology::Standard2of5 | Symbology::Industrial2of5 => industrial_2of5(data),
        Symbology::Codabar { check_digit } => {
            let chars: Vec<char> = data.chars().collect();
            let (first, last) = (chars[0], chars[chars.len() - 1]);
            match (codabar_start_stop(first), codabar_start_stop(last)) {
                (Some(_), Some(_)) if chars.len() >= 2 => {
                    let inner: String = chars[1..chars.len() - 1].iter().collect();
                    encode_codabar(&inner, first, last, check_digit)
                }
                _ => encode_codabar(data, 'A', 'A', check_digit),
            }
        }
        Symbology::Msi { check } => encode_msi(data, check, false),
        Symbology::Plessey { check_digit } => plessey(data, check_digit),
        Symbology::Ean13 => ean13(data),
        Symbology::Ean8 => ean8(data),
        Symbology::UpcA => upca(data),
        Symbology::UpcE => upce(data),
        Symbology::UpcEanExtension => extension(data),
    }
}

// ---------------------------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------------------------

/// Accumulates alternating elements; colour follows from the previous element.
#[derive(Default)]
struct Builder {
    elements: Vec<Element>,
}

impl Builder {
    fn push(&mut self, width: u8, guard: bool) {
        let bar = self.elements.last().is_none_or(|e| !e.bar);
        self.elements.push(Element { bar, width, guard });
    }

    fn widths(&mut self, widths: &[u8], guard: bool) {
        widths.iter().for_each(|&w| self.push(w, guard));
    }

    /// Push `n` elements from a wide-flag mask, most significant bit first (1 = wide).
    fn flags(&mut self, mask: u16, n: u32) {
        (0..n)
            .rev()
            .for_each(|i| self.push(if mask >> i & 1 == 1 { 2 } else { 1 }, false));
    }

    /// Push an `n`-module bitmap (1 = bar), most significant bit first, as runs.
    fn modules(&mut self, bits: u16, n: u32) {
        let mut i = n;
        while i > 0 {
            let bit = bits >> (i - 1) & 1;
            let mut run = 0;
            while i > 0 && bits >> (i - 1) & 1 == bit {
                run += 1;
                i -= 1;
            }
            self.push(run, false);
        }
    }

    /// Finish a ratio/plain symbol with one text segment spanning it.
    fn plain(self, text: String, ratio_based: bool) -> LinearSymbol {
        let end = self.elements.iter().map(|e| e.width as f32).sum();
        LinearSymbol {
            elements: self.elements,
            ratio_based,
            text: vec![TextSegment {
                text,
                start: 0.0,
                end,
            }],
            has_guards: false,
        }
    }
}

/// Parse a string of decimal digits.
fn digits(data: &str) -> Result<Vec<u8>, EncodeError> {
    data.chars()
        .map(|c| {
            c.to_digit(10)
                .map(|d| d as u8)
                .ok_or(EncodeError::InvalidCharacter(c))
        })
        .collect()
}

fn digit_string(d: &[u8]) -> String {
    d.iter().map(|&x| (b'0' + x) as char).collect()
}

/// Mod 10 check digit with weights 3,1,3,... from the right (EAN/UPC, Interleaved 2 of 5).
fn mod10_weighted(d: &[u8]) -> u8 {
    let sum: u32 = d
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &x)| x as u32 * if i % 2 == 0 { 3 } else { 1 })
        .sum();
    ((10 - sum % 10) % 10) as u8
}

// ---------------------------------------------------------------------------------------------
// Code 39
// ---------------------------------------------------------------------------------------------

const CODE39_CHARS: &[u8; 44] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ-. $/+%*";
/// 9 elements (bar first), 1 = wide.
const CODE39_PATTERNS: [u16; 44] = [
    0x034, 0x121, 0x061, 0x160, 0x031, 0x130, 0x070, 0x025, 0x124, 0x064, // 0-9
    0x109, 0x049, 0x148, 0x019, 0x118, 0x058, 0x00D, 0x10C, 0x04C, 0x01C, // A-J
    0x103, 0x043, 0x142, 0x013, 0x112, 0x052, 0x007, 0x106, 0x046, 0x016, // K-T
    0x181, 0x0C1, 0x1C0, 0x091, 0x190, 0x0D0, 0x085, 0x184, 0x0C4, 0x0A8, // U-Z - . space $
    0x0A2, 0x08A, 0x02A, 0x094, // / + % *
];

fn code39(data: &str, check_digit: bool) -> Result<LinearSymbol, EncodeError> {
    let mut values = data
        .chars()
        .map(|c| {
            let u = c.to_ascii_uppercase();
            CODE39_CHARS[..43]
                .iter()
                .position(|&x| x as char == u)
                .ok_or(EncodeError::InvalidCharacter(c))
        })
        .collect::<Result<Vec<_>, _>>()?;
    if check_digit {
        values.push(values.iter().sum::<usize>() % 43);
    }
    let mut b = Builder::default();
    let all = std::iter::once(43)
        .chain(values.iter().copied())
        .chain(std::iter::once(43));
    for (i, v) in all.enumerate() {
        if i > 0 {
            b.push(1, false); // inter-character gap
        }
        b.flags(CODE39_PATTERNS[v], 9);
    }
    let text = values
        .iter()
        .map(|&v| CODE39_CHARS[v] as char)
        .collect::<String>();
    Ok(b.plain(format!("*{text}*"), true))
}

// ---------------------------------------------------------------------------------------------
// Code 93
// ---------------------------------------------------------------------------------------------

/// Values 43..=46 are the shift characters ($) (%) (/) (+), entered in ZPL as & ' ( ).
const CODE93_CHARS: &[u8; 47] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ-. $/+%&'()";
/// 9-module bitmaps (1 = bar).
const CODE93_PATTERNS: [u16; 47] = [
    0x114, 0x148, 0x144, 0x142, 0x128, 0x124, 0x122, 0x150, 0x112, 0x10A, // 0-9
    0x1A8, 0x1A4, 0x1A2, 0x194, 0x192, 0x18A, 0x168, 0x164, 0x162, 0x134, // A-J
    0x11A, 0x158, 0x14C, 0x146, 0x12C, 0x116, 0x1B4, 0x1B2, 0x1AC, 0x1A6, // K-T
    0x196, 0x19A, 0x16C, 0x166, 0x136, 0x13A, // U-Z
    0x12E, 0x1D4, 0x1D2, 0x1CA, 0x16E, 0x176, 0x1AE, // - . space $ / + %
    0x126, 0x1DA, 0x1D6, 0x132, // ($) (%) (/) (+)
];
const CODE93_START_STOP: u16 = 0x15E;
const SHIFT_DOLLAR: usize = 43;
const SHIFT_PERCENT: usize = 44;
const SHIFT_SLASH: usize = 45;
const SHIFT_PLUS: usize = 46;

/// Full-ASCII shift pair (shift value, character) for a character outside the base set.
fn code93_full_ascii(c: u8) -> Option<(usize, u8)> {
    Some(match c {
        0 => (SHIFT_PERCENT, b'U'),
        1..=26 => (SHIFT_DOLLAR, b'A' + c - 1),
        27..=31 => (SHIFT_PERCENT, b'A' + c - 27),
        b'!'..=b',' => (SHIFT_SLASH, b'A' + c - b'!'),
        b':' => (SHIFT_SLASH, b'Z'),
        b';'..=b'?' => (SHIFT_PERCENT, b'F' + c - b';'),
        b'@' => (SHIFT_PERCENT, b'V'),
        b'['..=b'_' => (SHIFT_PERCENT, b'K' + c - b'['),
        b'`' => (SHIFT_PERCENT, b'W'),
        b'a'..=b'z' => (SHIFT_PLUS, c - 32),
        b'{'..=127 => (SHIFT_PERCENT, b'P' + c - b'{'),
        _ => return None,
    })
}

fn code93_check(values: &[usize], max_weight: usize) -> usize {
    values
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &v)| (i % max_weight + 1) * v)
        .sum::<usize>()
        % 47
}

fn code93(data: &str, check_digit: bool) -> Result<LinearSymbol, EncodeError> {
    let pos = |b: u8| CODE93_CHARS.iter().position(|&x| x == b);
    let mut values = Vec::new();
    for c in data.chars() {
        let b = u8::try_from(c).map_err(|_| EncodeError::InvalidCharacter(c))?;
        match pos(b) {
            Some(v) => values.push(v),
            None => {
                let (shift, ch) = code93_full_ascii(b).ok_or(EncodeError::InvalidCharacter(c))?;
                values.extend([shift, pos(ch).expect("shifted char in base set")]);
            }
        }
    }
    let c = code93_check(&values, 20);
    values.push(c);
    let k = code93_check(&values, 15);
    values.push(k);

    let mut b = Builder::default();
    b.modules(CODE93_START_STOP, 9);
    values
        .iter()
        .for_each(|&v| b.modules(CODE93_PATTERNS[v], 9));
    b.modules(CODE93_START_STOP, 9);
    b.push(1, false); // termination bar
    let mut text = data.to_string();
    if check_digit {
        text.extend([CODE93_CHARS[c] as char, CODE93_CHARS[k] as char]);
    }
    Ok(b.plain(text, false))
}

// ---------------------------------------------------------------------------------------------
// Code 11
// ---------------------------------------------------------------------------------------------

/// 5 elements (bar first), 1 = wide; index 10 = '-', 11 = start/stop.
const CODE11_PATTERNS: [u16; 12] = [
    0b00001, 0b10001, 0b01001, 0b11000, 0b00101, 0b10100, 0b01100, 0b00011, 0b10010, 0b10000,
    0b00100, 0b00110,
];

fn code11_check(values: &[u8], max_weight: usize) -> u8 {
    (values
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &v)| (i % max_weight + 1) * v as usize)
        .sum::<usize>()
        % 11) as u8
}

fn code11(data: &str, check_digits: u8) -> Result<LinearSymbol, EncodeError> {
    let mut values = data
        .chars()
        .map(|c| match c {
            '-' => Ok(10),
            _ => c
                .to_digit(10)
                .map(|d| d as u8)
                .ok_or(EncodeError::InvalidCharacter(c)),
        })
        .collect::<Result<Vec<u8>, _>>()?;
    if check_digits >= 1 {
        values.push(code11_check(&values, 10));
    }
    if check_digits >= 2 {
        values.push(code11_check(&values, 9));
    }
    let mut b = Builder::default();
    let all = std::iter::once(11)
        .chain(values.iter().copied())
        .chain(std::iter::once(11));
    for (i, v) in all.enumerate() {
        if i > 0 {
            b.push(1, false);
        }
        b.flags(CODE11_PATTERNS[v as usize], 5);
    }
    let text = values
        .iter()
        .map(|&v| if v == 10 { '-' } else { (b'0' + v) as char })
        .collect();
    Ok(b.plain(text, true))
}

// ---------------------------------------------------------------------------------------------
// 2 of 5 family
// ---------------------------------------------------------------------------------------------

/// 5 wide flags per digit, first element first.
const TWO_OF_FIVE: [u16; 10] = [
    0b00110, 0b10001, 0b01001, 0b11000, 0b00101, 0b10100, 0b01100, 0b00011, 0b10010, 0b01010,
];

fn wide(mask: u16, i: u32) -> u8 {
    if mask >> (4 - i) & 1 == 1 { 2 } else { 1 }
}

fn interleaved_2of5(data: &str, check_digit: bool) -> Result<LinearSymbol, EncodeError> {
    let mut d = digits(data)?;
    if check_digit {
        d.push(mod10_weighted(&d));
    }
    if d.len() % 2 == 1 {
        d.insert(0, 0);
    }
    let mut b = Builder::default();
    b.widths(&[1, 1, 1, 1], false);
    for pair in d.chunks(2) {
        let (bars, spaces) = (TWO_OF_FIVE[pair[0] as usize], TWO_OF_FIVE[pair[1] as usize]);
        (0..5).for_each(|i| b.widths(&[wide(bars, i), wide(spaces, i)], false));
    }
    b.widths(&[2, 1, 1], false);
    Ok(b.plain(digit_string(&d), true))
}

/// Industrial / Standard 2 of 5: data in the bars only, every bar followed by a narrow space.
fn industrial_2of5(data: &str) -> Result<LinearSymbol, EncodeError> {
    let d = digits(data)?;
    let mut b = Builder::default();
    b.widths(&[2, 1, 2, 1, 1, 1], false);
    for &x in &d {
        (0..5).for_each(|i| b.widths(&[wide(TWO_OF_FIVE[x as usize], i), 1], false));
    }
    b.widths(&[2, 1, 1, 1, 2], false);
    Ok(b.plain(digit_string(&d), true))
}

// ---------------------------------------------------------------------------------------------
// Codabar
// ---------------------------------------------------------------------------------------------

const CODABAR_CHARS: &[u8; 20] = b"0123456789-$:/.+ABCD";
/// 7 elements (bar first), 1 = wide.
const CODABAR_PATTERNS: [u16; 20] = [
    0x003, 0x006, 0x009, 0x060, 0x012, 0x042, 0x021, 0x024, 0x030, 0x048, // 0-9
    0x00C, 0x018, 0x045, 0x051, 0x054, 0x015, 0x01A, 0x029, 0x00B,
    0x00E, // - $ : / . + A B C D
];

/// Normalise a Codabar start/stop character (A-D, or aliases T N * E).
fn codabar_start_stop(c: char) -> Option<char> {
    match c.to_ascii_uppercase() {
        c @ 'A'..='D' => Some(c),
        'T' => Some('A'),
        'N' => Some('B'),
        '*' => Some('C'),
        'E' => Some('D'),
        _ => None,
    }
}

/// Encode Codabar (`^BK`) with explicit start/stop characters (A-D or aliases T N * E).
/// The optional check character is Mod 16 over all characters including start/stop.
pub fn encode_codabar(
    data: &str,
    start: char,
    stop: char,
    check_digit: bool,
) -> Result<LinearSymbol, EncodeError> {
    if data.is_empty() {
        return Err(EncodeError::Empty);
    }
    let pos = |c: char| CODABAR_CHARS.iter().position(|&x| x as char == c);
    let start = codabar_start_stop(start).ok_or(EncodeError::InvalidCharacter(start))?;
    let stop = codabar_start_stop(stop).ok_or(EncodeError::InvalidCharacter(stop))?;
    let mut values = vec![pos(start).unwrap()];
    for c in data.chars() {
        match pos(c) {
            Some(v) if v < 16 => values.push(v),
            _ => return Err(EncodeError::InvalidCharacter(c)),
        }
    }
    values.push(pos(stop).unwrap());
    if check_digit {
        let sum: usize = values.iter().sum();
        values.insert(values.len() - 1, (16 - sum % 16) % 16);
    }
    let mut b = Builder::default();
    for (i, &v) in values.iter().enumerate() {
        if i > 0 {
            b.push(1, false);
        }
        b.flags(CODABAR_PATTERNS[v], 7);
    }
    let text = values.iter().map(|&v| CODABAR_CHARS[v] as char).collect();
    Ok(b.plain(text, true))
}

// ---------------------------------------------------------------------------------------------
// MSI / Plessey (bit 1 = wide bar + narrow space, bit 0 = narrow bar + wide space)
// ---------------------------------------------------------------------------------------------

fn push_bit(b: &mut Builder, bit: bool) {
    b.widths(if bit { &[2, 1] } else { &[1, 2] }, false);
}

/// MSI Mod 10 (Luhn, rightmost digit doubled).
fn msi_mod10(d: &[u8]) -> u8 {
    let sum: u32 = d
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &x)| {
            if i % 2 == 0 {
                let y = x as u32 * 2;
                y / 10 + y % 10
            } else {
                x as u32
            }
        })
        .sum();
    ((10 - sum % 10) % 10) as u8
}

/// MSI Mod 11 (IBM weights 2..7 from the right); a result of 10 is encoded as "10".
fn msi_mod11(d: &[u8]) -> Vec<u8> {
    let sum: u32 = d
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &x)| x as u32 * (i as u32 % 6 + 2))
        .sum();
    match (11 - sum % 11) % 11 {
        10 => vec![1, 0],
        c => vec![c as u8],
    }
}

/// Encode MSI (`^BM`); `show_check` puts the check digits in the interpretation line (`e2`).
pub fn encode_msi(
    data: &str,
    check: MsiCheck,
    show_check: bool,
) -> Result<LinearSymbol, EncodeError> {
    if data.is_empty() {
        return Err(EncodeError::Empty);
    }
    let mut d = digits(data)?;
    let n = d.len();
    match check {
        MsiCheck::None => {}
        MsiCheck::Mod10 => d.push(msi_mod10(&d)),
        MsiCheck::Mod10Mod10 => {
            d.push(msi_mod10(&d));
            d.push(msi_mod10(&d));
        }
        MsiCheck::Mod11Mod10 => {
            d.extend(msi_mod11(&d));
            d.push(msi_mod10(&d));
        }
    }
    let mut b = Builder::default();
    b.widths(&[2, 1], false);
    for &x in &d {
        (0..4).rev().for_each(|i| push_bit(&mut b, x >> i & 1 == 1));
    }
    b.widths(&[1, 2, 1], false);
    Ok(b.plain(digit_string(if show_check { &d } else { &d[..n] }), true))
}

/// Plessey CRC generator polynomial x^8+x^7+x^6+x^5+x^3+1, highest term first.
const PLESSEY_POLY: [u8; 9] = [1, 1, 1, 1, 0, 1, 0, 0, 1];

/// The 8 Plessey CRC bits for a sequence of data bits.
fn plessey_crc(bits: &[bool]) -> [bool; 8] {
    let mut work: Vec<u8> = bits.iter().map(|&b| b as u8).chain([0; 8]).collect();
    for i in 0..bits.len() {
        if work[i] == 1 {
            PLESSEY_POLY
                .iter()
                .enumerate()
                .for_each(|(j, &p)| work[i + j] ^= p);
        }
    }
    std::array::from_fn(|i| work[bits.len() + i] == 1)
}

fn plessey(data: &str, check_digit: bool) -> Result<LinearSymbol, EncodeError> {
    let values = data
        .chars()
        .map(|c| {
            c.to_digit(16)
                .map(|d| d as u8)
                .ok_or(EncodeError::InvalidCharacter(c))
        })
        .collect::<Result<Vec<u8>, _>>()?;
    let bits: Vec<bool> = values
        .iter()
        .flat_map(|&v| (0..4).map(move |i| v >> i & 1 == 1))
        .collect();
    let crc = plessey_crc(&bits);
    let mut b = Builder::default();
    [true, true, false, true]
        .into_iter()
        .chain(bits)
        .chain(crc)
        .for_each(|bit| push_bit(&mut b, bit));
    b.widths(&[2, 2, 1, 2, 1, 1, 2, 1, 2], false); // stop + termination bar
    let mut text = data.to_ascii_uppercase();
    if check_digit {
        let nibble = |r: &[bool]| r.iter().rev().fold(0u32, |a, &x| a << 1 | x as u32);
        let hex = |v| char::from_digit(v, 16).unwrap().to_ascii_uppercase();
        text.extend([hex(nibble(&crc[..4])), hex(nibble(&crc[4..]))]);
    }
    Ok(b.plain(text, true))
}

// ---------------------------------------------------------------------------------------------
// EAN / UPC
// ---------------------------------------------------------------------------------------------

/// L-code widths (space, bar, space, bar). R-codes use the same widths starting with a bar;
/// G-codes are the L widths reversed.
const EAN_L: [[u8; 4]; 10] = [
    [3, 2, 1, 1],
    [2, 2, 2, 1],
    [2, 1, 2, 2],
    [1, 4, 1, 1],
    [1, 1, 3, 2],
    [1, 2, 3, 1],
    [1, 1, 1, 4],
    [1, 3, 1, 2],
    [1, 2, 1, 3],
    [3, 1, 1, 2],
];
/// EAN-13 left-half parity by leading digit, 6 bits MSB first (1 = G).
const EAN13_PARITY: [u8; 10] = [0x00, 0x0B, 0x0D, 0x0E, 0x13, 0x19, 0x1C, 0x15, 0x16, 0x1A];
/// UPC-E (number system 0) parity by check digit, 6 bits MSB first (1 = G/even).
const UPCE_PARITY: [u8; 10] = [0x38, 0x34, 0x32, 0x31, 0x2C, 0x26, 0x23, 0x2A, 0x29, 0x25];
/// 5-digit add-on parity by checksum, 5 bits MSB first (1 = G).
const EXT5_PARITY: [u8; 10] = [0x18, 0x14, 0x12, 0x11, 0x0C, 0x06, 0x03, 0x0A, 0x09, 0x05];

const GUARD_NORMAL: [u8; 3] = [1, 1, 1];
const GUARD_CENTRE: [u8; 5] = [1, 1, 1, 1, 1];

/// Push one EAN digit; `g` selects the G (even parity) code. L and R share widths.
fn ean_digit(b: &mut Builder, d: u8, g: bool, guard: bool) {
    let mut w = EAN_L[d as usize];
    if g {
        w.reverse();
    }
    b.widths(&w, guard);
}

/// Parse digits and fit to `n` by left zero padding or dropping trailing extras.
fn fit_digits(data: &str, n: usize) -> Result<Vec<u8>, EncodeError> {
    let mut d = digits(data)?;
    d.truncate(n);
    let mut out = vec![0; n - d.len()];
    out.append(&mut d);
    Ok(out)
}

fn seg(text: String, start: f32, end: f32) -> TextSegment {
    TextSegment { text, start, end }
}

fn guarded(b: Builder, text: Vec<TextSegment>) -> LinearSymbol {
    LinearSymbol {
        elements: b.elements,
        ratio_based: false,
        text,
        has_guards: true,
    }
}

/// Bars for a 13-digit EAN-13 / UPC-A (leading 0) string; `upc` extends the outer digits.
fn ean13_bars(d: &[u8], upc: bool) -> Builder {
    let mut b = Builder::default();
    let parity = EAN13_PARITY[d[0] as usize];
    b.widths(&GUARD_NORMAL, true);
    for i in 0..6 {
        ean_digit(&mut b, d[i + 1], parity >> (5 - i) & 1 == 1, upc && i == 0);
    }
    b.widths(&GUARD_CENTRE, true);
    for i in 0..6 {
        ean_digit(&mut b, d[i + 7], false, upc && i == 5);
    }
    b.widths(&GUARD_NORMAL, true);
    b
}

/// `^BE`: 12 data digits (padded/truncated), check digit computed.
fn ean13(data: &str) -> Result<LinearSymbol, EncodeError> {
    let mut d = fit_digits(data, 12)?;
    d.push(mod10_weighted(&d));
    let b = ean13_bars(&d, false);
    Ok(guarded(
        b,
        vec![
            seg(digit_string(&d[..1]), -8.0, -1.0),
            seg(digit_string(&d[1..7]), 3.0, 45.0),
            seg(digit_string(&d[7..]), 50.0, 92.0),
        ],
    ))
}

/// `^BU`: 11 data digits (padded/truncated), check digit computed.
fn upca(data: &str) -> Result<LinearSymbol, EncodeError> {
    let mut d = fit_digits(data, 11)?;
    d.push(mod10_weighted(&d));
    let mut full = vec![0];
    full.extend(&d);
    let b = ean13_bars(&full, true);
    Ok(guarded(
        b,
        vec![
            seg(digit_string(&d[..1]), -8.0, -1.0),
            seg(digit_string(&d[1..6]), 10.0, 45.0),
            seg(digit_string(&d[6..11]), 50.0, 85.0),
            seg(digit_string(&d[11..]), 96.0, 103.0),
        ],
    ))
}

/// `^B8`: 7 data digits (padded/truncated), check digit computed.
fn ean8(data: &str) -> Result<LinearSymbol, EncodeError> {
    let mut d = fit_digits(data, 7)?;
    d.push(mod10_weighted(&d));
    let mut b = Builder::default();
    b.widths(&GUARD_NORMAL, true);
    d[..4]
        .iter()
        .for_each(|&x| ean_digit(&mut b, x, false, false));
    b.widths(&GUARD_CENTRE, true);
    d[4..]
        .iter()
        .for_each(|&x| ean_digit(&mut b, x, false, false));
    b.widths(&GUARD_NORMAL, true);
    Ok(guarded(
        b,
        vec![
            seg(digit_string(&d[..4]), 3.0, 31.0),
            seg(digit_string(&d[4..]), 36.0, 64.0),
        ],
    ))
}

/// Expand a 6-digit UPC-E body to the 10 UPC-A manufacturer + product digits.
fn upce_expand(e: &[u8]) -> [u8; 10] {
    let [d1, d2, d3, d4, d5, d6] = [e[0], e[1], e[2], e[3], e[4], e[5]];
    match d6 {
        0..=2 => [d1, d2, d6, 0, 0, 0, 0, d3, d4, d5],
        3 => [d1, d2, d3, 0, 0, 0, 0, 0, d4, d5],
        4 => [d1, d2, d3, d4, 0, 0, 0, 0, 0, d5],
        _ => [d1, d2, d3, d4, d5, 0, 0, 0, 0, d6],
    }
}

/// Zero-suppress 10 UPC-A manufacturer + product digits to a 6-digit UPC-E body.
fn upce_compress(a: &[u8]) -> Option<[u8; 6]> {
    let (m, p) = (&a[..5], &a[5..]);
    if m[3..] == [0, 0] && m[2] <= 2 && p[..2] == [0, 0] {
        Some([m[0], m[1], p[2], p[3], p[4], m[2]])
    } else if m[3..] == [0, 0] && p[..3] == [0, 0, 0] {
        Some([m[0], m[1], m[2], p[3], p[4], 3])
    } else if m[4] == 0 && p[..4] == [0, 0, 0, 0] {
        Some([m[0], m[1], m[2], m[3], p[4], 4])
    } else if m[4] != 0 && p[..4] == [0, 0, 0, 0] && p[4] >= 5 {
        Some([m[0], m[1], m[2], m[3], m[4], p[4]])
    } else {
        None
    }
}

/// `^B9`. Accepts 6 digits (number system 0), 7/8 digits (number system + 6 [+ check]),
/// 10 digits (ZPL style: manufacturer + product, number system 0, zero-suppressed here) or
/// 11/12 digits (full UPC-A [+ check]). The check digit is always recomputed.
fn upce(data: &str) -> Result<LinearSymbol, EncodeError> {
    let d = digits(data)?;
    let len = d.len();
    let invalid = || EncodeError::InvalidLength(len);
    let (ns, body) = match len {
        6 => (0, [d[0], d[1], d[2], d[3], d[4], d[5]]),
        7 | 8 => (d[0], [d[1], d[2], d[3], d[4], d[5], d[6]]),
        10 => (0, upce_compress(&d).ok_or_else(invalid)?),
        11 | 12 => (d[0], upce_compress(&d[1..11]).ok_or_else(invalid)?),
        _ => return Err(invalid()),
    };
    if ns > 1 {
        return Err(EncodeError::InvalidCharacter((b'0' + ns) as char));
    }
    let mut upca = vec![ns];
    upca.extend(upce_expand(&body));
    let check = mod10_weighted(&upca);
    // Number system 0 parity by check digit (1 = even/G); number system 1 is its inverse.
    // Not EAN-13's table: check digit 0 is EEEOOO, where EAN-13's leading 0 is all odd.
    let parity = if ns == 0 {
        UPCE_PARITY[check as usize]
    } else {
        !UPCE_PARITY[check as usize] & 0x3F
    };
    let mut b = Builder::default();
    b.widths(&GUARD_NORMAL, true);
    for (i, &x) in body.iter().enumerate() {
        ean_digit(&mut b, x, parity >> (5 - i) & 1 == 1, false);
    }
    b.widths(&[1, 1, 1, 1, 1, 1], true);
    Ok(guarded(
        b,
        vec![
            seg(digit_string(&[ns]), -8.0, -1.0),
            seg(digit_string(&body), 3.0, 45.0),
            seg(digit_string(&[check]), 52.0, 59.0),
        ],
    ))
}

/// `^BS`: 2-digit add-on for up to 2 digits, else 5-digit (padded/truncated).
fn extension(data: &str) -> Result<LinearSymbol, EncodeError> {
    let n = if data.chars().count() <= 2 { 2 } else { 5 };
    let d = fit_digits(data, n)?;
    let parity = if n == 2 {
        // Value mod 4: 0 = LL, 1 = LG, 2 = GL, 3 = GG.
        (d[0] * 10 + d[1]) % 4
    } else {
        let sum = 3 * (d[0] + d[2] + d[4]) as u32 + 9 * (d[1] + d[3]) as u32;
        EXT5_PARITY[(sum % 10) as usize]
    };
    let mut b = Builder::default();
    b.widths(&[1, 1, 2], false);
    for (i, &x) in d.iter().enumerate() {
        if i > 0 {
            b.widths(&[1, 1], false);
        }
        ean_digit(&mut b, x, parity >> (n - 1 - i) & 1 == 1, false);
    }
    Ok(b.plain(digit_string(&d), false))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn enc(s: Symbology, d: &str) -> LinearSymbol {
        encode(s, d).unwrap()
    }

    fn text(sym: &LinearSymbol) -> String {
        sym.text.iter().map(|t| t.text.as_str()).collect()
    }

    /// Elements alternate starting with a bar and end with a bar.
    fn well_formed(sym: &LinearSymbol) {
        assert!(sym.elements.first().unwrap().bar && sym.elements.last().unwrap().bar);
        for (i, e) in sym.elements.iter().enumerate() {
            assert_eq!(e.bar, i % 2 == 0, "element {i}");
        }
    }

    /// Render as a module string ("1" bar, "0" space).
    fn modules(sym: &LinearSymbol) -> String {
        sym.elements
            .iter()
            .map(|e| (if e.bar { "1" } else { "0" }).repeat(e.width as usize))
            .collect()
    }

    #[test]
    fn ean13() {
        let s = enc(Symbology::Ean13, "400638133393");
        well_formed(&s);
        assert_eq!(s.units(), 95);
        assert_eq!(text(&s), "4006381333931");
        assert_eq!(s.text.len(), 3);
        // Recomputes a supplied check digit, pads short input.
        assert_eq!(
            text(&enc(Symbology::Ean13, "4006381333939")),
            "4006381333931"
        );
        assert_eq!(text(&enc(Symbology::Ean13, "12345")), "0000000123457");
        // Known module pattern of 5901234123457.
        assert_eq!(
            modules(&enc(Symbology::Ean13, "590123412345")),
            "10100010110100111011001100100110111101001110101010110011011011001000010101110010011101000100101"
        );
        assert!(s.elements[0].guard && !s.elements[3].guard);
    }

    #[test]
    fn ean8() {
        let s = enc(Symbology::Ean8, "5512345");
        well_formed(&s);
        assert_eq!(s.units(), 67);
        assert_eq!(text(&s), "55123457");
    }

    #[test]
    fn upca() {
        let s = enc(Symbology::UpcA, "03600029145");
        well_formed(&s);
        assert_eq!(s.units(), 95);
        assert_eq!(text(&s), "036000291452");
        assert_eq!(s.text.len(), 4);
        // First and last digit bars are extended.
        let guard_modules: usize = s
            .elements
            .iter()
            .filter(|e| e.guard)
            .map(|e| e.width as usize)
            .sum();
        assert_eq!(guard_modules, 3 + 7 + 5 + 7 + 3);
    }

    #[test]
    fn upce() {
        let s = enc(Symbology::UpcE, "0425261");
        well_formed(&s);
        assert_eq!(s.units(), 51);
        assert_eq!(text(&s), "04252614");
        assert_eq!(text(&enc(Symbology::UpcE, "425261")), "04252614");
        // ZPL 10-digit manufacturer + product form.
        assert_eq!(text(&enc(Symbology::UpcE, "4210000526")), "04252614");
        assert_eq!(text(&enc(Symbology::UpcE, "042100005264")), "04252614");
        assert_eq!(
            encode(Symbology::UpcE, "1234512345"),
            Err(EncodeError::InvalidLength(10))
        );
        // 0 123456 5: check 5 -> parity EOOEEO.
        assert_eq!(text(&enc(Symbology::UpcE, "123456")), "01234565");
        assert_eq!(
            modules(&enc(Symbology::UpcE, "123456")),
            "101011001100100110111101001110101110010101111010101"
        );
    }

    #[test]
    fn extension() {
        let s2 = enc(Symbology::UpcEanExtension, "12");
        well_formed(&s2);
        assert_eq!(s2.units(), 20);
        let s5 = enc(Symbology::UpcEanExtension, "52495");
        well_formed(&s5);
        assert_eq!(s5.units(), 47);
        assert_eq!(text(&s5), "52495");
        // 52495: checksum 1 -> GLGLL.
        assert_eq!(
            modules(&s5),
            "10110111001010010011010011101010001011010110001"
        );
        assert_eq!(text(&enc(Symbology::UpcEanExtension, "123")), "00123");
    }

    #[test]
    fn code39() {
        let s = enc(Symbology::Code39 { check_digit: false }, "a");
        well_formed(&s);
        assert_eq!(text(&s), "*A*");
        // 3 characters of 12 units (ratio 2) + 2 gaps.
        assert_eq!(s.units(), 38);
        assert_eq!(s.width_dots(2, 3.0), 3 * (6 * 2 + 3 * 6) + 2 * 2);
        assert_eq!(
            text(&enc(Symbology::Code39 { check_digit: true }, "CODE39")),
            "*CODE39W*"
        );
        assert_eq!(
            encode(Symbology::Code39 { check_digit: false }, "A*"),
            Err(EncodeError::InvalidCharacter('*'))
        );
        // Start character '*' = bWbwBwBwb.
        let w: Vec<u8> = s.elements[..9].iter().map(|e| e.width).collect();
        assert_eq!(w, [1, 2, 1, 1, 2, 1, 2, 1, 1]);
    }

    #[test]
    fn code93() {
        let s = enc(Symbology::Code93 { check_digit: true }, "TEST93");
        well_formed(&s);
        assert_eq!(text(&s), "TEST93+6");
        assert_eq!(s.units(), 9 * (6 + 4) + 1);
        assert_eq!(
            text(&enc(Symbology::Code93 { check_digit: false }, "TEST93")),
            "TEST93"
        );
        // Lowercase uses a (+) shift pair: 2 symbol characters.
        assert_eq!(
            enc(Symbology::Code93 { check_digit: false }, "a").units(),
            9 * (2 + 4) + 1
        );
        assert!(modules(&s).starts_with("101011110"));
        assert!(modules(&s).ends_with("1010111101"));
    }

    #[test]
    fn code11() {
        let s = enc(Symbology::Code11 { check_digits: 2 }, "123-45");
        well_formed(&s);
        assert_eq!(text(&s), "123-4552");
        assert_eq!(
            text(&enc(Symbology::Code11 { check_digits: 1 }, "123-45")),
            "123-455"
        );
    }

    #[test]
    fn interleaved() {
        let s = enc(Symbology::Interleaved2of5 { check_digit: true }, "1234567");
        well_formed(&s);
        assert_eq!(text(&s), "12345670");
        assert_eq!(
            text(&enc(
                Symbology::Interleaved2of5 { check_digit: false },
                "123"
            )),
            "0123"
        );
        // start 4 + 2 pairs * (2 * (3 + 2*2)) + stop 4
        assert_eq!(
            enc(Symbology::Interleaved2of5 { check_digit: false }, "0123").units(),
            4 + 2 * 14 + 4
        );
        assert_eq!(
            encode(Symbology::Interleaved2of5 { check_digit: false }, "12a"),
            Err(EncodeError::InvalidCharacter('a'))
        );
    }

    #[test]
    fn industrial() {
        let s = enc(Symbology::Industrial2of5, "12");
        well_formed(&s);
        assert_eq!(s.units(), 8 + 2 * 12 + 7);
    }

    #[test]
    fn codabar() {
        let s = encode_codabar("37859", 'A', 'B', true).unwrap();
        well_formed(&s);
        assert_eq!(text(&s), "A37859+B");
        assert_eq!(
            text(&enc(Symbology::Codabar { check_digit: false }, "t123n")),
            "A123B"
        );
        assert_eq!(
            text(&enc(Symbology::Codabar { check_digit: false }, "123")),
            "A123A"
        );
        // 'A' = nnWWnWn
        let w: Vec<u8> = s.elements[..7].iter().map(|e| e.width).collect();
        assert_eq!(w, [1, 1, 2, 2, 1, 2, 1]);
    }

    #[test]
    fn msi() {
        assert_eq!(
            text(&encode_msi("1234567", MsiCheck::Mod10, true).unwrap()),
            "12345674"
        );
        assert_eq!(
            text(&enc(
                Symbology::Msi {
                    check: MsiCheck::Mod10
                },
                "1234567"
            )),
            "1234567"
        );
        let s = encode_msi("1234567", MsiCheck::Mod10Mod10, true).unwrap();
        well_formed(&s);
        assert_eq!(text(&s), "123456741");
        // start 3 + 9 digits * 12 + stop 4
        assert_eq!(s.units(), 3 + 9 * 12 + 4);
    }

    #[test]
    fn plessey() {
        let s = enc(Symbology::Plessey { check_digit: false }, "01234ABCD");
        well_formed(&s);
        assert_eq!(text(&s), "01234ABCD");
        let bits: Vec<bool> = [1u8, 2, 3]
            .iter()
            .flat_map(|&v| (0..4).map(move |i| v >> i & 1 == 1))
            .collect();
        let crc = plessey_crc(&bits);
        // Data followed by its CRC leaves a zero remainder.
        let all: Vec<bool> = bits.iter().copied().chain(crc).collect();
        assert_eq!(plessey_crc(&all), [false; 8]);
        assert_eq!(
            enc(Symbology::Plessey { check_digit: true }, "123").text[0]
                .text
                .len(),
            5
        );
    }

    #[test]
    fn dots() {
        let s = enc(Symbology::Interleaved2of5 { check_digit: false }, "00");
        // ratio 2.0 with module 1 -> wide 2; ratio 1.2 with module 1 -> forced to 2.
        assert_eq!(s.width_dots(1, 1.2), s.width_dots(1, 2.0));
        let ean = enc(Symbology::Ean8, "1234567");
        assert_eq!(ean.width_dots(3, 2.5), 67 * 3);
        assert_eq!(ean.text_extent_dots(&ean.text[0], 2, 3.0), (6.0, 62.0));
        assert_eq!(encode(Symbology::Ean13, ""), Err(EncodeError::Empty));
    }
}
