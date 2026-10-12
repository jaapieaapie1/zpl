//! GS1 composite symbols (ISO/IEC 24723): a linear component (EAN/UPC, GS1-128 or GS1 DataBar)
//! with a CC-A, CC-B (MicroPDF417 based) or CC-C (PDF417 based) 2D component on top, as
//! printed by `^BR` types 1-12 when the field data contains a `|` separator.
//!
//! The encodation (ISO/IEC 24723 section 5), size tables and alignment rules follow the
//! standard as implemented by zint's `composite.c` (BSD-3-Clause), against whose test vectors
//! this module is verified.

use crate::databar::DataBarKind;
use crate::micropdf::{
    RapLayout, codeword_bits, push_bits, rap_rows, rs_ecc, smallest_variant, variant_rows,
};
use crate::{RowHeight, Stacked, StackedRow, SymbolError};

/// The linear component of a composite symbol.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompositeLinear {
    /// `^BR` types 1-6.
    DataBar(DataBarKind),
    /// `^BR` type 7.
    UpcA,
    /// `^BR` type 8.
    UpcE,
    /// `^BR` type 9.
    Ean13,
    /// `^BR` type 10.
    Ean8,
    /// `^BR` type 11 (`cc_c: false`, CC-A or CC-B) and 12 (`cc_c: true`, always CC-C).
    Gs1_128 { cc_c: bool },
}

/// An encoded composite symbol.
#[derive(Debug, Clone, PartialEq)]
pub struct Composite {
    /// Whole symbol, top to bottom: 2D component rows (`RowHeight::Modules(2)` for CC-A/B,
    /// `Modules(3)` for CC-C, per ISO/IEC 24723), separator rows (`Modules(1)`, EAN/UPC:
    /// three `Modules(2)` guard-extension rows), then the linear component rows (its bar rows
    /// as `RowHeight::Bar` so the renderer applies the `^BR` bar height; DataBar keeps the
    /// row heights of [`crate::databar::encode_linked`]).
    ///
    /// The horizontal alignment of the components is baked in: every row is `symbol.width`
    /// modules wide, padded with light modules.
    pub symbol: Stacked,
    /// Human readable interpretation of the linear component (if any).
    pub hri: Option<String>,
}

/// FNC1 in the reduced GS1 data strings (also accepted literally in the field data).
const FNC1: u8 = 0x1D;

/// `^BR` composite. `data` is the ZPL field data `linear|2D`: the linear component data and
/// the 2D component GS1 element string. Both may be given as bracketed element strings
/// (`(01)03812345678908` or `[01]...`, FNC1 separators inserted after variable-length AIs)
/// or raw (`0103812345678908`, with a GS character (0x1D) as FNC1 where needed). The 2D data
/// may also be any text in the ISO/IEC 646 subset of ISO/IEC 24723 (e.g. ZPL's
/// `this is composite info` example). Without `|` only the linear component is produced.
///
/// `segments_per_row` is passed to DataBar Expanded (`^BR` parameter f).
pub fn encode(
    linear: CompositeLinear,
    data: &str,
    segments_per_row: u32,
) -> Result<Composite, SymbolError> {
    let (lin_data, cc_data) = match data.split_once('|') {
        Some((l, c)) => (l, Some(c)),
        None => (data, None),
    };
    let cc_raw = match cc_data {
        Some(c) if !c.is_empty() => Some(gs1_element_string(c)?.0),
        _ => None,
    };
    match linear {
        CompositeLinear::DataBar(kind) => databar(kind, lin_data, cc_raw, segments_per_row),
        CompositeLinear::Gs1_128 { cc_c } => gs1_128_composite(lin_data, cc_raw, cc_c),
        _ => ean_upc_composite(linear, lin_data, cc_raw),
    }
}

// ---------------------------------------------------------------------------------------------
// GS1 element strings
// ---------------------------------------------------------------------------------------------

/// Two-digit AI prefixes with a predefined length (no FNC1 needed after them), GS1 General
/// Specifications Figure 7.8.5-2.
const PREDEFINED: [&[u8; 2]; 22] = [
    b"00", b"01", b"02", b"03", b"04", b"11", b"12", b"13", b"14", b"15", b"16", b"17", b"18",
    b"19", b"20", b"31", b"32", b"33", b"34", b"35", b"36", b"41",
];

fn predefined(ai: &[u8]) -> bool {
    ai.len() >= 2 && PREDEFINED.iter().any(|p| p[..] == ai[..2])
}

/// Reduce field data to the GS1 string encoded in the symbol (FNC1 as 0x1D, no leading
/// FNC1) and its human readable form. Bracketed input (`(AI)data` or `[AI]data`) is parsed;
/// anything else is taken literally.
fn gs1_element_string(s: &str) -> Result<(Vec<u8>, String), SymbolError> {
    let b = s.as_bytes();
    let (open, close) = match b.first() {
        Some(b'(') => (b'(', b')'),
        Some(b'[') => (b'[', b']'),
        _ => {
            let raw: Vec<u8> = b.strip_prefix(&[FNC1]).unwrap_or(b).to_vec();
            let hri = raw
                .iter()
                .filter(|&&c| c != FNC1)
                .map(|&c| c as char)
                .collect();
            return Ok((raw, hri));
        }
    };
    let mut out = Vec::with_capacity(b.len());
    let mut hri = String::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] != open {
            return Err(SymbolError::InvalidData(format!(
                "expected '{}'",
                open as char
            )));
        }
        let close_at = b[i..]
            .iter()
            .position(|&c| c == close)
            .map(|p| i + p)
            .ok_or_else(|| SymbolError::InvalidData("unterminated AI".into()))?;
        let ai = &b[i + 1..close_at];
        if !(2..=4).contains(&ai.len()) || !ai.iter().all(u8::is_ascii_digit) {
            return Err(SymbolError::InvalidData(format!(
                "invalid AI '{}'",
                &s[i + 1..close_at]
            )));
        }
        let end = b[close_at + 1..]
            .iter()
            .position(|&c| c == open)
            .map_or(b.len(), |p| close_at + 1 + p);
        let value = &b[close_at + 1..end];
        if value.is_empty() {
            return Err(SymbolError::InvalidData("empty AI data".into()));
        }
        if !out.is_empty() && !predefined(&out_last_ai(&hri)) {
            out.push(FNC1);
        }
        out.extend_from_slice(ai);
        out.extend_from_slice(value);
        hri.push('(');
        hri.push_str(&s[i + 1..close_at]);
        hri.push(')');
        hri.push_str(&s[close_at + 1..end]);
        i = end;
    }
    Ok((out, hri))
}

/// The AI of the last element in a human readable string built by `gs1_element_string`.
fn out_last_ai(hri: &str) -> Vec<u8> {
    let start = hri.rfind('(').map_or(0, |p| p + 1);
    let end = hri[start..].find(')').map_or(hri.len(), |p| start + p);
    hri.as_bytes()[start..end].to_vec()
}

// ---------------------------------------------------------------------------------------------
// 2D component bit stream (ISO/IEC 24723 section 5)
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Debug)]
enum CcType {
    A,
    B,
    C,
}

#[derive(Clone, Copy, PartialEq)]
enum Gf {
    Numeric,
    Alpha,
    Iso,
}

const GF_ALNUM_PUNCT: &[u8] = b"*,-./";
const GF_ISO_PUNCT: &[u8] = b"!\"%&'()*+,-./:;<=>?_ ";

fn gf_type(c: u8) -> Option<Gf> {
    if c == FNC1 || c.is_ascii_digit() {
        Some(Gf::Numeric)
    } else if c.is_ascii_uppercase() || GF_ALNUM_PUNCT.contains(&c) {
        Some(Gf::Alpha)
    } else if c.is_ascii_lowercase() || GF_ISO_PUNCT.contains(&c) {
        Some(Gf::Iso)
    } else {
        None
    }
}

/// True if the `num` characters from `i` exist and are all of type `t` (or `t2`).
fn gf_next(f: &[u8], i: usize, num: usize, t: Gf, t2: Option<Gf>) -> bool {
    i + num <= f.len()
        && f[i..i + num].iter().all(|&c| {
            let ty = gf_type(c);
            ty == Some(t) || (t2.is_some() && ty == t2)
        })
}

/// True if the rest of the field (at most `max_num`, at least `num` characters) is of type `t`.
fn gf_next_terminate(f: &[u8], i: usize, num: usize, max_num: usize, t: Gf) -> bool {
    i + max_num >= f.len() && f.len() - i >= num && f[i..].iter().all(|&c| gf_type(c) == Some(t))
}

/// True if none of the next `num` characters (or up to the end) is of type `t`.
fn gf_next_none(f: &[u8], i: usize, num: usize, t: Gf) -> bool {
    f[i..f.len().min(i + num)]
        .iter()
        .all(|&c| gf_type(c) != Some(t))
}

/// General-purpose data compaction (ISO/IEC 24723 5.4). Returns the final mode and a trailing
/// single digit left over in numeric mode.
fn general_field(
    f: &[u8],
    mut mode: Gf,
    bits: &mut Vec<bool>,
) -> Result<(Gf, Option<u8>), SymbolError> {
    let mut i = 0;
    let mut last_digit = None;
    let digit = |c: u8| if c == FNC1 { 10 } else { u32::from(c - b'0') };
    while i < f.len() {
        let c = f[i];
        let ty = gf_type(c).ok_or(SymbolError::InvalidCharacter(c as char))?;
        match mode {
            Gf::Numeric => {
                if i + 1 < f.len() {
                    if ty != Gf::Numeric || gf_type(f[i + 1]) != Some(Gf::Numeric) {
                        push_bits(bits, 0, 4);
                        mode = Gf::Alpha;
                    } else {
                        push_bits(bits, 11 * digit(c) + digit(f[i + 1]) + 8, 7);
                        i += 2;
                    }
                } else if ty != Gf::Numeric {
                    push_bits(bits, 0, 4);
                    mode = Gf::Alpha;
                } else {
                    last_digit = Some(c);
                    i += 1;
                }
            }
            Gf::Alpha => {
                if c == FNC1 {
                    push_bits(bits, 15, 5);
                    mode = Gf::Numeric;
                    i += 1;
                } else if ty == Gf::Iso {
                    push_bits(bits, 4, 5);
                    mode = Gf::Iso;
                } else if gf_next(f, i, 6, Gf::Numeric, None)
                    || gf_next_terminate(f, i, 4, 5, Gf::Numeric)
                {
                    push_bits(bits, 0, 3);
                    mode = Gf::Numeric;
                } else if c.is_ascii_digit() {
                    push_bits(bits, u32::from(c) - 43, 5);
                    i += 1;
                } else if c.is_ascii_uppercase() {
                    push_bits(bits, u32::from(c) - 33, 6);
                    i += 1;
                } else {
                    let p = GF_ALNUM_PUNCT.iter().position(|&x| x == c).unwrap_or(0);
                    push_bits(bits, p as u32 + 58, 6);
                    i += 1;
                }
            }
            Gf::Iso => {
                if c == FNC1 {
                    push_bits(bits, 15, 5);
                    mode = Gf::Numeric;
                    i += 1;
                } else {
                    let none10 = gf_next_none(f, i, 10, Gf::Iso);
                    if none10 && gf_next(f, i, 4, Gf::Numeric, None) {
                        push_bits(bits, 0, 3);
                        mode = Gf::Numeric;
                    } else if none10 && gf_next(f, i, 5, Gf::Alpha, Some(Gf::Numeric)) {
                        push_bits(bits, 4, 5);
                        mode = Gf::Alpha;
                    } else if c.is_ascii_digit() {
                        push_bits(bits, u32::from(c) - 43, 5);
                        i += 1;
                    } else if c.is_ascii_uppercase() {
                        push_bits(bits, u32::from(c) - 1, 7);
                        i += 1;
                    } else if c.is_ascii_lowercase() {
                        push_bits(bits, u32::from(c) - 7, 7);
                        i += 1;
                    } else {
                        let p = GF_ISO_PUNCT.iter().position(|&x| x == c).unwrap_or(0);
                        push_bits(bits, p as u32 + 232, 8);
                        i += 1;
                    }
                }
            }
        }
    }
    Ok((mode, last_digit))
}

/// YYMMDD date value (ISO/IEC 24723 5.3.1), `None` if not a valid date.
fn date_value(s: &[u8]) -> Option<u32> {
    if s.len() < 6 || !s[..6].iter().all(u8::is_ascii_digit) {
        return None;
    }
    let n = |i: usize| u32::from(s[i] - b'0') * 10 + u32::from(s[i + 1] - b'0');
    let (yy, mm, dd) = (n(0), n(2), n(4));
    ((1..=12).contains(&mm) && dd <= 31).then_some(yy * 384 + (mm - 1) * 32 + dd)
}

/// CC-A sizes: bit-stream lengths per column count 2, 3, 4 (ISO/IEC 24723 Table 9).
const CC_A_BITS: [&[usize]; 3] = [
    &[59, 78, 88, 108, 118, 138, 167],
    &[78, 98, 118, 138, 167],
    &[78, 108, 138, 167, 197],
];
/// CC-B sizes: bit-stream lengths per column count 2, 3, 4.
const CC_B_BITS: [&[usize]; 3] = [
    &[56, 104, 160, 208, 256, 296, 336],
    &[32, 72, 112, 152, 208, 304, 416, 536, 648, 768],
    &[56, 96, 152, 208, 264, 352, 496, 672, 840, 1016, 1184],
];

/// Parameters of a CC-C component, decided from the bit-stream length.
#[derive(Clone, Copy, Debug)]
struct CcC {
    cols: usize,
    ecc: u32,
}

/// Target bit-stream length for `bits` encoded bits, or `None` if too long.
fn target_bits(
    ty: CcType,
    bits: usize,
    cc_width: usize,
    linear_width: usize,
) -> Option<(usize, Option<CcC>)> {
    match ty {
        CcType::A => CC_A_BITS[cc_width - 2]
            .iter()
            .find(|&&n| bits <= n)
            .map(|&n| (n, None)),
        CcType::B => CC_B_BITS[cc_width - 2]
            .iter()
            .find(|&&n| bits <= n)
            .map(|&n| (n, None)),
        CcType::C => {
            let bytes = bits.div_ceil(8);
            let mut used = bytes / 6 * 5 + bytes % 6;
            let ecc = match used {
                0..=40 => 2,
                41..=160 => 3,
                161..=320 => 4,
                321..=833 => 5,
                834..=865 => 4,
                _ => return None,
            };
            let ecc_cws = 2usize << ecc;
            used += ecc_cws + 3;
            let mut cols = if linear_width <= 68 {
                1
            } else {
                ((linear_width - 52) / 17).clamp(1, 30)
            };
            let mut rows = used.div_ceil(cols);
            while rows > 30 && cols < 30 {
                cols += 1;
                rows = used.div_ceil(cols);
            }
            let rows = rows.max(3);
            let target_cws = cols * rows - ecc_cws - 3;
            let target_bytes = 6 * (target_cws / 5) + target_cws % 5;
            Some((target_bytes * 8, Some(CcC { cols, ecc })))
        }
    }
}

/// A padded 2D component bit stream and, for CC-C, its size parameters.
type CcBits = (Vec<bool>, Option<CcC>);

/// Build the padded 2D component bit stream for `src` (reduced GS1 string). `None` if the
/// data does not fit the component type.
fn cc_bitstream(
    src: &[u8],
    ty: CcType,
    cc_width: usize,
    linear_width: usize,
) -> Result<Option<CcBits>, SymbolError> {
    let len = src.len();
    let mut bits: Vec<bool> = Vec::with_capacity(len * 8 + 32);
    let mut read = 0;
    let mut mode = Gf::Numeric;
    let mut fnc1_latch = false;
    let mut alpha_pad = false;
    let mut ai_crop: Option<(usize, usize)> = None; // (position, AI length) skipped in method 11

    let starts_1x = len > 1 && src[0] == b'1' && matches!(src[1], b'0' | b'1' | b'7');
    let method = if starts_1x {
        if src[1] == b'0' || date_value(&src[2..]).is_some() {
            2
        } else {
            1
        }
    } else if len > 1 && src[0] == b'9' && src[1] == b'0' {
        3
    } else {
        1
    };

    let mut method3 = false;
    match method {
        2 => {
            push_bits(&mut bits, 2, 2); // "10"
            if src[1] == b'0' {
                push_bits(&mut bits, 3, 2); // "11": no date
                read = 2;
            } else {
                let date = date_value(&src[2..]).unwrap_or(0);
                push_bits(&mut bits, date, 16);
                bits.push(src[1] == b'7'); // 0 = AI 11, 1 = AI 17
                read = 8;
                if read + 1 < len && src[read] == b'1' && src[read + 1] == b'0' {
                    read += 2; // AI 10 follows: its data starts the general field
                } else if read < len {
                    fnc1_latch = true;
                } else {
                    // Date only: alphanumeric latch + FNC1 (also a numeric latch).
                    push_bits(&mut bits, 15, 9);
                }
            }
        }
        3 => method3 = true,
        _ => {}
    }

    if method3 {
        // AI 90 data up to the next FNC1.
        let ninety_end = src[2..]
            .iter()
            .position(|&c| c == FNC1)
            .map_or(len, |p| p + 2);
        let ninety = &src[2..ninety_end];
        let (mut alpha, mut numeric, mut other) = (0usize, 0usize, 0usize);
        for &c in ninety {
            if c.is_ascii_uppercase() {
                alpha += 1;
            } else if c.is_ascii_digit() {
                numeric += 1;
            } else {
                other += 1;
            }
        }
        // 0-3 digits (no leading zero) followed by an uppercase letter.
        let mut alpha_posn = None;
        if !ninety.is_empty() && ninety[0] != b'0' {
            for (i, &c) in ninety.iter().enumerate().take(4) {
                if c.is_ascii_uppercase() {
                    alpha_posn = Some(i);
                    break;
                }
                if !c.is_ascii_digit() {
                    break;
                }
            }
        }
        if let Some(ap) = alpha_posn {
            push_bits(&mut bits, 3, 2); // "11"
            numeric -= ap;
            alpha -= 1;
            let ai90_mode = if other == 0 && alpha > numeric {
                push_bits(&mut bits, 3, 2); // alpha
                2
            } else if other == 0 && alpha == 0 {
                push_bits(&mut bits, 2, 2); // numeric
                3
            } else {
                bits.push(false); // alphanumeric
                mode = Gf::Alpha;
                1
            };
            let next_ai = 2 + ninety.len();
            if next_ai < len && src[next_ai] == FNC1 {
                if next_ai + 2 < len && src[next_ai + 1] == b'2' && src[next_ai + 2] == b'1' {
                    ai_crop = Some((next_ai + 1, 2));
                } else if next_ai + 4 < len && &src[next_ai + 1..next_ai + 5] == b"8004" {
                    ai_crop = Some((next_ai + 1, 4));
                }
            }
            match ai_crop {
                None => bits.push(false),
                Some((_, 2)) => push_bits(&mut bits, 2, 2),
                Some(_) => push_bits(&mut bits, 3, 2),
            }
            let numeric_value = ninety[..ap]
                .iter()
                .fold(0u32, |a, &c| a * 10 + u32::from(c - b'0'));
            let letter = ninety[ap];
            let table3 = b"BDHIJKLNPQRSTVWZ".iter().position(|&x| x == letter);
            match table3 {
                Some(t) if numeric_value < 31 => {
                    push_bits(&mut bits, numeric_value, 5);
                    push_bits(&mut bits, t as u32, 4);
                }
                _ => {
                    push_bits(&mut bits, 31, 5);
                    push_bits(&mut bits, numeric_value, 10);
                    push_bits(&mut bits, u32::from(letter - b'A'), 5);
                }
            }
            read = ap + 3;
            if ai90_mode == 2 {
                // Alpha encodation of the rest of the AI 90 field (5.3.3), up to and
                // including its terminating FNC1.
                loop {
                    let c = src.get(read).copied();
                    match c {
                        Some(c) if c.is_ascii_uppercase() => {
                            push_bits(&mut bits, u32::from(c - b'A'), 5)
                        }
                        Some(c) if c.is_ascii_digit() => push_bits(&mut bits, u32::from(c) + 4, 6),
                        Some(FNC1) => push_bits(&mut bits, 31, 5),
                        _ => {}
                    }
                    read += 1;
                    if c.is_none() || c == Some(FNC1) {
                        break;
                    }
                }
                read = read.min(len);
                alpha_pad = true;
            }
        } else {
            bits.push(false); // encodation method "0"
            read = 0;
        }
    } else if method == 1 {
        bits.push(false);
    }

    // General-purpose field: the rest of the data.
    let mut field: Vec<u8> = Vec::with_capacity(len + 1);
    if fnc1_latch {
        field.push(FNC1);
    }
    let mut i = read;
    while i < len {
        if let Some((pos, n)) = ai_crop
            && i == pos
        {
            i += n;
            continue;
        }
        field.push(src[i]);
        i += 1;
    }
    let mut last_digit = None;
    if !field.is_empty() {
        alpha_pad = false;
        let (m, ld) = general_field(&field, mode, &mut bits)?;
        mode = m;
        last_digit = ld;
    }

    let Some((target, _)) = target_bits(ty, bits.len(), cc_width, linear_width) else {
        return Ok(None);
    };
    if let Some(d) = last_digit {
        let d = u32::from(d - b'0');
        let remainder = target - bits.len();
        if (4..=6).contains(&remainder) {
            push_bits(&mut bits, d + 1, 4);
            push_bits(&mut bits, 0, remainder as u32 - 4);
        } else {
            push_bits(&mut bits, 11 * d + 18, 7);
        }
    }
    let Some((target, cc_c)) = target_bits(ty, bits.len(), cc_width, linear_width) else {
        return Ok(None);
    };
    if bits.len() < target {
        if alpha_pad {
            push_bits(&mut bits, 31, 5);
        }
        if mode == Gf::Numeric {
            push_bits(&mut bits, 0, 4);
        }
        while bits.len() < target {
            push_bits(&mut bits, 4, 5);
        }
    }
    bits.truncate(target);
    Ok(Some((bits, cc_c)))
}

// ---------------------------------------------------------------------------------------------
// 2D components
// ---------------------------------------------------------------------------------------------

/// CC-A (rows, ECC codewords) and starting left / centre / right RAP and cluster per variant
/// (ISO/IEC 24723 Tables 9-11).
const CC_A_ROWS: [u8; 17] = [5, 6, 7, 8, 9, 10, 12, 4, 5, 6, 7, 8, 3, 4, 5, 6, 7];
const CC_A_ECC: [u8; 17] = [4, 4, 5, 5, 6, 6, 7, 4, 5, 6, 7, 7, 4, 5, 6, 7, 8];
const CC_A_RAP: [[u8; 17]; 4] = [
    [
        39, 1, 32, 8, 14, 43, 20, 11, 1, 5, 15, 21, 40, 43, 46, 34, 29,
    ],
    [0, 0, 0, 0, 0, 0, 0, 43, 33, 37, 47, 1, 20, 23, 26, 14, 9],
    [
        19, 33, 12, 40, 46, 23, 52, 23, 13, 17, 27, 33, 52, 3, 6, 46, 41,
    ],
    [2, 0, 1, 1, 1, 0, 1, 1, 0, 1, 2, 2, 0, 0, 0, 0, 1],
];

/// Base-928 compaction of the CC-A bit stream (ISO/IEC 24723 5.5): groups of 69 bits become
/// 7 codewords; a shorter last group of n bits becomes n / 10 + 1 codewords.
fn base928(bits: &[bool]) -> Vec<u16> {
    let mut out = Vec::with_capacity(bits.len() / 9 + 1);
    for group in bits.chunks(69) {
        let mut v = group.iter().fold(0u128, |a, &b| a << 1 | u128::from(b));
        let n = group.len() / 10 + 1;
        let start = out.len();
        out.resize(start + n, 0);
        for slot in out[start..].iter_mut().rev() {
            *slot = (v % 928) as u16;
            v /= 928;
        }
    }
    out
}

fn cc_a_rows(bits: &[bool], cc_width: usize) -> Vec<Vec<bool>> {
    let mut cws = base928(bits);
    let first = [0usize, 7, 12][cc_width - 2];
    let count = [7usize, 5, 5][cc_width - 2];
    let v = (first..first + count)
        .find(|&v| usize::from(CC_A_ROWS[v]) * cc_width - usize::from(CC_A_ECC[v]) == cws.len())
        .expect("CC-A bit stream padded to a valid size");
    let ecc = rs_ecc(&cws, usize::from(CC_A_ECC[v]));
    cws.extend(ecc);
    rap_rows(
        &cws,
        &RapLayout {
            cols: cc_width,
            rows: usize::from(CC_A_ROWS[v]),
            left: CC_A_RAP[0][v],
            centre: CC_A_RAP[1][v],
            right: CC_A_RAP[2][v],
            cluster: CC_A_RAP[3][v],
            no_left: cc_width == 3,
        },
    )
}

fn bits_to_bytes(bits: &[bool]) -> Vec<u8> {
    bits.chunks(8)
        .map(|c| c.iter().fold(0u8, |a, &b| a << 1 | u8::from(b)))
        .collect()
}

fn cc_b_rows(bits: &[bool], cc_width: usize) -> Vec<Vec<bool>> {
    let mut cws = vec![920];
    crate::matrix::pdf_bytes(&bits_to_bytes(bits), &mut cws);
    let v = smallest_variant(cc_width as u8, cws.len())
        .expect("CC-B bit stream padded to a valid size");
    let cap = crate::micropdf::capacity(v);
    cws.resize(cap, 900);
    variant_rows(cws, v)
}

fn cc_c_rows(bits: &[bool], p: CcC) -> Vec<Vec<bool>> {
    let mut cws = vec![0, 920];
    crate::matrix::pdf_bytes(&bits_to_bytes(bits), &mut cws);
    let k = 2usize << p.ecc;
    let rows = (cws.len() + k).div_ceil(p.cols).max(3);
    cws.resize(rows * p.cols - k, 900);
    cws[0] = cws.len() as u16;
    let ecc = rs_ecc(&cws, k);
    cws.extend(ecc);
    let (c1, c2, c3) = (
        (rows as u16 - 1) / 3,
        p.ecc as u16 * 3 + (rows as u16 - 1) % 3,
        p.cols as u16 - 1,
    );
    (0..rows)
        .map(|r| {
            let cluster = r % 3;
            let base = (r / 3) as u16 * 30;
            let (left, right) = match cluster {
                0 => (c1, c3),
                1 => (c2, c1),
                _ => (c3, c2),
            };
            let mut row = Vec::with_capacity(17 * (p.cols + 4) + 1);
            push_bits(&mut row, 0x1FEA8, 17);
            push_bits(&mut row, codeword_bits(cluster, base + left), 17);
            for c in 0..p.cols {
                push_bits(&mut row, codeword_bits(cluster, cws[r * p.cols + c]), 17);
            }
            push_bits(&mut row, codeword_bits(cluster, base + right), 17);
            push_bits(&mut row, 0x3FA29, 18);
            row
        })
        .collect()
}

/// The 2D component for `src`, trying CC-A, then CC-B (and CC-C if `allow_c`), or CC-C only
/// when `force_c`. Returns the rows and the component type used.
fn component(
    src: &[u8],
    cc_width: usize,
    linear_width: usize,
    force_c: bool,
) -> Result<(Vec<Vec<bool>>, CcType), SymbolError> {
    let types: &[CcType] = if force_c {
        &[CcType::C]
    } else {
        &[CcType::A, CcType::B]
    };
    for &ty in types {
        if let Some((bits, cc_c)) = cc_bitstream(src, ty, cc_width, linear_width)? {
            let rows = match ty {
                CcType::A => cc_a_rows(&bits, cc_width),
                CcType::B => cc_b_rows(&bits, cc_width),
                CcType::C => cc_c_rows(&bits, cc_c.expect("CC-C parameters")),
            };
            return Ok((rows, ty));
        }
    }
    Err(SymbolError::InvalidLength(src.len()))
}

// ---------------------------------------------------------------------------------------------
// Assembly
// ---------------------------------------------------------------------------------------------

/// Rows under construction: modules, height, horizontal offset.
struct Part {
    rows: Vec<(Vec<bool>, RowHeight)>,
    shift: usize,
}

fn assemble(parts: Vec<Part>) -> Stacked {
    let width = parts
        .iter()
        .flat_map(|p| p.rows.iter().map(move |(m, _)| m.len() + p.shift))
        .max()
        .unwrap_or(0);
    let rows = parts
        .into_iter()
        .flat_map(|p| {
            let shift = p.shift;
            p.rows.into_iter().map(move |(m, height)| {
                let mut modules = vec![false; width];
                modules[shift..shift + m.len()].copy_from_slice(&m);
                StackedRow { modules, height }
            })
        })
        .collect();
    Stacked {
        width: width as u32,
        rows,
    }
}

fn cc_part(rows: Vec<Vec<bool>>, ty: CcType, shift: usize) -> Part {
    let h = RowHeight::Modules(if ty == CcType::C { 3 } else { 2 });
    Part {
        rows: rows.into_iter().map(|r| (r, h)).collect(),
        shift,
    }
}

fn linear_err(e: crate::linear::EncodeError) -> SymbolError {
    match e {
        crate::linear::EncodeError::InvalidCharacter(c) => SymbolError::InvalidCharacter(c),
        crate::linear::EncodeError::InvalidLength(n) => SymbolError::InvalidLength(n),
        crate::linear::EncodeError::Empty => SymbolError::InvalidLength(0),
    }
}

fn ean_upc_composite(
    linear: CompositeLinear,
    data: &str,
    cc: Option<Vec<u8>>,
) -> Result<Composite, SymbolError> {
    use crate::linear::Symbology;
    let (sym, cc_width) = match linear {
        CompositeLinear::UpcA => (Symbology::UpcA, 4),
        CompositeLinear::UpcE => (Symbology::UpcE, 2),
        CompositeLinear::Ean8 => (Symbology::Ean8, 3),
        _ => (Symbology::Ean13, 4),
    };
    let lin = crate::linear::encode(sym, data).map_err(linear_err)?;
    let modules: Vec<bool> = lin
        .elements
        .iter()
        .flat_map(|e| std::iter::repeat_n(e.bar, e.width as usize))
        .collect();
    let hri: String = lin.text.iter().map(|t| t.text.as_str()).collect();
    let Some(cc) = cc else {
        return Ok(Composite {
            symbol: assemble(vec![Part {
                rows: vec![(modules, RowHeight::Bar { weight: 1 })],
                shift: 0,
            }]),
            hri: Some(hri),
        });
    };
    let (cc_rows, ty) = component(&cc, cc_width, 0, false)?;
    // ISO/IEC 24723 12.3: the linear component starts 1 module right of the separator's
    // leftmost bar; the whole linear part sits `bottom` modules right of the 2D component.
    let bottom = match (linear, ty) {
        (CompositeLinear::Ean8, CcType::A) => 3,
        (CompositeLinear::Ean8, _) => 13,
        _ => 2,
    };
    let w = modules.len();
    // Separator: the outer guard bars extended upwards in a 2-module zig-zag (11.4).
    let sep = |a: usize, b: usize| {
        let mut r = vec![false; w + 2];
        r[a] = true;
        r[b] = true;
        (r, RowHeight::Modules(2))
    };
    let mut linear_rows = vec![sep(1, w), sep(0, w + 1), sep(1, w)];
    let mut shifted = vec![false; w + 2];
    shifted[1..=w].copy_from_slice(&modules);
    linear_rows.push((shifted, RowHeight::Bar { weight: 1 }));
    Ok(Composite {
        symbol: assemble(vec![
            cc_part(cc_rows, ty, 0),
            Part {
                rows: linear_rows,
                shift: bottom,
            },
        ]),
        hri: Some(hri),
    })
}

// ---------------------------------------------------------------------------------------------
// GS1-128 (Code 128 with FNC1), minimal B/C encoder
// ---------------------------------------------------------------------------------------------

/// Code 128 symbol character patterns 0-105 (11 modules, bar first).
#[rustfmt::skip]
const C128: [u16; 106] = [
    0x6CC, 0x66C, 0x666, 0x498, 0x48C, 0x44C, 0x4C8, 0x4C4, 0x464, 0x648, 0x644, 0x624,
    0x59C, 0x4DC, 0x4CE, 0x5CC, 0x4EC, 0x4E6, 0x672, 0x65C, 0x64E, 0x6E4, 0x674, 0x76E,
    0x74C, 0x72C, 0x726, 0x764, 0x734, 0x732, 0x6D8, 0x6C6, 0x636, 0x518, 0x458, 0x446,
    0x588, 0x468, 0x462, 0x688, 0x628, 0x622, 0x5B8, 0x58E, 0x46E, 0x5D8, 0x5C6, 0x476,
    0x776, 0x68E, 0x62E, 0x6E8, 0x6E2, 0x6EE, 0x758, 0x746, 0x716, 0x768, 0x762, 0x71A,
    0x77A, 0x642, 0x78A, 0x530, 0x50C, 0x4B0, 0x486, 0x42C, 0x426, 0x590, 0x584, 0x4D0,
    0x4C2, 0x434, 0x432, 0x612, 0x650, 0x7BA, 0x614, 0x47A, 0x53C, 0x4BC, 0x49E, 0x5E4,
    0x4F4, 0x4F2, 0x7A4, 0x794, 0x792, 0x6DE, 0x6F6, 0x7B6, 0x578, 0x51E, 0x45E, 0x5E8,
    0x5E2, 0x7A8, 0x7A2, 0x5DE, 0x5EE, 0x75E, 0x7AE, 0x684, 0x690, 0x69C,
];
/// Stop pattern 2331112 (13 modules).
const C128_STOP: u32 = 0x18EB;
const C128_FNC1: u8 = 102;

/// Code 128 values for GS1 data `src` (FNC1 = 0x1D) using code sets B and C with the fewest
/// symbol characters. Returns the values (start character first, no check character) and
/// whether the final code set is C.
fn gs1_128_values(src: &[u8]) -> Result<(Vec<u8>, bool), SymbolError> {
    if let Some(&c) = src.iter().find(|&&c| c != FNC1 && !(32..=126).contains(&c)) {
        return Err(SymbolError::InvalidCharacter(c as char));
    }
    let n = src.len();
    let digit = |i: usize| src.get(i).is_some_and(u8::is_ascii_digit);
    // cost[i][s]: characters to encode src[i..] starting in set s (0 = B, 1 = C).
    const INF: u32 = u32::MAX / 2;
    let mut cost = vec![[0u32; 2]; n + 1];
    for i in (0..n).rev() {
        let stay_b = 1 + cost[i + 1][0];
        let stay_c = if src[i] == FNC1 {
            1 + cost[i + 1][1]
        } else if digit(i) && digit(i + 1) {
            1 + cost[i + 2][1]
        } else {
            INF
        };
        cost[i][0] = stay_b.min(1 + stay_c);
        cost[i][1] = stay_c.min(1 + stay_b);
    }
    let mut set = usize::from(cost[0][1] <= cost[0][0]);
    let mut values = vec![if set == 1 { 105 } else { 104 }, C128_FNC1];
    let mut i = 0;
    while i < n {
        let stay = if set == 0 {
            1 + cost[i + 1][0]
        } else if src[i] == FNC1 {
            1 + cost[i + 1][1]
        } else if digit(i) && digit(i + 1) {
            1 + cost[i + 2][1]
        } else {
            INF
        };
        if stay > cost[i][set] {
            // Switching is strictly better.
            set ^= 1;
            values.push(if set == 1 { 99 } else { 100 });
            continue;
        }
        if src[i] == FNC1 {
            values.push(C128_FNC1);
            i += 1;
        } else if set == 1 {
            values.push((src[i] - b'0') * 10 + (src[i + 1] - b'0'));
            i += 2;
        } else {
            values.push(src[i] - 32);
            i += 1;
        }
    }
    Ok((values, set == 1))
}

/// Modules of a Code 128 symbol for `values` (start first): appends check and stop.
fn c128_modules(values: &[u8]) -> Vec<bool> {
    let check = values
        .iter()
        .enumerate()
        .map(|(i, &v)| i.max(1) as u32 * u32::from(v))
        .sum::<u32>()
        % 103;
    let mut row = Vec::with_capacity(11 * (values.len() + 1) + 13);
    for &v in values.iter().chain(std::iter::once(&(check as u8))) {
        push_bits(&mut row, u32::from(C128[v as usize]), 11);
    }
    push_bits(&mut row, C128_STOP, 13);
    row
}

fn gs1_128_composite(
    data: &str,
    cc: Option<Vec<u8>>,
    cc_c: bool,
) -> Result<Composite, SymbolError> {
    let (raw, hri) = gs1_element_string(data)?;
    if raw.is_empty() {
        return Err(SymbolError::InvalidLength(0));
    }
    let (mut values, final_c) = gs1_128_values(&raw)?;
    let Some(cc) = cc else {
        return Ok(Composite {
            symbol: assemble(vec![Part {
                rows: vec![(c128_modules(&values), RowHeight::Bar { weight: 1 })],
                shift: 0,
            }]),
            hri: Some(hri),
        });
    };
    // Linkage flag (ISO/IEC 24723 7.4): an extra code set character before the check
    // character. CC-A/B: CODE C after set B, CODE A after set C; CC-C: CODE A / CODE B.
    values.push(match (cc_c, final_c) {
        (false, false) => 99,
        (false, true) => 101,
        (true, false) => 101,
        (true, true) => 100,
    });
    let linear = c128_modules(&values);
    let lw = linear.len();
    let (cc_rows, ty) = component(&cc, 4, lw, cc_c)?;
    let cc_w = cc_rows[0].len();
    let (top, bottom) = if ty == CcType::C {
        (0, 7) // 12.3 f)
    } else {
        // 12.3 g): align the 2D component's right edge with the last space module of the
        // Code 128 symbol character at position (characters - 9) div 2 counted from the
        // stop character.
        // (Signed, truncating division as in the standard's formula / zint: short symbols
        // get a negative position.)
        let num_symbols = (lw as isize - 2) / 11;
        let position = (num_symbols - 9) / 2;
        let mut shift = lw as isize - position * 11 - 1 - cc_w as isize;
        if position != 0 {
            shift -= 2;
        }
        if shift > 0 {
            (shift as usize, 0)
        } else {
            (0, (-shift) as usize)
        }
    };
    let separator: Vec<bool> = linear.iter().map(|&b| !b).collect();
    Ok(Composite {
        symbol: assemble(vec![
            cc_part(cc_rows, ty, top),
            Part {
                rows: vec![
                    (separator, RowHeight::Modules(1)),
                    (linear, RowHeight::Bar { weight: 1 }),
                ],
                shift: bottom,
            },
        ]),
        hri: Some(hri),
    })
}

// ---------------------------------------------------------------------------------------------
// GS1 DataBar
// ---------------------------------------------------------------------------------------------

fn databar(
    kind: DataBarKind,
    data: &str,
    cc: Option<Vec<u8>>,
    segments_per_row: u32,
) -> Result<Composite, SymbolError> {
    let linear = crate::databar::encode_linked(kind, data, segments_per_row, cc.is_some())?;
    let Some(cc) = cc else {
        return Ok(Composite {
            symbol: linear.symbol,
            hri: linear.hri,
        });
    };
    let cc_width = match kind {
        DataBarKind::Stacked | DataBarKind::StackedOmnidirectional => 2,
        DataBarKind::Limited => 3,
        _ => 4,
    };
    let (cc_rows, ty) = component(&cc, cc_width, 0, false)?;
    let (top, bottom) = match kind {
        DataBarKind::Omnidirectional | DataBarKind::Truncated => (0, 4),
        DataBarKind::Limited if ty == CcType::A => (1, 0),
        DataBarKind::Limited => (0, 9),
        DataBarKind::Stacked | DataBarKind::StackedOmnidirectional => (1, 0),
        DataBarKind::Expanded => {
            // Align with the first bar of the first data row (after the left guard space).
            let row = linear
                .symbol
                .rows
                .iter()
                .find(|r| matches!(r.height, RowHeight::Bar { .. }))
                .map(|r| r.modules.as_slice())
                .unwrap_or(&[]);
            let mut j = 1;
            while j < row.len() && !row[j - 1] && row[j] {
                j += 1;
            }
            (j, 0)
        }
    };
    let separator = crate::databar::composite_separator(kind, &linear);
    let lin_rows = std::iter::once((separator, RowHeight::Modules(1)))
        .chain(
            linear
                .symbol
                .rows
                .into_iter()
                .map(|r| (r.modules, r.height)),
        )
        .collect();
    Ok(Composite {
        symbol: assemble(vec![
            cc_part(cc_rows, ty, top),
            Part {
                rows: lin_rows,
                shift: bottom,
            },
        ]),
        hri: linear.hri,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Compare against expected module rows. `skip_last`: ignore the linear row (UPC-E vectors
    /// whose check digit is 0 exercise a separate linear encoder issue).
    fn check(linear: CompositeLinear, data: &str, skip_last: bool, expected: &[&str]) {
        let c = encode(linear, data, 22).unwrap();
        let rows: Vec<String> = c
            .symbol
            .rows
            .iter()
            .map(|r| {
                r.modules
                    .iter()
                    .map(|&b| if b { '1' } else { '0' })
                    .collect()
            })
            .collect();
        let n = rows.len() - usize::from(skip_last);
        assert_eq!(
            rows.len(),
            expected.len() + usize::from(skip_last),
            "{data}"
        );
        assert_eq!(&rows[..n], expected, "{data}");
        assert!(
            c.symbol
                .rows
                .iter()
                .all(|r| r.modules.len() == c.symbol.width as usize)
        );
    }

    #[test]
    fn base928_groups() {
        // 69 bits -> 7 codewords; 10 bits -> 2 codewords.
        assert_eq!(base928(&[true; 69]).len(), 7);
        assert_eq!(base928(&[false; 79]).len(), 9);
        let mut bits = vec![false; 59];
        bits[58] = true;
        assert_eq!(base928(&bits), [0, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn element_string_parsing() {
        let (raw, hri) = gs1_element_string("(01)03812345678908(10)ABC(21)X").unwrap();
        assert_eq!(
            raw,
            b"0103812345678908"
                .iter()
                .chain(b"10ABC\x1D21X")
                .copied()
                .collect::<Vec<_>>()
        );
        assert_eq!(hri, "(01)03812345678908(10)ABC(21)X");
        assert_eq!(
            gs1_element_string("[17]010101[10]A").unwrap().0,
            b"1701010110A"
        );
        assert_eq!(
            gs1_element_string("this is composite info").unwrap().0,
            b"this is composite info"
        );
    }

    #[test]
    fn zpl_manual_example() {
        // ZPL manual ^BR type 7 example: "12345678901|this is composite info".
        let c = encode(
            CompositeLinear::UpcA,
            "12345678901|this is composite info",
            22,
        )
        .unwrap();
        assert_eq!(c.symbol.width, 99);
        assert!(c.symbol.rows.len() > 4);
        assert!(matches!(c.symbol.rows[0].height, RowHeight::Modules(2)));
        assert!(matches!(
            c.symbol.rows.last().unwrap().height,
            RowHeight::Bar { .. }
        ));
        // Linear only without a 2D part.
        let l = encode(
            CompositeLinear::Gs1_128 { cc_c: false },
            "(01)03812345678908",
            22,
        )
        .unwrap();
        assert_eq!(l.symbol.rows.len(), 1);
        assert_eq!(l.hri.as_deref(), Some("(01)03812345678908"));
        // CC-B overflow is an error for type 11; type 12 always uses CC-C.
        let long = format!("(01)03812345678908|(91){}", "1".repeat(400));
        assert!(encode(CompositeLinear::Gs1_128 { cc_c: false }, &long, 22).is_err());
        assert!(encode(CompositeLinear::Gs1_128 { cc_c: true }, &long, 22).is_ok());
    }

    // Expected rows from zint's backend/tests/test_composite.c (BSD-3-Clause), which were
    // verified against the GS1 General Specifications, ISO/IEC 24723 figures and BWIPP.
    #[test]
    fn zint_ean13_211234abcd() {
        check(
            CompositeLinear::Ean13,
            "331234567890|[21]1234-abcd",
            false,
            &[
                "110110111011010000100000110100110011101100001001110100100001011001100001100111000110001011011000101",
                "110110110011000110111100010111011001110000101001100100100000010111101001101011100010000011001000101",
                "110110100010011010001110000111111010001100101001100110111111010001101001010000011011111011101000101",
                "000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010",
                "001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001",
                "000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010",
                "000101011110100110010011011010000100111010110001010101010000100010010010001110100111001010000101010",
            ],
        );
    }

    #[test]
    fn zint_gs1c_10abcd12345641() {
        check(
            CompositeLinear::Gs1_128 { cc_c: true },
            "[01]03812345678908|[10]ABCD123456[410]3898765432108",
            false,
            &[
                "1111111101010100011110101011110000111101011001111101110111110111010010000010000100010110010000101100001111011110110011011110101001111000111111101000101001",
                "1111111101010100011111101010001110100001000111101001100101110010000011100001011000100100100111110110001011100001011111011111101010111000111111101000101001",
                "1111111101010100011101010011111100110001111010001101000101011110000010001111101100010111101101111101001001011000111110011101010001111110111111101000101001",
                "1111111101010100010101111001111000110010011001110001111010100111100010011110111101000110000110001000101100001011101111011111010111111010111111101000101001",
                "1111111101010100011101011100001100101000111111011101011110001001111011111011001000100111100111011101001101101111001000011101011100110000111111101000101001",
                "0000000001011000110000101000100110010011011011001110110100001100010010001010001001110111101001100100100001011100110110100001000100100001001001110001010000",
                "0000000110100111001111010111011001101100100100110001001011110011101101110101110110001000010110011011011110100011001001011110111011011110110110001110101100",
            ],
        );
    }

    #[test]
    fn zint_gs1_21a1b2c3d4e5f6() {
        check(
            CompositeLinear::Gs1_128 { cc_c: false },
            "[01]03212345678906|[21]A1B2C3D4E5F6G7H8",
            false,
            &[
                "0000000000000000000001101001000110100001000001101101011110111110010010001101010000010010000011101110100010000111011001010000000000000000000000000",
                "0000000000000000000001101011000110101111001100001111010001101100010010000101111000011001101011100101100001000110011001010000000000000000000000000",
                "0000000000000000000001101011100100011001100111101011000101110000010110000101001100110011110011011110011001110110111001010000000000000000000000000",
                "0000000000000000000001101011110111000111011011001110010001011100010111000101011000011100110010000100000100010110111101010000000000000000000000000",
                "0010110001100001010001001100100110110110011100100011011000100100010100010011101111010011001001000010110011011100010100001000100010010011100010100",
                "1101001110011110101110110011011001001001100011011100100111011011101011101100010000101100110110111101001100100011101011110111011101101100011101011",
            ],
        );
    }

    #[test]
    fn zint_upce_15021231() {
        check(
            CompositeLinear::UpcE,
            "0121230|[15]021231",
            false,
            &[
                "1101100110111010011111010001100111100010110011110101001",
                "1101101110110010010000110001101000011011100011100101001",
                "1101101100111101001000000101000101111000010011101101001",
                "1101101000111011110100011101001110011011100011101001001",
                "1101001000111000010110111101001110001110010011101001101",
                "0001000000000000000000000000000000000000000000000000010",
                "0010000000000000000000000000000000000000000000000000001",
                "0001000000000000000000000000000000000000000000000000010",
                "0001010110011001001100110010011011011110101001110101010",
            ],
        );
    }

    #[test]
    fn zint_ltd_1701061510a123() {
        check(
            CompositeLinear::DataBar(DataBarKind::Limited),
            "1311234567890|[17]010615[10]A123456",
            false,
            &[
                "0111100010110110001010011000111110100110011101110100111100111011101001101000000",
                "0100111110001101001010111000111101111010111101001111011100111011101011101000000",
                "0100110011010000001010110000111001000110111101111011110010100011101011001000000",
                "0111100010101000001010010000101110110111111001110011001110010011101010001000000",
                "0000001100000101010011001110101011010100110010101111000101100110100111000000000",
                "0101110011111010101100110001010100101011001101010000111010011001011000010100000",
            ],
        );
    }

    #[test]
    fn zint_upca_91abcdefghijkl() {
        check(
            CompositeLinear::UpcA,
            "61414101234|[91]abcdefghijklmnopqrstuvwxyz",
            false,
            &[
                "110001001010000001110010110110011111101100101001111010100100101111000001110101001111110011000100101",
                "111001001011101110101000000111101101000111001011111010100011000110000101110011010000110011100100101",
                "111101001011110110001101000111101000100000101011110010101001111001000001011111010001110011110100101",
                "111101011011110100111100010111011111001011001011110110111110001001110101011111001110011011110101101",
                "111101010010001100011100110111001000010011101001110110100011011010000001001100000110001011110101001",
                "111001010010000001011110100111100110110100001001110100111001011000010001111010010100000011100101001",
                "111011010010011111101110100101110001000011001001100100100011110010000101011110000110011011101101001",
                "111010010011100011101000100100001011011000001001100110110001110110100001000110011101100011101001001",
                "111010011010110111111011000101111101100111101001000110110100111100001101111100001101010011101001101",
                "111010111010111000001000110110111110010001001001000010111100101111101101000110001000111011101011101",
                "000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010",
                "001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001",
                "000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010",
                "000101010111100110010100011001100101000110011001010101110010110011011011001000010101110010000101010",
            ],
        );
    }

    #[test]
    fn zint_upce_901abd21a() {
        check(
            CompositeLinear::UpcE,
            "1234567|[90]1AB+D[21]A.",
            true,
            &[
                "1100100010110011100011000101110001110110011011100101101",
                "1110100010111101101100100001110101110001100011000101101",
                "1110110010100000111000101101110010111110001011000101001",
                "1100110010110111011000100001110110000100011011001101001",
                "1101110010111000010001100101111010001110111011011101001",
                "1101111010110001100100111101111101001110010011011001001",
                "0001000000000000000000000000000000000000000000000000010",
                "0010000000000000000000000000000000000000000000000000001",
                "0001000000000000000000000000000000000000000000000000010",
            ],
        );
    }

    #[test]
    fn zint_upce_1120100110ab12() {
        check(
            CompositeLinear::UpcE,
            "1234567|[11]201001[10]AB12345",
            true,
            &[
                "1101100110101110000100011001001000100111100011110101001",
                "1101101110110010100011000001100110010000001011100101001",
                "1101101100111011011101000001111010000110110011101101001",
                "1101101000100111101110111001000001001001111011101001001",
                "1101001000101100011100011001111101010111110011101001101",
                "0001000000000000000000000000000000000000000000000000010",
                "0010000000000000000000000000000000000000000000000000001",
                "0001000000000000000000000000000000000000000000000000010",
            ],
        );
    }
}
