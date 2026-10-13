//! ^BQ QR codes: the ^FD switches and the symbol, shared with pdf-renderer so both draw the
//! same modules.
//!
//! Automatic input encodes the data as one segment of the narrowest mode that holds all of
//! it (numeric, alphanumeric or byte), and the error correction level is then raised as far
//! as the data still fits in the smallest version for the requested level (as Zebra
//! printers do). The mask is the ^BQ `e` parameter, 7 unless given
//! (docs/commands/zpl/BQ.md); Labelary ignores it and picks the lowest ISO penalty mask.
//! The encoding itself is qr-encode's (`single_segment`, `boost_ec`).

use qr_core::{BitMatrix, EcLevel};
use qr_encode::Options;

/// The ^BQ mask when none (or one outside 0-7) is given (docs/commands/zpl/BQ.md).
pub const DEFAULT_MASK: u8 = 7;

/// Top-left corner of a `size` x `size` module QR symbol of `module` dot modules. The
/// symbol is never turned (^FW has no effect, docs/commands/zpl/BQ.md). ^FO puts its
/// top-left corner on the origin, ^FT its bottom-left (docs/commands/zpl/FT.md: the origin
/// of a bar code is its base, at the left edge); right justified (z=1) its right edge.
/// Labelary instead moves an ^FO symbol 10 dots down (right when rotated 270) and keeps an
/// ^FT one 3 modules above the origin.
pub fn top_left(x: u32, y: u32, size: u32, module: u32, bottom: bool, right: bool) -> (u32, u32) {
    let side = size * module;
    let x = if right { x.saturating_sub(side) } else { x };
    let y = if bottom { y.saturating_sub(side) } else { y };
    (x, y)
}

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

/// A finished QR symbol: `size` x `size` modules.
#[derive(Debug, Clone, PartialEq)]
pub struct QrSymbol {
    pub size: u32,
    modules: BitMatrix,
}

impl QrSymbol {
    /// Whether the module in column `x`, row `y` is dark.
    pub fn get(&self, x: u32, y: u32) -> bool {
        self.modules.get(x, y)
    }
}

/// The QR symbol for ^FD `data` with ^BQ mask `mask`, `None` if the data does not fit.
pub fn encode(data: &str, mask: Option<u32>) -> Option<QrSymbol> {
    let field = parse(data);
    // Manual numeric / alphanumeric input keeps only the characters the mode can hold
    // (Labelary encodes `N0123A45` as 012345); Kanji is not supported and goes as bytes.
    // Automatic input is one segment of the narrowest mode (`single_segment`).
    let filtered: Vec<u8>;
    let (mode, payload) = match field.mode {
        None => (None, field.payload),
        Some(Mode::Numeric) => {
            filtered = field.payload.iter().copied().filter(u8::is_ascii_digit).collect();
            (Some(qr_core::Mode::Numeric), filtered.as_slice())
        }
        Some(Mode::Alphanumeric) => {
            filtered = field.payload.iter().copied().filter(alphanumeric).collect();
            (Some(qr_core::Mode::Alphanumeric), filtered.as_slice())
        }
        Some(Mode::Byte | Mode::Kanji) => (Some(qr_core::Mode::Byte), field.payload),
    };
    let mask = mask.filter(|&m| m <= 7).map_or(DEFAULT_MASK, |m| m as u8);
    let options = Options { mode, single_segment: true, boost_ec: true, mask: Some(mask), ..Options::new(field.ec) };
    let modules = qr_encode::encode(payload, options).ok()?;
    Some(QrSymbol { size: modules.size(), modules })
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
        let mag = encode("QA,MAG", None).unwrap();
        assert_eq!((mag.size, format(&mag).0), (21, 0b10));
        let l = encode("LA,ECC LEVEL L", None).unwrap();
        assert_eq!((l.size, format(&l).0), (21, 0b11));
    }

    #[test]
    fn mask_is_the_bq_parameter() {
        // docs/commands/zpl/BQ.md: e = mask value 0-7, default 7.
        assert_eq!(format(&encode("QA,MAG", None).unwrap()).1, 7);
        assert_eq!(format(&encode("QA,MAG", Some(2)).unwrap()).1, 2);
        assert_eq!(format(&encode("QA,MAG", Some(9)).unwrap()).1, 7);
    }

    #[test]
    fn placement_is_the_field_origin() {
        // ^FO: top-left corner on the origin (FO.md); ^FT: the base at the origin (FT.md).
        assert_eq!(top_left(100, 200, 21, 4, false, false), (100, 200));
        assert_eq!(top_left(100, 200, 21, 4, true, false), (100, 200 - 84));
        // Right justified: the right edge on x.
        assert_eq!(top_left(400, 200, 21, 4, true, true), (400 - 84, 200 - 84));
    }

    #[test]
    fn manual_input_drops_characters_outside_the_mode() {
        // Labelary encodes only the digits; 12 digits fit version 1 at H.
        let code = encode("MM,N0123456789A12AABCDEF", None).unwrap();
        assert_eq!(code, encode("HM,N012345678912", None).unwrap());
        assert_eq!((code.size, format(&code).0), (21, 0b10));
        assert!(encode("MM,Aabc", None).is_some());
    }

    /// Every ^FD of the qr_* compat cases decodes to its payload. Where the symbol differs
    /// from Labelary's it is only by mask (see the module docs).
    #[test]
    fn compat_symbols_decode() {
        let cases: [(&str, &[u8]); 9] = [
            ("HA,ECC LEVEL H", b"ECC LEVEL H"),
            ("MA,Bbinary data", b"Bbinary data"),
            ("MM,N0123456789A12AABCDEF", b"012345678912"),
            ("QA,FT MAG2", b"FT MAG2"),
            ("QA,FT B", b"FT B"),
            ("QA,TYPESET", b"TYPESET"),
            ("QA,DEFAULT MAG", b"DEFAULT MAG"),
            ("QA,lower case text", b"lower case text"),
            ("QA,A LONGER QR CODE AT VERSION THREE", b"A LONGER QR CODE AT VERSION THREE"),
        ];
        for (data, payload) in cases {
            let decoded = qr_decode::decode(&encode(data, None).unwrap().modules).unwrap();
            assert_eq!(decoded.bytes, payload, "{data}");
        }
    }
}
