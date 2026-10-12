//! ^BQ QR codes: the ^FD switches and the symbol, shared with pdf-renderer so both draw the
//! same modules.
//!
//! Matches Labelary: automatic input encodes the data as one segment of the narrowest mode
//! that holds all of it (numeric, alphanumeric or byte), the error correction level is then
//! raised as far as the data still fits in the smallest version for the requested level,
//! and the mask is always the lowest ISO penalty one (Labelary ignores the ^BQ mask).

use qrcode::bits::Bits;
use qrcode::canvas::Canvas;
use qrcode::{EcLevel, Version};
use zpl_parser::command::Orientation;

/// Top-left corner of a `size` x `size` module QR symbol of `module` dot modules. The
/// symbol is never turned, but Labelary keeps a 10 dot band above it in its own frame and
/// turns that: ^FO normal moves the symbol down by it, ^FO B (270) right. ^FT puts the bottom
/// edge 3 modules above the origin, whatever the orientation.
pub fn top_left(x: u32, y: u32, size: u32, module: u32, orientation: Orientation, bottom: bool) -> (u32, u32) {
    const BAND: u32 = 10;
    match (bottom, orientation) {
        (true, _) => (x, y.saturating_sub((size + 3) * module)),
        (false, Orientation::Normal) => (x, y + BAND),
        (false, Orientation::Rotated270) => (x + BAND, y),
        (false, _) => (x, y),
    }
}

const LEVELS: [EcLevel; 4] = [EcLevel::L, EcLevel::M, EcLevel::Q, EcLevel::H];

/// Character mode of the data segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Numeric,
    Alphanumeric,
    Byte,
    Kanji,
}

/// The ^FD field data split into its switches.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QrData<'a> {
    pub ec: EcLevel,
    /// Manual input (`M`) names the character mode; automatic input (`A`) does not.
    pub mode: Option<Mode>,
    pub payload: &'a [u8],
}

/// Parses `[D<mixed>,]<ec><input>,[<mode>]<data>`. Data without the switches is encoded
/// as is, at level M.
pub fn parse(data: &str) -> QrData<'_> {
    let bytes = data.as_bytes();
    // Mixed mode (`Diijjxx,`) only splits the data over several symbols; skip it.
    let bytes = match bytes {
        [b'D', rest @ ..] if rest.len() > 6 && rest[6] == b',' => &rest[7..],
        _ => bytes,
    };
    let (ec, input, rest) = match bytes {
        [ec, input, b',', rest @ ..] => (*ec, *input, rest),
        _ => return QrData { ec: EcLevel::M, mode: None, payload: bytes },
    };
    let ec = match ec {
        b'H' => EcLevel::H,
        b'Q' => EcLevel::Q,
        b'L' => EcLevel::L,
        _ => EcLevel::M,
    };
    if input != b'M' {
        return QrData { ec, mode: None, payload: rest };
    }
    let (mode, payload) = match rest {
        [b'N', data @ ..] => (Mode::Numeric, data),
        [b'A', data @ ..] => (Mode::Alphanumeric, data),
        [b'K', data @ ..] => (Mode::Kanji, data),
        // `Bxxxx`: exactly xxxx bytes follow.
        [b'B', count @ ..] if count.len() >= 4 && count[..4].iter().all(u8::is_ascii_digit) => {
            let n = count[..4].iter().fold(0usize, |n, d| n * 10 + (d - b'0') as usize);
            (Mode::Byte, &count[4..(4 + n).min(count.len())])
        }
        data => (Mode::Byte, data),
    };
    QrData { ec, mode: Some(mode), payload }
}

fn alphanumeric(c: &u8) -> bool {
    c.is_ascii_digit() || c.is_ascii_uppercase() || b" $%*+-./:".contains(c)
}

/// The narrowest mode holding all of `data`.
fn single_mode(data: &[u8]) -> Mode {
    if data.iter().all(u8::is_ascii_digit) {
        Mode::Numeric
    } else if data.iter().all(alphanumeric) {
        Mode::Alphanumeric
    } else {
        Mode::Byte
    }
}

/// A finished QR symbol: `size` x `size` modules, row by row.
#[derive(Debug, Clone, PartialEq)]
pub struct QrSymbol {
    pub size: u32,
    modules: Vec<bool>,
}

impl QrSymbol {
    /// Whether the module in column `x`, row `y` is dark.
    pub fn get(&self, x: u32, y: u32) -> bool {
        self.modules[(y * self.size + x) as usize]
    }
}

/// `data` as one `mode` segment in `version`, terminated for `ec`; `None` if it does not
/// fit. The mode must hold every character of `data`.
fn bits(data: &[u8], mode: Mode, version: i16, ec: EcLevel) -> Option<Bits> {
    let mut bits = Bits::new(Version::Normal(version));
    match mode {
        Mode::Numeric => bits.push_numeric_data(data),
        Mode::Alphanumeric => bits.push_alphanumeric_data(data),
        Mode::Byte | Mode::Kanji => bits.push_byte_data(data),
    }
    .ok()?;
    bits.push_terminator(ec).ok()?;
    Some(bits)
}

/// The QR symbol for ^FD `data`, `None` if the data does not fit.
pub fn encode(data: &str) -> Option<QrSymbol> {
    let field = parse(data);
    let mode = field.mode.unwrap_or_else(|| single_mode(field.payload));
    // Manual numeric / alphanumeric input keeps only the characters the mode can hold
    // (Labelary encodes `N0123A45` as 012345); Kanji is not supported and goes as bytes.
    let filtered: Vec<u8>;
    let (mode, payload) = match mode {
        Mode::Numeric => {
            filtered = field.payload.iter().copied().filter(u8::is_ascii_digit).collect();
            (mode, filtered.as_slice())
        }
        Mode::Alphanumeric => {
            filtered = field.payload.iter().copied().filter(alphanumeric).collect();
            (mode, filtered.as_slice())
        }
        Mode::Byte | Mode::Kanji => (Mode::Byte, field.payload),
    };
    let field = QrData { payload, ..field };
    let (version, _) = (1..=40).find_map(|v| bits(field.payload, mode, v, field.ec).map(|b| (v, b)))?;
    // Raise the level while the data still fits in the same version.
    let (ec, bits) = LEVELS
        .iter()
        .rev()
        .take_while(|&&ec| ec != field.ec)
        .chain([&field.ec])
        .find_map(|&ec| bits(field.payload, mode, version, ec).map(|b| (ec, b)))?;

    let (data, ec_data) = qrcode::ec::construct_codewords(&bits.into_bytes(), Version::Normal(version), ec).ok()?;
    let mut canvas = Canvas::new(Version::Normal(version), ec);
    canvas.draw_all_functional_patterns();
    canvas.draw_data(&data, &ec_data);
    let canvas = canvas.apply_best_mask();
    let modules = canvas.into_colors().into_iter().map(|c| c == qrcode::Color::Dark).collect();
    Some(QrSymbol { size: (version * 4 + 17) as u32, modules })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_switches() {
        assert_eq!(parse("HA,hello"), QrData { ec: EcLevel::H, mode: None, payload: b"hello" });
        assert_eq!(parse("hello"), QrData { ec: EcLevel::M, mode: None, payload: b"hello" });
        assert_eq!(parse("LM,N0123"), QrData { ec: EcLevel::L, mode: Some(Mode::Numeric), payload: b"0123" });
        assert_eq!(parse("QM,B0003abcdef"), QrData { ec: EcLevel::Q, mode: Some(Mode::Byte), payload: b"abc" });
        assert_eq!(parse("D03048F,MA,x"), QrData { ec: EcLevel::M, mode: None, payload: b"x" });
    }

    /// Format information: (EC level bits, mask), read around the top-left finder.
    fn format(code: &QrSymbol) -> (u32, u32) {
        let cells =
            [(0, 8), (1, 8), (2, 8), (3, 8), (4, 8), (5, 8), (7, 8), (8, 8), (8, 7), (8, 5), (8, 4), (8, 3), (8, 2), (8, 1), (8, 0)];
        let bits = cells.iter().fold(0u32, |v, &(x, y)| v << 1 | code.get(x, y) as u32) ^ 0b101010000010010;
        (bits >> 13, (bits >> 10) & 7)
    }

    #[test]
    fn level_is_raised_within_the_version() {
        // Labelary: "MAG" asked at Q fits version 1 at H (level bits 10); "ECC LEVEL L" only
        // up to Q (11).
        let mag = encode("QA,MAG").unwrap();
        assert_eq!((mag.size, format(&mag)), (21, (0b10, 5)));
        let l = encode("LA,ECC LEVEL L").unwrap();
        assert_eq!((l.size, format(&l).0), (21, 0b11));
    }

    #[test]
    fn placement_follows_labelary() {
        let at = |o, bottom| top_left(100, 200, 21, 4, o, bottom);
        assert_eq!(at(Orientation::Normal, false), (100, 210));
        assert_eq!(at(Orientation::Rotated90, false), (100, 200));
        assert_eq!(at(Orientation::Rotated180, false), (100, 200));
        assert_eq!(at(Orientation::Rotated270, false), (110, 200));
        assert_eq!(at(Orientation::Rotated90, true), (100, 200 - 96));
    }

    #[test]
    fn manual_input_drops_characters_outside_the_mode() {
        // Labelary encodes only the digits; 12 digits fit version 1 at H.
        let code = encode("MM,N0123456789A12AABCDEF").unwrap();
        assert_eq!(code, encode("HM,N012345678912").unwrap());
        assert_eq!((code.size, format(&code)), (21, (0b10, 3)));
        assert!(encode("MM,Aabc").is_some());
    }
}
