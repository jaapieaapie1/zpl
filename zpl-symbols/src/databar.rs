//! GS1 DataBar (ISO/IEC 24724:2011), ZPL `^BR` symbology types 1-6.
//!
//! * Omnidirectional, Truncated, Stacked, Stacked Omnidirectional and Limited encode a GTIN
//!   (13 digits, check digit computed).
//! * Expanded (and Expanded Stacked when `segments_per_row` is 2-20) encode GS1 element strings
//!   using the compressed encodation methods (AI 01 + 310x/320x/392x/393x/11/13/15/17) or
//!   general-purpose encodation.
//!
//! Row heights: ZPL documents `^BR` parameter `e` (bar height) as affecting only the linear part
//! of the UCC/EAN-128 composites (types 11/12), i.e. DataBar types 1-6 have symbology-defined row
//! heights. Every bar row therefore uses `RowHeight::Bar { weight }` where `weight` is the
//! nominal row height **in modules** (Omni 33, Truncated 13, Stacked 5 + 7, Stacked Omni 33 + 33,
//! Limited 10, Expanded 34 per row). A renderer can either draw each bar row `weight * module`
//! dots tall (Zebra semantics for types 1-6), or split a bar height by the weights (the 5:7
//! Stacked ratio is kept). Separator rows are `RowHeight::Modules(1)`; the renderer draws them
//! with the `^BR` separator height `d` (1 or 2 dots).
//!
//! Width algorithm and tables follow ISO/IEC 24724 (Annex B `getRSSwidths`), structured after
//! zint's `rss.c` (BSD-3-Clause).

use crate::{RowHeight, Stacked, StackedRow, SymbolError};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataBarKind {
    Omnidirectional,
    Truncated,
    Stacked,
    StackedOmnidirectional,
    Limited,
    Expanded,
}

impl DataBarKind {
    /// Kind for ZPL `^BR` symbology type 1-6.
    pub fn from_zpl(kind: u32) -> Option<Self> {
        Some(match kind {
            1 => Self::Omnidirectional,
            2 => Self::Truncated,
            3 => Self::Stacked,
            4 => Self::StackedOmnidirectional,
            5 => Self::Limited,
            6 => Self::Expanded,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DataBar {
    /// Rows top to bottom including separator patterns. Bar rows use `RowHeight::Bar { weight }`
    /// with `weight` = nominal row height in modules (see module docs), separator rows
    /// `RowHeight::Modules(1)` (drawn with the `^BR` separator height).
    pub symbol: Stacked,
    /// Human readable text "(01)12345678901231" style.
    pub hri: Option<String>,
}

/// Encode a GS1 DataBar symbol. See [`encode_linked`].
pub fn encode(
    kind: DataBarKind,
    data: &str,
    segments_per_row: u32,
) -> Result<DataBar, SymbolError> {
    encode_linked(kind, data, segments_per_row, false)
}

/// Encode a GS1 DataBar symbol.
///
/// `data` is the `^FD` content. Omni/Truncated/Stacked/Limited: a GTIN of up to 13 digits
/// (left zero-padded), or 14 digits including a valid check digit, optionally prefixed with
/// `(01)`, `[01]` or `01`. Limited only encodes values below 2000000000000 (leading `0`/`1`).
/// Expanded: GS1 element strings either with AIs in parentheses/brackets
/// (`(01)98898765432106(3202)012345(15)991231`) or raw (`0198898765432106320201234515991231`)
/// with variable-length fields terminated by GS (0x1D) / FNC1.
///
/// `segments_per_row`: Expanded Stacked segments (symbol characters) per row, even 2-20;
/// 0 or >= 22 gives a single row Expanded symbol. Ignored for the other kinds.
///
/// `linked`: sets the 2D composite component linkage flag (for GS1 composite symbols).
pub fn encode_linked(
    kind: DataBarKind,
    data: &str,
    segments_per_row: u32,
    linked: bool,
) -> Result<DataBar, SymbolError> {
    match kind {
        DataBarKind::Expanded => expanded(data, segments_per_row, linked),
        DataBarKind::Limited => limited(data, linked),
        _ => omni(kind, data, linked),
    }
}

/// Separator row (width `symbol.symbol.width`) to place directly above the first bar row of
/// `symbol` when it is the linear component of a GS1 composite symbol (ISO/IEC 24723 / 24724
/// separator pattern: complement of the row below with finder pattern adjustments).
pub fn composite_separator(kind: DataBarKind, symbol: &DataBar) -> Vec<bool> {
    let st = &symbol.symbol;
    let row = &st.rows[0].modules;
    let width = st.width as usize;
    let mut sep = vec![false; width];
    match kind {
        DataBarKind::Omnidirectional | DataBarKind::Truncated => {
            omn_separator(&mut sep, row, 96, &[18, 63], false)
        }
        DataBarKind::Stacked | DataBarKind::StackedOmnidirectional => {
            omn_separator(&mut sep, row, 50, &[18], false)
        }
        DataBarKind::Limited => {
            for i in 4..70 {
                sep[i] = !row[i];
            }
        }
        DataBarKind::Expanded => {
            // Count the finder patterns in the first row: 49 modules per segment pair.
            let cols = width.saturating_sub(4).div_ceil(49);
            let mut latch = false;
            exp_separator(
                &mut sep,
                row,
                row_used_width(row, width),
                cols,
                0,
                true,
                false,
                &mut latch,
            );
        }
    }
    sep
}

// ---------------------------------------------------------------------------------------------
// Width generation (ISO/IEC 24724 Annex B)

fn combins(n: i32, r: i32) -> i32 {
    let (min_denom, max_denom) = if n - r > r { (r, n - r) } else { (n - r, r) };
    let mut val: i32 = 1;
    let mut j = 1;
    let mut i = n;
    while i > max_denom {
        val *= i;
        if j <= min_denom {
            val /= j;
            j += 1;
        }
        i -= 1;
    }
    while j <= min_denom {
        val /= j;
        j += 1;
    }
    val
}

/// `getRSSwidths`: element widths of one odd/even subset of a character.
fn get_widths(mut val: i32, mut n: i32, elements: i32, max_width: i32, no_narrow: bool) -> [u8; 7] {
    let mut widths = [0u8; 7];
    let mut narrow_mask = 0u32;
    let mut bar = 0;
    while bar < elements - 1 {
        let mut elm_width = 1;
        narrow_mask |= 1 << bar;
        let mut sub_val;
        loop {
            sub_val = combins(n - elm_width - 1, elements - bar - 2);
            if no_narrow
                && narrow_mask == 0
                && n - elm_width - (elements - bar - 1) >= elements - bar - 1
            {
                sub_val -= combins(n - elm_width - (elements - bar), elements - bar - 2);
            }
            if elements - bar - 1 > 1 {
                let mut less_val = 0;
                let mut mxw = n - elm_width - (elements - bar - 2);
                while mxw > max_width {
                    less_val += combins(n - elm_width - mxw - 1, elements - bar - 3);
                    mxw -= 1;
                }
                sub_val -= less_val * (elements - 1 - bar);
            } else if n - elm_width > max_width {
                sub_val -= 1;
            }
            val -= sub_val;
            if val < 0 {
                break;
            }
            elm_width += 1;
            narrow_mask &= !(1 << bar);
        }
        val += sub_val;
        n -= elm_width;
        widths[bar as usize] = elm_width as u8;
        bar += 1;
    }
    widths[bar as usize] = n as u8;
    widths
}

/// Interleaved odd/even widths of a character (`2 * elements` entries used).
#[allow(clippy::too_many_arguments)]
fn char_widths(
    v_odd: i32,
    v_even: i32,
    n_odd: i32,
    n_even: i32,
    elements: i32,
    max_width: i32,
    no_narrow: bool,
) -> [u8; 14] {
    let odd = get_widths(v_odd, n_odd, elements, max_width, no_narrow);
    let even = get_widths(v_even, n_even, elements, 9 - max_width, !no_narrow);
    let mut out = [0u8; 14];
    for i in 0..elements as usize {
        out[2 * i] = odd[i];
        out[2 * i + 1] = even[i];
    }
    out
}

/// Append runs of alternating colour to `row`, starting dark if `dark`.
fn expand(row: &mut Vec<bool>, mut dark: bool, widths: &[u8]) {
    for &w in widths {
        row.extend(std::iter::repeat_n(dark, w as usize));
        dark = !dark;
    }
}

fn bar_row(modules: Vec<bool>, weight: u32) -> StackedRow {
    StackedRow {
        modules,
        height: RowHeight::Bar { weight },
    }
}

fn sep_row(modules: Vec<bool>) -> StackedRow {
    StackedRow {
        modules,
        height: RowHeight::Modules(1),
    }
}

fn gs1_check_digit(digits: &[u8]) -> u8 {
    // digits are values 0-9; weight 3 on the rightmost.
    let mut sum = 0u32;
    for (i, &d) in digits.iter().rev().enumerate() {
        sum += d as u32 * if i % 2 == 0 { 3 } else { 1 };
    }
    ((10 - sum % 10) % 10) as u8
}

/// Parse a GTIN for the fixed-length DataBar variants into 13 digit values.
fn parse_gtin(data: &str) -> Result<[u8; 13], SymbolError> {
    let mut s = data.trim().as_bytes();
    if (s.len() == 17 || s.len() == 18) && (s.starts_with(b"(01)") || s.starts_with(b"[01]")) {
        s = &s[4..];
    } else if (s.len() == 15 || s.len() == 16) && s.starts_with(b"01") {
        s = &s[2..];
    }
    if s.is_empty() || s.len() > 14 {
        return Err(SymbolError::InvalidLength(data.len()));
    }
    if let Some(&c) = s.iter().find(|c| !c.is_ascii_digit()) {
        return Err(SymbolError::InvalidCharacter(c as char));
    }
    let digits: Vec<u8> = s.iter().map(|c| c - b'0').collect();
    let body = &digits[..digits.len().min(13)];
    let mut out = [0u8; 13];
    out[13 - body.len()..].copy_from_slice(body);
    if digits.len() == 14 && gs1_check_digit(&out) != digits[13] {
        return Err(SymbolError::InvalidData(format!(
            "invalid GTIN check digit '{}', expected '{}'",
            digits[13],
            gs1_check_digit(&out)
        )));
    }
    Ok(out)
}

fn gtin_hri(digits: &[u8; 13]) -> String {
    let mut s = String::with_capacity(18);
    s.push_str("(01)");
    for &d in digits {
        s.push((b'0' + d) as char);
    }
    s.push((b'0' + gs1_check_digit(digits)) as char);
    s
}

// ---------------------------------------------------------------------------------------------
// Omnidirectional / Truncated / Stacked / Stacked Omnidirectional

const OMN_G_SUM: [i32; 9] = [0, 161, 961, 2015, 2715, 0, 336, 1036, 1516];
const OMN_T: [i32; 9] = [1, 10, 34, 70, 126, 4, 20, 48, 81];
const OMN_MODULES: [i32; 18] = [12, 10, 8, 6, 4, 5, 7, 9, 11, 4, 6, 8, 10, 12, 10, 8, 6, 4];
const OMN_WIDEST: [i32; 9] = [8, 6, 4, 3, 1, 2, 4, 6, 8];
const OMN_FINDER: [[u8; 5]; 9] = [
    [3, 8, 2, 1, 1],
    [3, 5, 5, 1, 1],
    [3, 3, 7, 1, 1],
    [3, 1, 9, 1, 1],
    [2, 7, 4, 1, 1],
    [2, 5, 6, 1, 1],
    [2, 3, 8, 1, 1],
    [1, 5, 7, 1, 1],
    [1, 3, 9, 1, 1],
];
const OMN_WEIGHT: [[i32; 8]; 4] = [
    [1, 3, 9, 27, 2, 6, 18, 54],
    [4, 12, 36, 29, 8, 24, 72, 58],
    [16, 48, 65, 37, 32, 17, 51, 74],
    [64, 34, 23, 69, 49, 68, 46, 59],
];

fn omn_group(val: i32, outside: bool) -> usize {
    let (start, end) = if outside { (0, 4) } else { (5, 8) };
    for i in start..end {
        if val < OMN_G_SUM[i + 1] {
            return i;
        }
    }
    end
}

/// The 46 element widths of the linear symbol (starting with a space) and the right check value.
fn omn_widths(digits: &[u8; 13], linked: bool) -> ([u8; 46], usize) {
    let mut val: u64 = digits.iter().fold(0, |a, &d| a * 10 + d as u64);
    if linked {
        val += 10_000_000_000_000;
    }
    let left = (val / 4_537_077) as i32;
    let right = (val % 4_537_077) as i32;
    let dc = [left / 1597, left % 1597, right / 1597, right % 1597];

    let mut dw = [[0u8; 14]; 4];
    for i in 0..4 {
        let outside = i & 1 == 0;
        let group = omn_group(dc[i], outside);
        let v = dc[i] - OMN_G_SUM[group];
        let (v_div, v_mod) = (v / OMN_T[group], v % OMN_T[group]);
        dw[i] = char_widths(
            if outside { v_div } else { v_mod },
            if outside { v_mod } else { v_div },
            OMN_MODULES[group],
            OMN_MODULES[group + 9],
            4,
            OMN_WIDEST[group],
            !outside,
        );
    }

    let mut checksum = 0;
    for i in 0..4 {
        for j in 0..8 {
            checksum += OMN_WEIGHT[i][j] * dw[i][j] as i32;
        }
    }
    let mut checksum = (checksum % 79) as usize;
    if checksum >= 8 {
        checksum += 1;
    }
    if checksum >= 72 {
        checksum += 1;
    }
    let (c_left, c_right) = (checksum / 9, checksum % 9);

    let mut tw = [0u8; 46];
    tw[0] = 1;
    tw[1] = 1;
    tw[44] = 1;
    tw[45] = 1;
    for i in 0..8 {
        tw[i + 2] = dw[0][i];
        tw[i + 15] = dw[1][7 - i];
        tw[i + 23] = dw[3][i];
        tw[i + 36] = dw[2][7 - i];
    }
    for i in 0..5 {
        tw[i + 10] = OMN_FINDER[c_left][i];
        tw[i + 31] = OMN_FINDER[c_right][4 - i];
    }
    (tw, c_right)
}

/// Finder adjustment of an Omnidirectional separator: over finder spaces alternate dark/light,
/// starting dark after each bar (left to right), light over bars.
fn omn_finder_adjust(sep: &mut [bool], row: &[bool], start: usize) {
    let mut latch = true;
    for i in start..start + 13 {
        if !row[i] {
            sep[i] = latch;
            latch = !latch;
        } else {
            sep[i] = false;
            latch = true;
        }
    }
}

fn omn_separator(
    sep: &mut [bool],
    row: &[bool],
    width: usize,
    finders: &[usize],
    finder_value_3: bool,
) {
    for i in 4..width - 4 {
        sep[i] = !row[i];
    }
    if finder_value_3 {
        // ISO/IEC 24724:2011 5.3.2.2: the single dark module over finder value 3 is shifted
        // to be over the start of the three module wide finder bar.
        let start = finders[0];
        for (i, m) in sep[start..start + 13].iter_mut().enumerate() {
            *m = i == 10;
        }
    } else {
        for &f in finders {
            omn_finder_adjust(sep, row, f);
        }
    }
}

fn omni(kind: DataBarKind, data: &str, linked: bool) -> Result<DataBar, SymbolError> {
    let digits = parse_gtin(data)?;
    let (tw, c_right) = omn_widths(&digits, linked);
    let hri = Some(gtin_hri(&digits));

    let rows = match kind {
        DataBarKind::Omnidirectional | DataBarKind::Truncated => {
            let mut row = Vec::with_capacity(96);
            expand(&mut row, false, &tw);
            let weight = if kind == DataBarKind::Truncated {
                13
            } else {
                33
            };
            vec![bar_row(row, weight)]
        }
        _ => {
            // Top row: left half + right guard (bar, space); bottom row: (bar, space) + right half.
            let mut top = Vec::with_capacity(50);
            expand(&mut top, false, &tw[..23]);
            top.extend([true, false]);
            let mut bottom = Vec::with_capacity(50);
            bottom.extend([true, false]);
            expand(&mut bottom, true, &tw[23..]);
            debug_assert_eq!((top.len(), bottom.len()), (50, 50));

            if kind == DataBarKind::Stacked {
                // ISO/IEC 24724:2011 5.3.2.1 single row separator (zint #183 interpretation).
                let mut sep = vec![false; 50];
                for i in 1..46 {
                    sep[i] = if top[i] == bottom[i] {
                        !top[i]
                    } else {
                        !sep[i - 1]
                    };
                }
                sep[1] = false;
                sep[2] = false;
                sep[3] = false;
                vec![bar_row(top, 5), sep_row(sep), bar_row(bottom, 7)]
            } else {
                let mut middle = vec![false; 50];
                for i in (5..46).step_by(2) {
                    middle[i] = true;
                }
                let mut sep_top = vec![false; 50];
                omn_separator(&mut sep_top, &top, 50, &[18], false);
                let mut sep_bottom = vec![false; 50];
                // 17 = guard (2) + inner character (15); +2 skips finder elements 4 & 5.
                omn_separator(&mut sep_bottom, &bottom, 50, &[19], c_right == 3);
                vec![
                    bar_row(top, 33),
                    sep_row(sep_top),
                    sep_row(middle),
                    sep_row(sep_bottom),
                    bar_row(bottom, 33),
                ]
            }
        }
    };
    Ok(DataBar {
        symbol: Stacked {
            width: rows[0].modules.len() as u32,
            rows,
        },
        hri,
    })
}

// ---------------------------------------------------------------------------------------------
// Limited

const LTD_G_SUM: [i32; 7] = [0, 183064, 820064, 1000776, 1491021, 1979845, 1996939];
const LTD_T_EVEN: [i32; 7] = [28, 728, 6454, 203, 2408, 1, 16632];
const LTD_MODULES: [i32; 7] = [17, 13, 9, 15, 11, 19, 7];
const LTD_WIDEST: [i32; 7] = [6, 5, 3, 5, 4, 8, 1];
const LTD_WEIGHT: [[i32; 14]; 2] = [
    [1, 3, 9, 27, 81, 65, 17, 51, 64, 14, 42, 37, 22, 66],
    [20, 60, 2, 6, 18, 54, 73, 41, 34, 13, 39, 28, 84, 74],
];
/// ISO/IEC 24724:2011 Annex C check character patterns.
const LTD_FINDER: [[u8; 14]; 89] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 3, 2, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 3, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 3, 2, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 3, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 3, 1, 1, 3, 1, 1, 1],
    [1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 3, 2, 1, 1],
    [1, 1, 1, 1, 1, 2, 1, 1, 1, 2, 3, 1, 1, 1],
    [1, 1, 1, 1, 1, 2, 1, 2, 1, 1, 3, 1, 1, 1],
    [1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 3, 1, 1, 1],
    [1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 3, 2, 1, 1],
    [1, 1, 1, 2, 1, 1, 1, 1, 1, 2, 3, 1, 1, 1],
    [1, 1, 1, 2, 1, 1, 1, 2, 1, 1, 3, 1, 1, 1],
    [1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 3, 1, 1, 1],
    [1, 1, 1, 3, 1, 1, 1, 1, 1, 1, 3, 1, 1, 1],
    [1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 3, 2, 1, 1],
    [1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 3, 1, 1, 1],
    [1, 2, 1, 1, 1, 1, 1, 2, 1, 1, 3, 1, 1, 1],
    [1, 2, 1, 1, 1, 2, 1, 1, 1, 1, 3, 1, 1, 1],
    [1, 2, 1, 2, 1, 1, 1, 1, 1, 1, 3, 1, 1, 1],
    [1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 3, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 3, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 2, 3, 2, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 2, 2, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 3, 2, 1, 2, 1, 1, 1],
    [1, 1, 1, 1, 1, 2, 1, 1, 2, 1, 2, 2, 1, 1],
    [1, 1, 1, 1, 1, 2, 1, 1, 2, 2, 2, 1, 1, 1],
    [1, 1, 1, 1, 1, 2, 1, 2, 2, 1, 2, 1, 1, 1],
    [1, 1, 1, 1, 1, 3, 1, 1, 2, 1, 2, 1, 1, 1],
    [1, 1, 1, 2, 1, 1, 1, 1, 2, 1, 2, 2, 1, 1],
    [1, 1, 1, 2, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1],
    [1, 1, 1, 2, 1, 1, 1, 2, 2, 1, 2, 1, 1, 1],
    [1, 1, 1, 2, 1, 2, 1, 1, 2, 1, 2, 1, 1, 1],
    [1, 1, 1, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1],
    [1, 2, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 1, 1],
    [1, 2, 1, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1, 1],
    [1, 2, 1, 1, 1, 1, 1, 2, 2, 1, 2, 1, 1, 1],
    [1, 2, 1, 1, 1, 2, 1, 1, 2, 1, 2, 1, 1, 1],
    [1, 2, 1, 2, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1],
    [1, 3, 1, 1, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 3, 1, 1, 3, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 3, 2, 1, 2, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 2, 3, 1, 1, 2, 1, 1],
    [1, 1, 1, 2, 1, 1, 1, 1, 3, 1, 1, 2, 1, 1],
    [1, 2, 1, 1, 1, 1, 1, 1, 3, 1, 1, 2, 1, 1],
    [1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 2, 3, 1, 1],
    [1, 1, 1, 1, 1, 1, 2, 1, 1, 2, 2, 2, 1, 1],
    [1, 1, 1, 1, 1, 1, 2, 1, 1, 3, 2, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 2, 2, 1, 1],
    [1, 1, 1, 2, 1, 1, 2, 1, 1, 1, 2, 2, 1, 1],
    [1, 1, 1, 2, 1, 1, 2, 1, 1, 2, 2, 1, 1, 1],
    [1, 1, 1, 2, 1, 1, 2, 2, 1, 1, 2, 1, 1, 1],
    [1, 1, 1, 2, 1, 2, 2, 1, 1, 1, 2, 1, 1, 1],
    [1, 1, 1, 3, 1, 1, 2, 1, 1, 1, 2, 1, 1, 1],
    [1, 2, 1, 1, 1, 1, 2, 1, 1, 1, 2, 2, 1, 1],
    [1, 2, 1, 1, 1, 1, 2, 1, 1, 2, 2, 1, 1, 1],
    [1, 2, 1, 2, 1, 1, 2, 1, 1, 1, 2, 1, 1, 1],
    [1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 2, 3, 1, 1],
    [1, 1, 1, 1, 2, 1, 1, 1, 1, 2, 2, 2, 1, 1],
    [1, 1, 1, 1, 2, 1, 1, 1, 1, 3, 2, 1, 1, 1],
    [1, 1, 1, 1, 2, 1, 1, 2, 1, 1, 2, 2, 1, 1],
    [1, 1, 1, 1, 2, 1, 1, 2, 1, 2, 2, 1, 1, 1],
    [1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 2, 2, 1, 1],
    [1, 2, 1, 1, 2, 1, 1, 1, 1, 1, 2, 2, 1, 1],
    [1, 2, 1, 1, 2, 1, 1, 1, 1, 2, 2, 1, 1, 1],
    [1, 2, 1, 1, 2, 1, 1, 2, 1, 1, 2, 1, 1, 1],
    [1, 2, 1, 1, 2, 2, 1, 1, 1, 1, 2, 1, 1, 1],
    [1, 2, 1, 2, 2, 1, 1, 1, 1, 1, 2, 1, 1, 1],
    [1, 3, 1, 1, 2, 1, 1, 1, 1, 1, 2, 1, 1, 1],
    [1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 3, 1, 1],
    [1, 1, 2, 1, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1],
    [1, 1, 2, 1, 1, 1, 1, 1, 1, 3, 2, 1, 1, 1],
    [1, 1, 2, 1, 1, 1, 1, 2, 1, 1, 2, 2, 1, 1],
    [1, 1, 2, 1, 1, 1, 1, 2, 1, 2, 2, 1, 1, 1],
    [1, 1, 2, 1, 1, 1, 1, 3, 1, 1, 2, 1, 1, 1],
    [1, 1, 2, 1, 1, 2, 1, 1, 1, 1, 2, 2, 1, 1],
    [1, 1, 2, 1, 1, 2, 1, 1, 1, 2, 2, 1, 1, 1],
    [1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1],
    [2, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 1, 1],
    [2, 1, 1, 1, 1, 1, 1, 1, 1, 3, 2, 1, 1, 1],
    [2, 1, 1, 1, 1, 1, 1, 2, 1, 1, 2, 2, 1, 1],
    [2, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 1, 1, 1],
    [2, 1, 1, 1, 1, 1, 1, 3, 1, 1, 2, 1, 1, 1],
    [2, 1, 1, 1, 1, 2, 1, 1, 1, 2, 2, 1, 1, 1],
    [2, 1, 1, 1, 1, 2, 1, 2, 1, 1, 2, 1, 1, 1],
    [2, 1, 1, 2, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1],
    [2, 1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 2, 1, 1],
];

fn limited(data: &str, linked: bool) -> Result<DataBar, SymbolError> {
    let digits = parse_gtin(data)?;
    if digits[0] > 1 {
        return Err(SymbolError::InvalidData(
            "GS1 DataBar Limited GTIN must start with 0 or 1".into(),
        ));
    }
    let mut val: u64 = digits.iter().fold(0, |a, &d| a * 10 + d as u64);
    if linked {
        val += 2_015_133_531_096;
    }
    let mut pair = [(val / 2_013_571) as i32, (val % 2_013_571) as i32];
    let mut pw = [[0u8; 14]; 2];
    for i in 0..2 {
        let group = (0..7).rev().find(|&g| pair[i] >= LTD_G_SUM[g]).unwrap_or(0);
        pair[i] -= LTD_G_SUM[group];
        pw[i] = char_widths(
            pair[i] / LTD_T_EVEN[group],
            pair[i] % LTD_T_EVEN[group],
            LTD_MODULES[group],
            26 - LTD_MODULES[group],
            7,
            LTD_WIDEST[group],
            false,
        );
    }
    let mut checksum = 0;
    for i in 0..14 {
        checksum += LTD_WEIGHT[0][i] * pw[0][i] as i32 + LTD_WEIGHT[1][i] * pw[1][i] as i32;
    }
    let check = &LTD_FINDER[(checksum % 89) as usize];

    let mut tw = [0u8; 47];
    tw[0] = 1;
    tw[1] = 1;
    tw[44] = 1;
    tw[45] = 1;
    tw[46] = 5; // trailing 5X space, ISO/IEC 24724:2011
    tw[2..16].copy_from_slice(&pw[0]);
    tw[16..30].copy_from_slice(check);
    tw[30..44].copy_from_slice(&pw[1]);
    let mut row = Vec::with_capacity(79);
    expand(&mut row, false, &tw);
    Ok(DataBar {
        symbol: Stacked {
            width: row.len() as u32,
            rows: vec![bar_row(row, 10)],
        },
        hri: Some(gtin_hri(&digits)),
    })
}

// ---------------------------------------------------------------------------------------------
// Expanded / Expanded Stacked

const EXP_G_SUM: [i32; 5] = [0, 348, 1388, 2948, 3988];
const EXP_T_EVEN: [i32; 5] = [4, 20, 52, 104, 204];
const EXP_MODULES: [i32; 5] = [12, 10, 8, 6, 4];
const EXP_WIDEST: [i32; 5] = [7, 5, 4, 3, 1];
const EXP_WEIGHT: [[u8; 8]; 23] = [
    [1, 3, 9, 27, 81, 32, 96, 77],
    [20, 60, 180, 118, 143, 7, 21, 63],
    [189, 145, 13, 39, 117, 140, 209, 205],
    [193, 157, 49, 147, 19, 57, 171, 91],
    [62, 186, 136, 197, 169, 85, 44, 132],
    [185, 133, 188, 142, 4, 12, 36, 108],
    [113, 128, 173, 97, 80, 29, 87, 50],
    [150, 28, 84, 41, 123, 158, 52, 156],
    [46, 138, 203, 187, 139, 206, 196, 166],
    [76, 17, 51, 153, 37, 111, 122, 155],
    [43, 129, 176, 106, 107, 110, 119, 146],
    [16, 48, 144, 10, 30, 90, 59, 177],
    [109, 116, 137, 200, 178, 112, 125, 164],
    [70, 210, 208, 202, 184, 130, 179, 115],
    [134, 191, 151, 31, 93, 68, 204, 190],
    [148, 22, 66, 198, 172, 94, 71, 2],
    [6, 18, 54, 162, 64, 192, 154, 40],
    [120, 149, 25, 75, 14, 42, 126, 167],
    [79, 26, 78, 23, 69, 207, 199, 175],
    [103, 98, 83, 38, 114, 131, 182, 124],
    [161, 61, 183, 127, 170, 88, 53, 159],
    [55, 165, 73, 8, 24, 72, 5, 15],
    [45, 135, 194, 160, 58, 174, 100, 89],
];
const EXP_FINDER: [[u8; 5]; 12] = [
    [1, 8, 4, 1, 1],
    [1, 1, 4, 8, 1],
    [3, 6, 4, 1, 1],
    [1, 1, 4, 6, 3],
    [3, 4, 6, 1, 1],
    [1, 1, 6, 4, 3],
    [3, 2, 8, 1, 1],
    [1, 1, 8, 2, 3],
    [2, 6, 5, 1, 1],
    [1, 1, 5, 6, 2],
    [2, 2, 9, 1, 1],
    [1, 1, 9, 2, 2],
];
const EXP_FINDER_SEQ: [[u8; 11]; 10] = [
    [1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 4, 3, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 6, 3, 8, 0, 0, 0, 0, 0, 0, 0],
    [1, 10, 3, 8, 5, 0, 0, 0, 0, 0, 0],
    [1, 10, 3, 8, 7, 12, 0, 0, 0, 0, 0],
    [1, 10, 3, 8, 9, 12, 11, 0, 0, 0, 0],
    [1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0],
    [1, 2, 3, 4, 5, 6, 7, 10, 9, 0, 0],
    [1, 2, 3, 4, 5, 6, 7, 10, 11, 12, 0],
    [1, 2, 3, 4, 5, 8, 7, 10, 9, 12, 11],
];
const EXP_WEIGHT_ROWS: [[u8; 21]; 10] = [
    [
        0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 5, 6, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 9, 10, 3, 4, 13, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 17, 18, 3, 4, 13, 14, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 17, 18, 3, 4, 13, 14, 11, 12, 21, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 17, 18, 3, 4, 13, 14, 15, 16, 21, 22, 19, 20, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 17, 18, 15, 16, 0, 0, 0, 0,
    ],
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 17, 18, 19, 20, 21, 22, 0, 0,
    ],
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 13, 14, 11, 12, 17, 18, 15, 16, 21, 22, 19, 20,
    ],
];

const FNC1: u8 = 0x1D;

/// Number of digits of the AI starting `s` (GS1 General Specifications AI ranges).
fn ai_digits(s: &[u8]) -> usize {
    match (s.first(), s.get(1)) {
        (Some(a), Some(b)) => match (a, b) {
            (b'2', b'3'..=b'5') | (b'4', b'0'..=b'2') | (b'7', b'1') => 3,
            (b'3', b'1'..=b'6' | b'9') | (b'4', b'3') | (b'7', _) | (b'8', _) => 4,
            _ => 2,
        },
        _ => 2,
    }
}

/// Predefined (fixed) data length of AIs that need no FNC1 terminator, excluding the AI.
fn predefined_len(ai: &[u8]) -> Option<usize> {
    Some(match (ai[0], ai[1]) {
        (b'0', b'0') => 18,
        (b'0', b'1'..=b'3') => 14,
        (b'0', b'4') => 16,
        (b'1', b'1'..=b'9') => 6,
        (b'2', b'0') => 2,
        (b'3', b'1'..=b'6') => 6,
        (b'4', b'1') => 13,
        _ => return None,
    })
}

/// Turn the ^FD content into the reduced GS1 string (FNC1 as 0x1D) and its HRI.
fn parse_gs1(data: &str) -> Result<(Vec<u8>, String), SymbolError> {
    let s = data.as_bytes();
    let mut reduced = Vec::with_capacity(s.len());
    let mut hri = String::with_capacity(s.len() + 8);
    if matches!(s.first(), Some(b'(' | b'[')) {
        let mut i = 0;
        while i < s.len() {
            let close = match s[i] {
                b'(' => b')',
                b'[' => b']',
                c => return Err(SymbolError::InvalidCharacter(c as char)),
            };
            let end = s[i + 1..]
                .iter()
                .position(|&c| c == close)
                .ok_or_else(|| SymbolError::InvalidData("unterminated AI".into()))?
                + i
                + 1;
            let ai = &s[i + 1..end];
            if !(2..=4).contains(&ai.len()) || !ai.iter().all(u8::is_ascii_digit) {
                return Err(SymbolError::InvalidData("invalid AI".into()));
            }
            let vstart = end + 1;
            let vend = s[vstart..]
                .iter()
                .position(|&c| c == b'(' || c == b'[')
                .map_or(s.len(), |p| p + vstart);
            let value = &s[vstart..vend];
            if value.is_empty() {
                return Err(SymbolError::InvalidData("empty AI data".into()));
            }
            reduced.extend_from_slice(ai);
            reduced.extend_from_slice(value);
            if vend < s.len() && predefined_len(ai).is_none() {
                reduced.push(FNC1);
            }
            hri.push('(');
            hri.push_str(std::str::from_utf8(ai).unwrap_or_default());
            hri.push(')');
            hri.push_str(
                std::str::from_utf8(value).map_err(|_| SymbolError::InvalidData("utf8".into()))?,
            );
            i = vend;
        }
    } else {
        // Raw element string; GS (0x1D) terminates variable-length fields.
        let mut raw: &[u8] = s;
        while raw.first() == Some(&FNC1) {
            raw = &raw[1..];
        }
        reduced.extend_from_slice(raw);
        while reduced.last() == Some(&FNC1) {
            reduced.pop();
        }
        // Best effort HRI.
        let mut i = 0;
        while i < reduced.len() {
            let n = ai_digits(&reduced[i..]).min(reduced.len() - i);
            let ai = &reduced[i..i + n];
            let vstart = i + n;
            let vend = match (ai.len() >= 2).then(|| predefined_len(ai)).flatten() {
                Some(len) => (vstart + len).min(reduced.len()),
                None => reduced[vstart..]
                    .iter()
                    .position(|&c| c == FNC1)
                    .map_or(reduced.len(), |p| p + vstart),
            };
            hri.push('(');
            hri.extend(ai.iter().map(|&c| c as char));
            hri.push(')');
            hri.extend(reduced[vstart..vend].iter().map(|&c| c as char));
            i = vend;
            if reduced.get(i) == Some(&FNC1) {
                i += 1;
            }
        }
    }
    if reduced.is_empty() {
        return Err(SymbolError::InvalidLength(0));
    }
    if reduced.len() > 77 {
        return Err(SymbolError::InvalidLength(reduced.len()));
    }
    if let Some(&c) = reduced.iter().find(|&&c| gf_type(c) == 0) {
        return Err(SymbolError::InvalidCharacter(c as char));
    }
    Ok((reduced, hri))
}

struct Bits {
    v: Vec<u8>,
}

impl Bits {
    fn push(&mut self, val: u32, n: u32) {
        for i in (0..n).rev() {
            self.v.push(((val >> i) & 1) as u8);
        }
    }
    fn len(&self) -> usize {
        self.v.len()
    }
}

const GF_NUMERIC: u8 = 1;
const GF_ALNUM: u8 = 2;
const GF_ISO: u8 = 3;
const ALNUM_PUNCS: &[u8] = b"*,-./";
const ISO_PUNCS: &[u8] = b"!\"%&'()*+,-./:;<=>?_ ";

fn gf_type(c: u8) -> u8 {
    if c == FNC1 || c.is_ascii_digit() {
        GF_NUMERIC
    } else if c.is_ascii_uppercase() || ALNUM_PUNCS.contains(&c) {
        GF_ALNUM
    } else if c.is_ascii_lowercase() || ISO_PUNCS.contains(&c) {
        GF_ISO
    } else {
        0
    }
}

fn gf_next(f: &[u8], i: usize, num: usize, t: u8, t2: u8) -> bool {
    i + num <= f.len()
        && f[i..i + num]
            .iter()
            .all(|&c| gf_type(c) == t || (t2 != 0 && gf_type(c) == t2))
}

/// The remaining chars (at least `num`, at most `max_num`) are all of type `t`.
fn gf_next_terminate(f: &[u8], i: usize, num: usize, max_num: usize, t: u8) -> bool {
    let rest = &f[i..];
    rest.len() <= max_num && rest.len() >= num && rest.iter().all(|&c| gf_type(c) == t)
}

fn gf_next_none(f: &[u8], i: usize, num: usize, t: u8) -> bool {
    f[i..].iter().take(num).all(|&c| gf_type(c) != t)
}

fn digit_val(c: u8) -> u32 {
    if c == FNC1 { 10 } else { (c - b'0') as u32 }
}

/// General purpose encodation (ISO/IEC 24724:2011 7.2.5.5). Returns (final mode, odd last digit).
fn general_field(f: &[u8], bits: &mut Bits) -> (u8, Option<u8>) {
    let mut mode = GF_NUMERIC;
    let mut last_digit = None;
    let mut i = 0;
    while i < f.len() {
        let c = f[i];
        let t = gf_type(c);
        match mode {
            GF_NUMERIC => {
                if i + 1 < f.len() {
                    if t != GF_NUMERIC || gf_type(f[i + 1]) != GF_NUMERIC {
                        bits.push(0, 4);
                        mode = GF_ALNUM;
                    } else {
                        bits.push(11 * digit_val(c) + digit_val(f[i + 1]) + 8, 7);
                        i += 2;
                    }
                } else if t != GF_NUMERIC {
                    bits.push(0, 4);
                    mode = GF_ALNUM;
                } else {
                    last_digit = Some(c);
                    i += 1;
                }
            }
            GF_ALNUM => {
                if c == FNC1 {
                    bits.push(15, 5);
                    mode = GF_NUMERIC;
                    i += 1;
                } else if t == GF_ISO {
                    bits.push(4, 5);
                    mode = GF_ISO;
                } else if gf_next(f, i, 6, GF_NUMERIC, 0)
                    || gf_next_terminate(f, i, 4, 5, GF_NUMERIC)
                {
                    bits.push(0, 3);
                    mode = GF_NUMERIC;
                } else if c.is_ascii_digit() {
                    bits.push((c - 43) as u32, 5);
                    i += 1;
                } else if c.is_ascii_uppercase() {
                    bits.push((c - 33) as u32, 6);
                    i += 1;
                } else {
                    let p = ALNUM_PUNCS.iter().position(|&x| x == c).unwrap_or(0);
                    bits.push(p as u32 + 58, 6);
                    i += 1;
                }
            }
            _ => {
                if c == FNC1 {
                    bits.push(15, 5);
                    mode = GF_NUMERIC;
                    i += 1;
                } else {
                    let none_iso = gf_next_none(f, i, 10, GF_ISO);
                    if none_iso && gf_next(f, i, 4, GF_NUMERIC, 0) {
                        bits.push(0, 3);
                        mode = GF_NUMERIC;
                    } else if none_iso && gf_next(f, i, 5, GF_ALNUM, GF_NUMERIC) {
                        bits.push(4, 5);
                        mode = GF_ALNUM;
                    } else if c.is_ascii_digit() {
                        bits.push((c - 43) as u32, 5);
                        i += 1;
                    } else if c.is_ascii_uppercase() {
                        bits.push((c - 1) as u32, 7);
                        i += 1;
                    } else if c.is_ascii_lowercase() {
                        bits.push((c - 7) as u32, 7);
                        i += 1;
                    } else {
                        let p = ISO_PUNCS.iter().position(|&x| x == c).unwrap_or(0);
                        bits.push(p as u32 + 232, 8);
                        i += 1;
                    }
                }
            }
        }
    }
    (mode, last_digit)
}

fn to_int(s: &[u8], start: usize, len: usize) -> Option<u32> {
    let d = s.get(start..start + len)?;
    d.iter()
        .all(u8::is_ascii_digit)
        .then(|| d.iter().fold(0, |a, &c| a * 10 + (c - b'0') as u32))
}

fn exp_date(s: &[u8], pos: usize) -> Option<u32> {
    let (yy, mm, dd) = (
        to_int(s, pos, 2)?,
        to_int(s, pos + 2, 2)?,
        to_int(s, pos + 4, 2)?,
    );
    ((1..=12).contains(&mm) && dd <= 31).then_some(yy * 384 + (mm - 1) * 32 + dd)
}

/// Encoding method (ISO/IEC 24724:2011 Table 10) for the reduced string.
fn exp_method(s: &[u8]) -> u32 {
    let len = s.len();
    // Compressed methods drop the GTIN check digit, so only use them for a valid one.
    let gtin_ok = len >= 16 && s.starts_with(b"01") && s[2..16].iter().all(u8::is_ascii_digit) && {
        let d: Vec<u8> = s[2..15].iter().map(|c| c - b'0').collect();
        gs1_check_digit(&d) == s[15] - b'0'
    };
    if !gtin_ok {
        return 2;
    }
    if len >= 20 && s[2] == b'9' && s[16] == b'3' {
        let date_ai = len == 34
            && s[26] == b'1'
            && matches!(s[27], b'1' | b'3' | b'5' | b'7')
            && exp_date(s, 28).is_some();
        if len >= 26 && (s[17..19] == *b"10" || s[17..19] == *b"20") && s[19].is_ascii_digit() {
            if let Some(weight) = to_int(s, 20, 6).filter(|&w| w <= 99999) {
                let metric = s[17] == b'1';
                if len == 26 {
                    return if metric {
                        if s[19] == b'3' && weight <= 32767 {
                            3
                        } else {
                            7
                        }
                    } else if (s[19] == b'2' && weight <= 9999)
                        || (s[19] == b'3' && weight <= 22767)
                    {
                        4
                    } else {
                        8
                    };
                } else if date_ai {
                    return if metric { 6 } else { 7 } + (s[27] - b'0') as u32;
                }
            }
        } else if s[17] == b'9' && (b'0'..=b'3').contains(&s[19]) {
            if s[18] == b'2' {
                return 5;
            } else if s[18] == b'3' && to_int(s, 20, 3).is_some() {
                return 6;
            }
        }
    }
    1
}

/// Builds the Expanded bit stream. `cols_per_row` = segment pairs per row (0 = not stacked).
/// Returns (bits, symbol characters).
fn exp_bits(s: &[u8], linked: bool, cols_per_row: usize) -> Result<Vec<u8>, SymbolError> {
    let mut bits = Bits {
        v: Vec::with_capacity(264),
    };
    bits.push(linked as u32, 1);
    let method = exp_method(s);
    let read_posn = match method {
        1 => {
            bits.push(4, 3);
            16
        }
        2 => {
            bits.push(0, 4);
            0
        }
        3 | 4 => {
            bits.push(4 + method - 3, 4);
            26
        }
        5 => {
            bits.push(0x30, 7);
            20
        }
        6 => {
            bits.push(0x34, 7);
            23
        }
        _ => {
            bits.push(56 + method - 7, 7);
            s.len()
        }
    };
    let gtin12 = |bits: &mut Bits| {
        for i in (3..15).step_by(3) {
            bits.push(to_int(s, i, 3).unwrap_or(0), 10);
        }
    };
    match method {
        1 => {
            bits.push((s[2] - b'0') as u32, 4);
            gtin12(&mut bits);
        }
        3 | 4 => {
            gtin12(&mut bits);
            let w = to_int(s, 20, 6).unwrap_or(0);
            bits.push(
                if method == 4 && s[19] == b'3' {
                    w + 10000
                } else {
                    w
                },
                15,
            );
        }
        5 | 6 => {
            gtin12(&mut bits);
            bits.push((s[19] - b'0') as u32, 2);
            if method == 6 {
                bits.push(to_int(s, 20, 3).unwrap_or(0), 10);
            }
        }
        7..=14 => {
            gtin12(&mut bits);
            let w = (s[19] - b'0') as u32 * 100_000 + to_int(s, 21, 5).unwrap_or(0);
            bits.push(w, 20);
            bits.push(
                if s.len() == 34 {
                    exp_date(s, 28).unwrap_or(38400)
                } else {
                    38400
                },
                16,
            );
        }
        _ => {}
    }

    let (mode, last_digit) = general_field(&s[read_posn..], &mut bits);
    let chars_per_row = cols_per_row * 2;
    let symbol_chars_for = |bp: usize| {
        let mut n = bp.div_ceil(12) + 1;
        if chars_per_row != 0 && n % chars_per_row == 1 {
            n += 1;
        }
        n.max(4)
    };
    let mut symbol_chars = symbol_chars_for(bits.len());
    if let Some(d) = last_digit {
        let remainder = 12 * (symbol_chars - 1) - bits.len();
        if (4..=6).contains(&remainder) {
            bits.push((d - b'0') as u32 + 1, 4);
        } else {
            bits.push((d - b'0') as u32 * 11 + 10 + 8, 7);
        }
        symbol_chars = symbol_chars_for(bits.len());
    }
    if bits.len() > 252 {
        return Err(SymbolError::InvalidLength(s.len()));
    }
    // Padding (7.2.5.5.4).
    let target = 12 * (symbol_chars - 1);
    if mode == GF_NUMERIC && bits.len() < target {
        bits.push(0, 4);
    }
    while bits.len() < target {
        bits.push(4, 5);
    }
    bits.v.truncate(target);
    // Variable length symbol field.
    let pos = match method {
        1 => Some(2),
        2 => Some(3),
        5 | 6 => Some(6),
        _ => None,
    };
    if let Some(p) = pos {
        bits.v[p] = (symbol_chars & 1) as u8;
        bits.v[p + 1] = (symbol_chars > 14) as u8;
    }
    Ok(bits.v)
}

fn exp_group(v: i32) -> usize {
    (0..4).find(|&i| v < EXP_G_SUM[i + 1]).unwrap_or(4)
}

fn exp_char(v: i32) -> [u8; 8] {
    let g = exp_group(v);
    let w = char_widths(
        (v - EXP_G_SUM[g]) / EXP_T_EVEN[g],
        (v - EXP_G_SUM[g]) % EXP_T_EVEN[g],
        EXP_MODULES[g],
        17 - EXP_MODULES[g],
        4,
        EXP_WIDEST[g],
        true,
    );
    let mut out = [0u8; 8];
    out.copy_from_slice(&w[..8]);
    out
}

/// Width of `row` up to the last dark module plus its trailing light guard module.
fn row_used_width(row: &[bool], width: usize) -> usize {
    row.iter()
        .rposition(|&m| m)
        .map_or(width, |p| (p + 2).min(width))
}

/// Separator for Expanded Stacked rows and Expanded composites (zint `dbar_exp_separator`).
/// `row` is the adjacent bar row, `width` its used width, `cols` its segment pairs.
#[allow(clippy::too_many_arguments)]
fn exp_separator(
    sep: &mut [bool],
    row: &[bool],
    width: usize,
    cols: usize,
    special_case_row: usize,
    left_to_right: bool,
    odd_last_row: bool,
    v2_latch: &mut bool,
) {
    let at = |i: usize| row.get(i).copied().unwrap_or(false);
    let end = width.saturating_sub(4).min(sep.len());
    for (j, m) in sep
        .iter_mut()
        .enumerate()
        .take(end)
        .skip(4 + special_case_row)
    {
        *m = !at(j);
    }
    let mut v2 = *v2_latch;
    let mut space_latch = false;
    let apply = |sep: &mut [bool], i: usize, space_latch: &mut bool| {
        if i >= sep.len() {
            return;
        }
        if at(i) {
            sep[i] = false;
            *space_latch = false;
        } else {
            sep[i] = !*space_latch;
            *space_latch = !*space_latch;
        }
    };
    for j in 0..cols {
        let mut k = 49 * j + 19 + special_case_row;
        if left_to_right {
            let (s, e) = if v2 { (2, 15) } else { (0, 13) };
            for i in s..e {
                apply(sep, i + k, &mut space_latch);
            }
        } else {
            if odd_last_row {
                k -= 17;
            }
            let (s, e) = if v2 { (14, 2) } else { (12, 0) };
            for i in (e..=s).rev() {
                apply(sep, i + k, &mut space_latch);
            }
        }
        v2 = !v2;
    }
    *v2_latch = v2;
}

fn expanded(data: &str, segments_per_row: u32, linked: bool) -> Result<DataBar, SymbolError> {
    let (reduced, hri) = parse_gs1(data)?;
    let stacked = (1..22).contains(&segments_per_row);
    let mut cols_per_row = if stacked {
        (segments_per_row as usize / 2).max(1)
    } else {
        0
    };
    if linked && cols_per_row == 1 {
        // ISO/IEC 24724: at least four symbol characters in the first row of a composite.
        cols_per_row = 2;
    }
    let bits = exp_bits(&reduced, linked, cols_per_row)?;

    let data_chars = bits.len() / 12;
    let symbol_chars = data_chars + 1;
    let mut cw = [[0u8; 8]; 21];
    for (i, chunk) in bits.chunks(12).enumerate() {
        let v = chunk.iter().fold(0i32, |a, &b| (a << 1) | b as i32);
        cw[i] = exp_char(v);
    }
    let mut checksum = 0u32;
    let wrow = &EXP_WEIGHT_ROWS[(data_chars - 2) / 2];
    for i in 0..data_chars {
        let weights = &EXP_WEIGHT[wrow[i] as usize];
        for j in 0..8 {
            checksum += cw[i][j] as u32 * weights[j] as u32;
        }
    }
    let check_char = 211 * (symbol_chars as i32 - 4) + (checksum % 211) as i32;
    let check_widths = exp_char(check_char);

    let codeblocks = symbol_chars.div_ceil(2);
    let pattern_width = codeblocks * 5 + symbol_chars * 8 + 4;
    let mut elements = [0u8; 235];
    let seq = &EXP_FINDER_SEQ[(symbol_chars - 1) / 2 - 1];
    for i in 0..codeblocks {
        let k = seq[i] as usize - 1;
        elements[21 * i + 10..21 * i + 15].copy_from_slice(&EXP_FINDER[k]);
    }
    elements[2..10].copy_from_slice(&check_widths);
    for i in (1..data_chars).step_by(2) {
        let k = (i - 1) / 2 * 21 + 23;
        elements[k..k + 8].copy_from_slice(&cw[i]);
    }
    for i in (0..data_chars).step_by(2) {
        let k = i / 2 * 21 + 15;
        for j in 0..8 {
            elements[k + j] = cw[i][7 - j];
        }
    }

    if !stacked {
        elements[0] = 1;
        elements[1] = 1;
        elements[pattern_width - 2] = 1;
        elements[pattern_width - 1] = 1;
        let mut row = Vec::with_capacity(560);
        expand(&mut row, false, &elements[..pattern_width]);
        return Ok(DataBar {
            symbol: Stacked {
                width: row.len() as u32,
                rows: vec![bar_row(row, 34)],
            },
            hri: Some(hri),
        });
    }

    let stack_rows = codeblocks.div_ceil(cols_per_row);
    let max_width = 4 + 49 * cols_per_row + 2;
    let mut grid: Vec<Vec<bool>> = vec![vec![false; max_width]; 4 * stack_rows - 3];
    let mut width = 0;
    let mut current_block = 0;
    let mut v2_latch = false;
    for current_row in 1..=stack_rows {
        let mut sub = [0u8; 235];
        let mut n_sub = 2;
        let num_columns = if current_row * cols_per_row > codeblocks {
            codeblocks - current_block
        } else {
            cols_per_row
        };
        sub[0] = 1;
        sub[1] = 1;
        let mut special_case_row = 0;
        if current_row == stack_rows
            && num_columns != cols_per_row
            && current_row & 1 == 0
            && cols_per_row & 1 == 0
            && num_columns & 1 == 1
        {
            special_case_row = 1;
            sub[0] = 2;
        }
        let left_to_right = cols_per_row & 1 == 1 || current_row & 1 == 1 || special_case_row == 1;
        let mut reader = 0;
        loop {
            let i = 2 + current_block * 21;
            for j in 0..21 {
                if i + j < pattern_width {
                    if left_to_right {
                        sub[j + reader * 21 + 2] = elements[i + j];
                    } else {
                        sub[(20 - j) + (num_columns - 1 - reader) * 21 + 2] = elements[i + j];
                    }
                }
                n_sub += 1;
            }
            reader += 1;
            current_block += 1;
            if reader >= cols_per_row || current_block >= codeblocks {
                break;
            }
        }
        sub[n_sub] = 1;
        sub[n_sub + 1] = 1;
        n_sub += 2;

        let start_dark = !(current_row & 1 == 1 || special_case_row == 1);
        let r = 4 * (current_row - 1);
        let mut row = Vec::with_capacity(max_width);
        expand(&mut row, start_dark, &sub[..n_sub]);
        let writer = row.len();
        width = width.max(writer);
        grid[r][..writer].copy_from_slice(&row);

        if current_row != 1 {
            let odd_last_row = current_row == stack_rows && data_chars % 2 == 0;
            for j in (5..49 * cols_per_row).step_by(2) {
                grid[r - 2][j] = true;
            }
            let (above, cur) = grid.split_at_mut(r);
            // Separator directly above the current row; the v2 latch is not carried on.
            let mut latch = v2_latch;
            exp_separator(
                &mut above[r - 1],
                &cur[0],
                writer,
                reader,
                special_case_row,
                left_to_right,
                odd_last_row,
                &mut latch,
            );
        }
        if current_row != stack_rows {
            let (cur, below) = grid.split_at_mut(r + 1);
            exp_separator(
                &mut below[0],
                &cur[r],
                writer,
                reader,
                0,
                left_to_right,
                false,
                &mut v2_latch,
            );
        }
    }
    let rows = grid
        .into_iter()
        .enumerate()
        .map(|(i, mut m)| {
            m.truncate(width);
            if i % 4 == 0 {
                bar_row(m, 34)
            } else {
                sep_row(m)
            }
        })
        .collect();
    Ok(DataBar {
        symbol: Stacked {
            width: width as u32,
            rows,
        },
        hri: Some(hri),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn widths_of(row: &[bool]) -> Vec<usize> {
        let mut out = vec![];
        let mut cur = row[0];
        let mut n = 0;
        for &m in row {
            if m == cur {
                n += 1;
            } else {
                out.push(n);
                cur = m;
                n = 1;
            }
        }
        out.push(n);
        out
    }

    fn row_str(row: &[bool]) -> String {
        row.iter().map(|&b| if b { '1' } else { '0' }).collect()
    }

    #[test]
    fn databar_get_widths_sums() {
        for v in 0..87 {
            let w = get_widths(v, 12, 4, 7, true);
            assert_eq!(w[..4].iter().map(|&x| x as i32).sum::<i32>(), 12);
        }
    }

    // Reference module patterns produced by zint (via zxing-cpp's writer).
    #[test]
    fn databar_reference_patterns() {
        let omni = encode(DataBarKind::Omnidirectional, "0001234567890", 0).unwrap();
        assert_eq!(
            row_str(&omni.symbol.rows[0].modules),
            "010101001000000001001111111000010111001011011110111001010110000101111111000111001100111101110101"
        );
        let stk = encode(DataBarKind::Stacked, "0001234567890", 0).unwrap();
        let rows: Vec<String> = stk
            .symbol
            .rows
            .iter()
            .map(|r| row_str(&r.modules))
            .collect();
        assert_eq!(
            rows,
            [
                "01010100100000000100111111100001011100101101111010",
                "00001010101011111010000000111010100011010010000000",
                "10111001010110000101111111000111001100111101110101",
            ]
        );
        let so = encode(DataBarKind::StackedOmnidirectional, "0001234567890", 0).unwrap();
        let rows: Vec<String> = so.symbol.rows.iter().map(|r| row_str(&r.modules)).collect();
        assert_eq!(
            rows,
            [
                "01010100100000000100111111100001011100101101111010",
                "00001011011111111010000000010100100011010010000000",
                "00000101010101010101010101010101010101010101010000",
                "00000110101001111010000000101000110011000010000000",
                "10111001010110000101111111000111001100111101110101",
            ]
        );
        let ltd = encode(DataBarKind::Limited, "0001234567890", 0).unwrap();
        assert_eq!(
            row_str(&ltd.symbol.rows[0].modules),
            "0101101011001010000010000001010101101101000101010111001100111010000010110100000"
        );
        let exp = encode(
            DataBarKind::Expanded,
            "(01)98898765432106(3202)012345(15)991231",
            0,
        )
        .unwrap();
        assert_eq!(
            row_str(&exp.symbol.rows[0].modules),
            "01001000011000110110111111110000101110000110010100011010000001100010101111110000111010011100000010010100111110111001100011111100001011101100000100100100011110010110001011111111001110001101111010000101"
        );
    }

    #[test]
    fn databar_input_forms() {
        let a = encode(DataBarKind::Omnidirectional, "0001234567890", 0).unwrap();
        for d in [
            "(01)00012345678905",
            "0100012345678905",
            "00012345678905",
            "1234567890",
        ] {
            assert_eq!(
                encode(DataBarKind::Omnidirectional, d, 0).unwrap(),
                a,
                "{d}"
            );
        }
        assert!(encode(DataBarKind::Omnidirectional, "00012345678906", 0).is_err());
        assert!(encode(DataBarKind::Limited, "2001234567890", 0).is_err());
        assert!(encode(DataBarKind::Omnidirectional, "12A", 0).is_err());
    }

    #[test]
    fn databar_shapes() {
        let t = encode(DataBarKind::Truncated, "0001234567890", 0).unwrap();
        assert_eq!(t.symbol.rows[0].height, RowHeight::Bar { weight: 13 });
        let s = encode(DataBarKind::Stacked, "0001234567890", 0).unwrap();
        assert_eq!(s.symbol.width, 50);
        assert_eq!(s.symbol.rows.len(), 3);
        let so = encode(DataBarKind::StackedOmnidirectional, "0001234567890", 0).unwrap();
        assert_eq!(so.symbol.rows.len(), 5);
        let l = encode(DataBarKind::Limited, "0001234567890", 0).unwrap();
        assert_eq!(l.symbol.width, 79);
        for st in [&s, &so, &l, &t] {
            assert!(
                st.symbol
                    .rows
                    .iter()
                    .all(|r| r.modules.len() == st.symbol.width as usize)
            );
        }
    }

    #[test]
    fn databar_stacked_rows_match_linear() {
        let lin = encode(DataBarKind::Omnidirectional, "0001234567890", 0).unwrap();
        let s = encode(DataBarKind::Stacked, "0001234567890", 0).unwrap();
        let l = &lin.symbol.rows[0].modules;
        assert_eq!(s.symbol.rows[0].modules[..48], l[..48]);
        assert_eq!(s.symbol.rows[2].modules[2..], l[48..]);
    }

    #[test]
    fn databar_limited_widths() {
        let l = encode(DataBarKind::Limited, "1501234567890", 0).unwrap();
        let w = widths_of(&l.symbol.rows[0].modules);
        assert_eq!(w.iter().sum::<usize>(), 79);
    }

    #[test]
    fn databar_expanded_methods() {
        let cases = [
            ("(01)98898765432106(3202)012345(15)991231", 10),
            ("(01)90012345678908(3103)001750", 3),
            ("(01)90012345678908(3922)795", 5),
            ("(01)90012345678908(3932)0401234", 6),
            ("(01)00012345678905(10)ABC123", 1),
            ("(10)12A", 2),
        ];
        for (d, m) in cases {
            let (r, _) = parse_gs1(d).unwrap();
            let m2 = exp_method(&r);
            assert_eq!(m2, if m == 10 { 12 } else { m }, "{d}");
            encode(DataBarKind::Expanded, d, 0).unwrap();
        }
    }

    #[test]
    fn databar_expanded_hri_and_raw() {
        let a = encode(
            DataBarKind::Expanded,
            "(01)98898765432106(10)AB-12(21)XYZ",
            0,
        )
        .unwrap();
        assert_eq!(a.hri.as_deref(), Some("(01)98898765432106(10)AB-12(21)XYZ"));
        let b = encode(DataBarKind::Expanded, "019889876543210610AB-12\x1d21XYZ", 0).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn databar_expanded_stacked_shape() {
        for segs in [2, 4, 6, 8, 10, 20] {
            let s = encode(
                DataBarKind::Expanded,
                "(01)98898765432106(10)ABCDEF123456(21)12345",
                segs,
            )
            .unwrap();
            assert_eq!(s.symbol.rows.len() % 4, 1);
            assert!(
                s.symbol
                    .rows
                    .iter()
                    .all(|r| r.modules.len() == s.symbol.width as usize)
            );
        }
    }

    #[test]
    fn databar_linked_and_composite_separator() {
        let a = encode(DataBarKind::Omnidirectional, "0001234567890", 0).unwrap();
        let b = encode_linked(DataBarKind::Omnidirectional, "0001234567890", 0, true).unwrap();
        assert_ne!(a.symbol, b.symbol);
        for kind in [
            DataBarKind::Omnidirectional,
            DataBarKind::Truncated,
            DataBarKind::Stacked,
            DataBarKind::StackedOmnidirectional,
            DataBarKind::Limited,
        ] {
            let s = encode_linked(kind, "0001234567890", 0, true).unwrap();
            let sep = composite_separator(kind, &s);
            assert_eq!(sep.len(), s.symbol.width as usize);
            assert!(!sep[..4].iter().any(|&m| m));
        }
        // Linked Expanded Stacked needs >= 4 characters in the first row.
        let e = encode_linked(
            DataBarKind::Expanded,
            "(01)98898765432106(10)ABCDEF",
            2,
            true,
        )
        .unwrap();
        assert_eq!(e.symbol.width, 102);
        let sep = composite_separator(DataBarKind::Expanded, &e);
        assert_eq!(sep.len(), 102);
    }

    #[test]
    #[ignore]
    fn databar_speed() {
        let t = std::time::Instant::now();
        for _ in 0..10000 {
            std::hint::black_box(encode(DataBarKind::Omnidirectional, "0001234567890", 0).unwrap());
        }
        let omni = t.elapsed() / 10000;
        let t = std::time::Instant::now();
        for _ in 0..10000 {
            std::hint::black_box(
                encode(
                    DataBarKind::Expanded,
                    "(01)98898765432106(10)ABCDEF123456(21)12345",
                    4,
                )
                .unwrap(),
            );
        }
        eprintln!("omni {:?} expanded stacked {:?}", omni, t.elapsed() / 10000);
    }

    /// Dumps symbols for the zxing-cpp round trip check:
    /// `DATABAR_DUMP=path cargo test -p zpl-symbols databar_dump -- --ignored`
    #[test]
    #[ignore]
    fn databar_dump() {
        use std::fmt::Write;
        let path = std::env::var("DATABAR_DUMP").unwrap();
        let cases: Vec<(DataBarKind, &str, u32)> = vec![
            (DataBarKind::Omnidirectional, "0001234567890", 0),
            (DataBarKind::Omnidirectional, "2001234567890", 0),
            (DataBarKind::Omnidirectional, "9999999999999", 0),
            (DataBarKind::Omnidirectional, "0000000000000", 0),
            (DataBarKind::Truncated, "0412345678901", 0),
            (DataBarKind::Stacked, "0412345678901", 0),
            (DataBarKind::StackedOmnidirectional, "0412345678901", 0),
            (DataBarKind::StackedOmnidirectional, "9876543210123", 0),
            (DataBarKind::Limited, "1501234567890", 0),
            (DataBarKind::Limited, "0001234567890", 0),
            (DataBarKind::Limited, "1999999999999", 0),
            (
                DataBarKind::Expanded,
                "(01)98898765432106(3202)012345(15)991231",
                0,
            ),
            (DataBarKind::Expanded, "(01)90012345678908(3103)001750", 0),
            (DataBarKind::Expanded, "(01)90012345678908(3203)012345", 0),
            (DataBarKind::Expanded, "(01)90012345678908(3922)795", 0),
            (DataBarKind::Expanded, "(01)90012345678908(3932)0401234", 0),
            (
                DataBarKind::Expanded,
                "(01)90012345678908(3102)123456(11)250101",
                0,
            ),
            (DataBarKind::Expanded, "(01)00012345678905(10)ABC123", 0),
            (DataBarKind::Expanded, "(01)00012345678905(21)abc%def", 0),
            (DataBarKind::Expanded, "(10)12A", 0),
            (DataBarKind::Expanded, "(01)12345678901231", 0),
            (DataBarKind::Expanded, "(21)abcdefgh?!(30)1234567", 0),
            (
                DataBarKind::Expanded,
                "(01)98898765432106(10)AB-12(21)XYZ",
                0,
            ),
            (
                DataBarKind::Expanded,
                "(01)98898765432106(10)ABCDEF123456(21)12345",
                2,
            ),
            (
                DataBarKind::Expanded,
                "(01)98898765432106(10)ABCDEF123456(21)12345",
                4,
            ),
            (
                DataBarKind::Expanded,
                "(01)98898765432106(10)ABCDEF123456(21)12345",
                6,
            ),
            (
                DataBarKind::Expanded,
                "(01)98898765432106(10)ABCDEF123456(21)12345",
                8,
            ),
            (
                DataBarKind::Expanded,
                "(01)98898765432106(10)ABCDEF123456(21)12345",
                10,
            ),
            (
                DataBarKind::Expanded,
                "(01)98898765432106(3202)012345(15)991231",
                4,
            ),
            (
                DataBarKind::Expanded,
                "(01)90614141999999(10)1234222222222221",
                4,
            ),
            (
                DataBarKind::Expanded,
                "(01)90614141999999(10)1234222222222221",
                6,
            ),
            (
                DataBarKind::Expanded,
                "(01)90614141999999(10)1234222222222221",
                8,
            ),
            (
                DataBarKind::Expanded,
                "(01)90614141999999(10)1234222222222221(21)ABCDEFGH",
                2,
            ),
            (
                DataBarKind::Expanded,
                "(01)90614141999999(10)1234222222222221(21)ABCDEFGH",
                4,
            ),
            (
                DataBarKind::Expanded,
                "(01)90614141999999(10)1234222222222221(21)ABCDEFGH",
                6,
            ),
            (
                DataBarKind::Expanded,
                "(01)90614141999999(10)1234222222222221(21)ABCDEFGH",
                8,
            ),
            (
                DataBarKind::Expanded,
                "(01)90614141999999(10)1234222222222221(21)ABCDEFGH",
                12,
            ),
            (
                DataBarKind::Expanded,
                "(01)90614141999999(10)1234222222222221(21)ABCDEFGH",
                14,
            ),
            (
                DataBarKind::Expanded,
                "(01)90614141999999(10)1234222222222221(21)ABCDEFGH",
                18,
            ),
            (
                DataBarKind::Expanded,
                "(01)90614141999999(10)12342(21)ABCDEFGH",
                6,
            ),
            (
                DataBarKind::Expanded,
                "(01)90614141999999(10)12342(21)ABCDEFG",
                6,
            ),
            (
                DataBarKind::Expanded,
                "(01)90614141999999(10)12342(21)ABCDEF",
                6,
            ),
            (
                DataBarKind::Expanded,
                "(01)90614141999999(10)12342(21)ABCDEF",
                10,
            ),
            (
                DataBarKind::Expanded,
                "(01)90614141999999(10)12342(21)ABCDEFGHI",
                4,
            ),
            (
                DataBarKind::Expanded,
                "(01)90614141999999(10)12342(21)ABCDEFGHIJ",
                4,
            ),
            (
                DataBarKind::Expanded,
                "(01)90614141999999(10)12342(21)ABCDEFGHIJK",
                4,
            ),
            (
                DataBarKind::Expanded,
                "(01)90614141999999(10)12342(21)ABCDEFGHIJKL",
                4,
            ),
            (DataBarKind::Expanded, "0190614141999999102020\x1d21XY", 0),
            (DataBarKind::Expanded, "(01)90614141999996(3102)000101", 0),
            (DataBarKind::Expanded, "(01)90012345678908(3109)123456", 0),
            (
                DataBarKind::Expanded,
                "(01)90012345678908(3205)123456(17)251300",
                0,
            ),
            (
                DataBarKind::Expanded,
                "(01)90012345678908(3205)123456(17)251231",
                0,
            ),
            (
                DataBarKind::Expanded,
                "(01)90012345678908(3205)123456(13)251231",
                6,
            ),
            (DataBarKind::Expanded, "(10)12A", 2),
            (
                DataBarKind::Expanded,
                "(90)ABCDEFGHIJKLMNOPQRSTUVWXYZ01ab",
                6,
            ),
            (DataBarKind::Expanded, "(01)00012345678905(10)ABC123", 2),
        ];
        let mut out = String::new();
        for (kind, data, segs) in cases {
            let s = encode(kind, data, segs).unwrap();
            writeln!(
                out,
                "# {kind:?}|{data}|{segs}|{}",
                s.hri.unwrap_or_default()
            )
            .unwrap();
            for r in &s.symbol.rows {
                let h = match r.height {
                    RowHeight::Bar { weight } => weight,
                    RowHeight::Modules(m) => m,
                };
                writeln!(out, "{h} {}", row_str(&r.modules)).unwrap();
            }
        }
        std::fs::write(path, out).unwrap();
    }
}
