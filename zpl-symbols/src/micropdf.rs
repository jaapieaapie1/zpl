//! MicroPDF417 (ISO/IEC 24728) and TLC39 (TCIF, `^BT`).
//!
//! MicroPDF417 reuses the PDF417 codeword patterns, high-level encoding and GF(929)
//! Reed-Solomon arithmetic from [`crate::matrix`], but replaces the start/stop patterns and
//! row indicators with 10-module row address patterns (RAPs) and comes in 34 fixed sizes.
//! Tables after ISO/IEC 24728:2006 Tables 1, 2 and 10-12 (as tabulated in zint's
//! `pdf417_tabs.h`, BSD-3-Clause).

use crate::matrix::{CODEWORDS, SUB_ALPHA, is_pdf_text, pdf_bytes, pdf_numeric, pdf_text};
use crate::{RowHeight, Stacked, StackedRow, SymbolError};

/// Data columns per variant (ISO/IEC 24728 Table 1 order).
const COLS: [u8; 34] = [
    1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4,
];
/// Rows per variant.
const ROWS: [u8; 34] = [
    11, 14, 17, 20, 24, 28, 8, 11, 14, 17, 20, 23, 26, 6, 8, 10, 12, 15, 20, 26, 32, 38, 44, 4, 6,
    8, 10, 12, 15, 20, 26, 32, 38, 44,
];
/// Error-correction codewords per variant.
const ECC: [u8; 34] = [
    7, 7, 7, 8, 8, 8, 8, 9, 9, 10, 11, 13, 15, 12, 14, 16, 18, 21, 26, 32, 38, 44, 50, 8, 12, 14,
    16, 18, 21, 26, 32, 38, 44, 50,
];
/// Starting left / centre / right RAP numbers (1-based) and start cluster / 3 per variant.
const RAP: [[u8; 34]; 4] = [
    [
        1, 8, 36, 19, 9, 25, 1, 1, 8, 36, 19, 9, 27, 1, 7, 15, 25, 37, 1, 1, 21, 15, 1, 47, 1, 7,
        15, 25, 37, 1, 1, 21, 15, 1,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 7, 15, 25, 37, 17, 9, 29, 31, 25, 19, 1, 7, 15,
        25, 37, 17, 9, 29, 31, 25,
    ],
    [
        9, 8, 36, 19, 17, 33, 1, 9, 8, 36, 19, 17, 35, 1, 7, 15, 25, 37, 33, 17, 37, 47, 49, 43, 1,
        7, 15, 25, 37, 33, 17, 37, 47, 49,
    ],
    [
        0, 1, 2, 0, 2, 0, 0, 0, 1, 2, 0, 2, 2, 0, 0, 2, 0, 0, 0, 0, 2, 2, 0, 1, 0, 0, 2, 0, 0, 0,
        0, 2, 2, 0,
    ],
];

/// Left and right row address patterns (10 modules, ISO/IEC 15438 / 24728 Table 2).
pub(crate) const RAP_SIDE: [u16; 52] = [
    0x322, 0x3A2, 0x3B2, 0x332, 0x372, 0x37A, 0x33A, 0x3BA, 0x39A, 0x3DA, 0x3CA, 0x38A, 0x30A,
    0x31A, 0x312, 0x392, 0x3D2, 0x3D6, 0x3D4, 0x394, 0x3B4, 0x3A4, 0x3A6, 0x3AE, 0x3AC, 0x3A8,
    0x328, 0x32C, 0x32E, 0x326, 0x336, 0x3B6, 0x396, 0x316, 0x314, 0x334, 0x374, 0x364, 0x366,
    0x36E, 0x36C, 0x368, 0x348, 0x358, 0x35C, 0x35E, 0x34E, 0x34C, 0x344, 0x346, 0x342, 0x362,
];
/// Centre row address patterns (10 modules).
pub(crate) const RAP_CENTRE: [u16; 52] = [
    0x2CE, 0x24E, 0x26E, 0x22E, 0x226, 0x236, 0x216, 0x212, 0x21A, 0x23A, 0x232, 0x222, 0x262,
    0x272, 0x27A, 0x2FA, 0x2F2, 0x2F6, 0x276, 0x274, 0x264, 0x266, 0x246, 0x242, 0x2C2, 0x2E2,
    0x2E6, 0x2E4, 0x2EC, 0x26C, 0x22C, 0x228, 0x268, 0x2E8, 0x2C8, 0x2CC, 0x2C4, 0x2C6, 0x286,
    0x28E, 0x28C, 0x29C, 0x298, 0x2B8, 0x2B0, 0x290, 0x2D0, 0x250, 0x258, 0x25C, 0x2DC, 0x2DE,
];

/// Maximum number of data codewords (largest symbol, 4 x 44 with 50 ECC).
const MAX_DATA_CWS: usize = 126;

/// Variant index (ISO table order) for a `^BF` mode. ZPL lists the 4-column 4-row symbol last.
fn variant_for_mode(mode: u8) -> usize {
    match mode {
        0..=22 => mode as usize,
        23..=32 => mode as usize + 1,
        33 => 23,
        _ => 0,
    }
}

pub(crate) fn capacity(v: usize) -> usize {
    COLS[v] as usize * ROWS[v] as usize - ECC[v] as usize
}

/// Append the low `len` bits of `bits` (most significant first).
pub(crate) fn push_bits(row: &mut Vec<bool>, bits: u32, len: u32) {
    row.extend((0..len).rev().map(|b| bits >> b & 1 != 0));
}

/// The 17-module pattern of codeword `value` in `cluster` (0, 1, 2 = clusters 0, 3, 6).
pub(crate) fn codeword_bits(cluster: usize, value: u16) -> u32 {
    0x1_0000 | u32::from(CODEWORDS[cluster][value as usize])
}

/// Generator polynomial coefficients for `k` check codewords (low degree first, leading 1
/// omitted), cached per `k` (1-64).
fn rs_generator(k: usize) -> &'static [u32] {
    static CACHE: [std::sync::OnceLock<Vec<u32>>; 64] = [const { std::sync::OnceLock::new() }; 64];
    CACHE[k - 1].get_or_init(|| {
        let mut poly: Vec<u32> = vec![1];
        let mut a = 1u32;
        for _ in 0..k {
            a = a * 3 % 929;
            let mut next = vec![0u32; poly.len() + 1];
            for (j, &p) in poly.iter().enumerate() {
                next[j + 1] = (next[j + 1] + p) % 929;
                next[j] = (next[j] + 929 - p * a % 929) % 929;
            }
            poly = next;
        }
        poly.truncate(k);
        poly
    })
}

/// Reed-Solomon check codewords over GF(929) with generator `prod(x - 3^i), i = 1..=k`
/// (ISO/IEC 15438 / 24728 / 24723; identical to the PDF417 levels for k = 2^(s+1)).
pub(crate) fn rs_ecc(data: &[u16], k: usize) -> Vec<u16> {
    let coef = rs_generator(k);
    let mut e = vec![0u32; k];
    for &d in data {
        let t1 = (u32::from(d) + e[k - 1]) % 929;
        for j in (1..k).rev() {
            e[j] = (e[j - 1] + 929 - t1 * coef[j] % 929) % 929;
        }
        e[0] = (929 - t1 * coef[0] % 929) % 929;
    }
    e.iter()
        .rev()
        .map(|&v| if v == 0 { 0 } else { (929 - v) as u16 })
        .collect()
}

/// Row layout parameters shared by MicroPDF417 and the CC-A / CC-B composite components.
pub(crate) struct RapLayout {
    pub cols: usize,
    pub rows: usize,
    /// 1-based starting RAP numbers and start cluster / 3.
    pub left: u8,
    pub centre: u8,
    pub right: u8,
    pub cluster: u8,
    /// CC-A with 3 columns has no left RAP (ISO/IEC 24723 Figure 6).
    pub no_left: bool,
}

/// Lay out `cws` (data + ECC, `cols * rows` of them) into module rows.
pub(crate) fn rap_rows(cws: &[u16], l: &RapLayout) -> Vec<Vec<bool>> {
    let (mut left, mut centre, mut right) = (
        usize::from(l.left) - 1,
        usize::from(l.centre.max(1)) - 1,
        usize::from(l.right) - 1,
    );
    let mut cluster = usize::from(l.cluster);
    let width = 11 + 17 * l.cols + 10 * usize::from(l.cols >= 3) + 10 * usize::from(!l.no_left);
    let mut out = Vec::with_capacity(l.rows);
    for r in 0..l.rows {
        let mut row = Vec::with_capacity(width);
        let cw = |row: &mut Vec<bool>, c: usize| {
            push_bits(row, codeword_bits(cluster, cws[r * l.cols + c]), 17);
        };
        if !l.no_left {
            push_bits(&mut row, u32::from(RAP_SIDE[left]), 10);
        }
        match l.cols {
            1 => cw(&mut row, 0),
            2 => (0..2).for_each(|c| cw(&mut row, c)),
            3 => {
                cw(&mut row, 0);
                push_bits(&mut row, u32::from(RAP_CENTRE[centre]), 10);
                cw(&mut row, 1);
                cw(&mut row, 2);
            }
            _ => {
                cw(&mut row, 0);
                cw(&mut row, 1);
                push_bits(&mut row, u32::from(RAP_CENTRE[centre]), 10);
                cw(&mut row, 2);
                cw(&mut row, 3);
            }
        }
        push_bits(&mut row, u32::from(RAP_SIDE[right]), 10);
        row.push(true); // 1-module stop bar
        out.push(row);
        left = (left + 1) % 52;
        centre = (centre + 1) % 52;
        right = (right + 1) % 52;
        cluster = (cluster + 1) % 3;
    }
    out
}

/// Encode a full MicroPDF417 codeword stream (data codewords, already padded) for variant
/// `v` (0-33, ISO table order): append ECC and lay out the rows.
pub(crate) fn variant_rows(mut cws: Vec<u16>, v: usize) -> Vec<Vec<bool>> {
    let ecc = rs_ecc(&cws, ECC[v] as usize);
    cws.extend(ecc);
    rap_rows(
        &cws,
        &RapLayout {
            cols: COLS[v] as usize,
            rows: ROWS[v] as usize,
            left: RAP[0][v],
            centre: RAP[1][v],
            right: RAP[2][v],
            cluster: RAP[3][v],
            no_left: false,
        },
    )
}

/// Smallest variant with `cols` columns holding `n` data codewords.
pub(crate) fn smallest_variant(cols: u8, n: usize) -> Option<usize> {
    (0..34)
        .filter(|&v| COLS[v] == cols && capacity(v) >= n)
        .min_by_key(|&v| ROWS[v])
}

/// How a run of the input is compacted.
#[derive(Clone, Copy, PartialEq)]
enum Compaction {
    Text,
    Numeric,
    Byte,
}

/// A maximal run of digits, other text-compactable characters, or other bytes.
#[derive(Clone, Copy)]
struct Run {
    start: usize,
    end: usize,
    /// Options in preference order of the default choice.
    options: &'static [Compaction],
}

fn runs(data: &[u8]) -> Vec<Run> {
    let class = |c: u8| {
        if c.is_ascii_digit() {
            0
        } else if is_pdf_text(c) {
            1
        } else {
            2
        }
    };
    let mut spans: Vec<(usize, usize, u8)> = Vec::new();
    let mut i = 0;
    while i < data.len() {
        let k = class(data[i]);
        let start = i;
        while i < data.len() && class(data[i]) == k {
            i += 1;
        }
        spans.push((start, i, k));
    }
    // Only alternatives that can pay off are tried: Numeric Compaction for 5+ digits (fewer
    // never save a codeword), Byte Compaction for short runs next to non-text bytes.
    let near_binary =
        |j: usize| (j > 0 && spans[j - 1].2 == 2) || spans.get(j + 1).is_some_and(|s| s.2 == 2);
    spans
        .iter()
        .enumerate()
        .map(|(j, &(start, end, k))| {
            use Compaction::*;
            let n = end - start;
            let nb = n < 8 && near_binary(j);
            let options: &'static [Compaction] = match k {
                0 if n >= 13 => &[Numeric, Text],
                0 if n >= 5 && nb => &[Text, Numeric, Byte],
                0 if n >= 5 => &[Text, Numeric],
                0 | 1 if nb => &[Text, Byte],
                0 | 1 => &[Text],
                _ => &[Byte],
            };
            Run {
                start,
                end,
                options,
            }
        })
        .collect()
}

/// Encode `data` with one compaction choice per run. MicroPDF417 starts in Byte Compaction
/// (ISO/IEC 24728 5.4.3), so every block gets an explicit latch.
fn encode_runs(data: &[u8], runs: &[Run], choice: &[Compaction], out: &mut Vec<u16>) {
    let mut mode = Compaction::Byte;
    let mut first = true;
    let mut sub = SUB_ALPHA;
    let mut i = 0;
    while i < runs.len() {
        // Merge consecutive runs with the same compaction into one block.
        let c = choice[i];
        let mut j = i + 1;
        while j < runs.len() && choice[j] == c {
            j += 1;
        }
        let block = &data[runs[i].start..runs[j - 1].end];
        match c {
            Compaction::Text => {
                if mode != Compaction::Text || first {
                    out.push(900);
                    sub = SUB_ALPHA;
                }
                sub = pdf_text(block, sub, out);
                mode = Compaction::Text;
            }
            Compaction::Numeric => {
                out.push(902);
                pdf_numeric(block, out);
                mode = Compaction::Numeric;
            }
            Compaction::Byte => {
                if block.len() == 1 && mode == Compaction::Text {
                    out.push(913);
                    out.push(u16::from(block[0]));
                } else {
                    pdf_bytes(block, out);
                    mode = Compaction::Byte;
                }
            }
        }
        first = false;
        i = j;
    }
}

/// MicroPDF417 data codewords for `data` (no padding): the PDF417 text / byte / numeric
/// compactions, with the compaction of each character run chosen greedily (left to right) to
/// minimise the total codeword count, so short digit strings use Numeric Compaction when
/// that is shorter (as the ZPL capacity table assumes, e.g. 8 digits in mode 0).
pub(crate) fn micro_codewords(data: &[u8]) -> Vec<u16> {
    let runs = runs(data);
    let mut choice: Vec<Compaction> = runs.iter().map(|r| r.options[0]).collect();
    let mut best = Vec::new();
    encode_runs(data, &runs, &choice, &mut best);
    let mut trial = Vec::with_capacity(best.len() + 8);
    for i in 0..runs.len() {
        let mut keep = choice[i];
        for &alt in &runs[i].options[1..] {
            choice[i] = alt;
            trial.clear();
            encode_runs(data, &runs, &choice, &mut trial);
            if trial.len() < best.len() {
                std::mem::swap(&mut best, &mut trial);
                keep = alt;
            }
        }
        choice[i] = keep;
    }
    best
}

fn to_stacked(rows: Vec<Vec<bool>>) -> Stacked {
    Stacked {
        width: rows.first().map_or(0, |r| r.len() as u32),
        rows: rows
            .into_iter()
            .map(|modules| StackedRow {
                modules,
                height: RowHeight::Bar { weight: 1 },
            })
            .collect(),
    }
}

/// `^BF` MicroPDF417.
///
/// `mode`: ^BF m (0-33) selecting columns / rows / error correction per the ZPL mode table
/// (out-of-range values are treated as 0). If the data does not fit the selected mode the
/// smallest variant with the same column count that fits is used, else the smallest variant
/// with more columns. Rows use `RowHeight::Bar { weight: 1 }` (the renderer applies the ^BF
/// row height `h`). No quiet zone is included.
pub fn micro_pdf417(data: &[u8], mode: u8) -> Result<Stacked, SymbolError> {
    let mut cws = micro_codewords(data);
    if cws.len() > MAX_DATA_CWS {
        return Err(SymbolError::InvalidLength(data.len()));
    }
    let mut v = variant_for_mode(mode);
    if capacity(v) < cws.len() {
        v = (COLS[v]..=4)
            .find_map(|c| smallest_variant(c, cws.len()))
            .ok_or(SymbolError::InvalidLength(data.len()))?;
    }
    cws.resize(capacity(v), 900);
    Ok(to_stacked(variant_rows(cws, v)))
}

/// A TLC39 symbol: Code 39 linear part (the 6-digit ECI number) and the MicroPDF417 part
/// (serial number and additional data). `micro` is empty (no rows) when the field data has
/// no 2D part.
#[derive(Debug, Clone, PartialEq)]
pub struct Tlc39 {
    pub code39: crate::linear::LinearSymbol,
    pub micro: Stacked,
}

/// `^BT` TLC39. `data` is `ECI[,serial[,additional...]]`: the first six characters must be
/// digits (the ECI number, encoded in Code 39 without check digit). If the seventh character
/// is a comma, everything after it (serial number and the comma separated additional fields,
/// max 150 bytes) is encoded in a 4-column MicroPDF417 of the smallest fitting size;
/// otherwise only the Code 39 part is produced.
pub fn tlc39(data: &str) -> Result<Tlc39, SymbolError> {
    let bytes = data.as_bytes();
    if bytes.len() < 6 {
        return Err(SymbolError::InvalidLength(bytes.len()));
    }
    if let Some(&c) = bytes[..6].iter().find(|c| !c.is_ascii_digit()) {
        return Err(SymbolError::InvalidCharacter(c as char));
    }
    let eci = &data[..6];
    let code39 =
        crate::linear::encode(crate::linear::Symbology::Code39 { check_digit: false }, eci)
            .map_err(|e| match e {
                crate::linear::EncodeError::InvalidCharacter(c) => SymbolError::InvalidCharacter(c),
                crate::linear::EncodeError::InvalidLength(n) => SymbolError::InvalidLength(n),
                crate::linear::EncodeError::Empty => SymbolError::InvalidLength(0),
            })?;
    let micro = match bytes.get(6) {
        Some(b',') => {
            let rest = &bytes[7..];
            if rest.len() > 150 {
                return Err(SymbolError::InvalidLength(rest.len()));
            }
            let mut cws = micro_codewords(rest);
            let v = smallest_variant(4, cws.len()).ok_or(SymbolError::InvalidLength(rest.len()))?;
            cws.resize(capacity(v), 900);
            to_stacked(variant_rows(cws, v))
        }
        _ => Stacked {
            width: 0,
            rows: Vec::new(),
        },
    };
    Ok(Tlc39 { code39, micro })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rows_str(s: &Stacked) -> Vec<String> {
        s.rows
            .iter()
            .map(|r| {
                r.modules
                    .iter()
                    .map(|&b| if b { '1' } else { '0' })
                    .collect()
            })
            .collect()
    }

    #[test]
    fn rs_matches_iso_coefficients() {
        // ISO/IEC 24728 / 24723 Annex F generator coefficients.
        assert_eq!(rs_generator(4), [522, 568, 723, 809]);
        assert_eq!(rs_generator(7), [76, 925, 537, 597, 784, 691, 437]);
        assert_eq!(rs_generator(50)[..4], [923, 797, 576, 875]);
        assert_eq!(rs_generator(50)[48..], [718, 435]);
    }

    #[test]
    fn mode_table_matches_zpl() {
        // (mode, columns, rows, ecc %) from the ^BF mode table.
        let table: [(u8, u8, u8, u32); 34] = [
            (0, 1, 11, 64),
            (1, 1, 14, 50),
            (2, 1, 17, 41),
            (3, 1, 20, 40),
            (4, 1, 24, 33),
            (5, 1, 28, 29),
            (6, 2, 8, 50),
            (7, 2, 11, 41),
            (8, 2, 14, 32),
            (9, 2, 17, 29),
            (10, 2, 20, 28),
            (11, 2, 23, 28),
            (12, 2, 26, 29),
            (13, 3, 6, 67),
            (14, 3, 8, 58),
            (15, 3, 10, 53),
            (16, 3, 12, 50),
            (17, 3, 15, 47),
            (18, 3, 20, 43),
            (19, 3, 26, 41),
            (20, 3, 32, 40),
            (21, 3, 38, 39),
            (22, 3, 44, 38),
            (23, 4, 6, 50),
            (24, 4, 8, 44),
            (25, 4, 10, 40),
            (26, 4, 12, 38),
            (27, 4, 15, 35),
            (28, 4, 20, 33),
            (29, 4, 26, 31),
            (30, 4, 32, 30),
            (31, 4, 38, 29),
            (32, 4, 44, 28),
            (33, 4, 4, 50),
        ];
        for (m, c, r, pct) in table {
            let v = variant_for_mode(m);
            assert_eq!((COLS[v], ROWS[v]), (c, r), "mode {m}");
            let total = u32::from(c) * u32::from(r);
            let got = (u32::from(ECC[v]) * 100 + total / 2) / total;
            assert!(got.abs_diff(pct) <= 1, "mode {m}: {got}% vs {pct}%");
            let s = micro_pdf417(b"1", m).unwrap();
            assert_eq!(s.rows.len(), r as usize);
            assert_eq!(s.width, [0, 38, 55, 82, 99][c as usize]);
        }
    }

    #[test]
    fn capacity_matches_zpl_table() {
        // Max digits / alpha characters from the ^BF table fit their mode exactly.
        let digits = "1".repeat(366);
        let alpha = "A".repeat(250);
        assert_eq!(micro_pdf417(digits.as_bytes(), 32).unwrap().rows.len(), 44);
        assert_eq!(micro_pdf417(alpha.as_bytes(), 32).unwrap().rows.len(), 44);
        assert!(micro_pdf417("1".repeat(367).as_bytes(), 32).is_err());
        assert!(micro_pdf417("A".repeat(251).as_bytes(), 32).is_err());
        assert_eq!(micro_pdf417(b"12345678", 0).unwrap().rows.len(), 11);
        // 9 digits do not fit mode 0: grows to the next 1-column size.
        assert_eq!(micro_pdf417(b"123456789", 0).unwrap().rows.len(), 14);
    }

    // Expected rows generated by zint 2.15 (via zxing-cpp's writer) for the same variant;
    // all 34 variants were checked bit-identical against zint during development.
    #[test]
    fn zint_vectors() {
        let s = micro_pdf417(b"ZPL MICRO", 13).unwrap();
        assert_eq!(
            rows_str(&s),
            [
                "1100100010100001100011001001011001110111011110011010001100010101100000011001000101",
                "1110100010110010001110100001001001110111101001000000101111011000001101011101000101",
                "1110110010100000111010011001001101110111000100111110101011111000111011011101100101",
                "1100110010100001011101110001000101110101100011100001101110011001011000011001100101",
                "1101110010100101111101100001000100110111010111101110001110101100010000011011100101",
                "1101111010110011100010111001000110110100101000111100001001110011011100011011110101",
            ]
        );
        let s = micro_pdf417(b"1234567", 0).unwrap();
        assert_eq!(
            rows_str(&s),
            [
                "11001000101001111011110100011100110101",
                "11101000101111010110110000011110110101",
                "11101100101110001111010111011110010101",
                "11001100101110111100110010011100010101",
                "11011100101010111111000111011000010101",
                "11011110101000011110010100011000110101",
                "11001110101110011101000001011000100101",
                "11101110101110000010011010011100100101",
                "11100110101101111100010100011110100101",
                "11110110101000101000100000011110101101",
                "11110010101011001111100001011110101001",
            ]
        );
        // 4 columns x 4 rows (ZPL mode 33).
        let s = micro_pdf417(b"x", 33).unwrap();
        assert_eq!(
            rows_str(&s)[0],
            "110100111010111111010110000100001101111101001001110110101111110101100001011111101011000011010010001"
        );
        assert_eq!(s.rows.len(), 4);
    }

    #[test]
    fn tlc39_parts() {
        let t = tlc39("123456,ABCD12345678901234,ABCDEFGHIJKLMNOPQRSTUVWXYZ").unwrap();
        assert_eq!(t.micro.width, 99);
        assert!(!t.micro.rows.is_empty());
        let only = tlc39("1234567").unwrap();
        assert!(only.micro.rows.is_empty());
        assert_eq!(only.code39.text[0].text, "*123456*");
        assert!(tlc39("12A456,X").is_err());
    }
}
