//! Stacked symbologies: Codablock A/E/F (ZPL `^BB`) and Code 49 (ZPL `^B4`).
//!
//! Both encoders return a [`Stacked`] symbol in module units without quiet zones. Data rows
//! have `RowHeight::Bar { weight: 1 }`; the 1-module separator bars between rows and the
//! top/bottom binding bars are rows of their own with `RowHeight::Modules(1)`.
//!
//! Renderer notes:
//! - `^BB` `h` is the height of each individual row in dots (default 8); when it is absent
//!   `^BY` `h` is the total height shared by all `Bar` rows.
//! - `^B4` `h` is a multiplier: each `Bar` row is `h * module width` dots high (default:
//!   `^BY` height shared by the rows). The interpretation line (`f` = `A`/`B`) prints the text
//!   returned by [`code49`].
//!
//! Codablock F follows AIM ISS-X-24 using the row/column optimiser of zint (BSD-3-Clause,
//! <https://github.com/zint/zint>, `backend/codablock.c`), so symbols are identical to zint's
//! for the same row/column choice. Code 49 follows ANSI/AIM BC6-2000 (USS-49) as implemented by
//! zint `backend/code49.c`; the symbol character tables come from that standard.

use crate::{RowHeight, Stacked, StackedRow, SymbolError};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CodablockMode {
    /// Code 39 based, 1-22 rows.
    A,
    /// Codablock F with an automatic leading FNC1.
    E,
    /// Code 128 based (AIM ISS-X-24), 2-44 rows.
    F,
}

// =============================================================================================
// Shared helpers
// =============================================================================================

/// Push `width` modules of colour `dark`.
fn push_run(out: &mut Vec<bool>, dark: bool, width: u8) {
    out.extend(std::iter::repeat_n(dark, width as usize));
}

/// Build the final symbol: top bind, data rows separated by separator rows, bottom bind.
/// `sep` is the dark span `[from, to)` of the separators between rows.
fn assemble(data_rows: Vec<Vec<bool>>, width: usize, sep: (usize, usize)) -> Stacked {
    let full = StackedRow {
        modules: vec![true; width],
        height: RowHeight::Modules(1),
    };
    let separator = StackedRow {
        modules: (0..width).map(|x| x >= sep.0 && x < sep.1).collect(),
        height: RowHeight::Modules(1),
    };
    let mut rows = vec![full.clone()];
    let n = data_rows.len();
    for (i, mut modules) in data_rows.into_iter().enumerate() {
        modules.resize(width, false);
        rows.push(StackedRow {
            modules,
            height: RowHeight::Bar { weight: 1 },
        });
        if i + 1 < n {
            rows.push(separator.clone());
        }
    }
    rows.push(full);
    Stacked {
        width: width as u32,
        rows,
    }
}

// =============================================================================================
// Codablock (^BB)
// =============================================================================================

/// Encode a Codablock symbol (ZPL `^BBo,h,s,c,r,m`).
///
/// - `columns` = `^BB c`, data characters per row (0 = automatic). Codablock F/E clamp it to
///   4..=62 (the AIM minimum; ZPL documents 2..=62), Codablock A to 1..=61.
/// - `rows` = `^BB r` (0 = automatic). F/E: up to 44 rows; A: up to 22 rows.
/// - `security` = `^BB s`: only honoured for Codablock A (F and E always carry their K1/K2
///   check characters, as documented for ZPL). Single-row symbols never carry symbol checks.
///
/// Like ZPL: with neither `columns` nor `rows`, or with `rows == 1`, or with `columns > 62` and
/// no `rows`, a single row is produced (a plain Code 128 symbol for F/E, Code 39 for A).
///
/// Field data: ASCII. For F/E, U+0080..U+0083 are FNC1..FNC4 and U+0084 is NUL (the ZPL `^FH`
/// escapes `_80`..`_84`); FNC4 applies to the following ASCII character (extended ASCII).
pub fn codablock(
    data: &str,
    mode: CodablockMode,
    columns: u32,
    rows: u32,
    security: bool,
) -> Result<Stacked, SymbolError> {
    if data.is_empty() {
        return Err(SymbolError::InvalidLength(0));
    }
    let single = rows == 1 || (rows == 0 && (columns == 0 || columns > 62));
    match mode {
        CodablockMode::A => codablock_a(data, columns, rows, security, single),
        CodablockMode::E | CodablockMode::F => {
            let (mut cf, kmsg) = cbf_input(data)?;
            if mode == CodablockMode::E {
                // Automatic leading FNC1 (not counted in K1/K2).
                cf.insert(0, A_FNC1);
            }
            if single {
                let values = code128_values(&cf);
                let width = values.len() * 11 + 13;
                return Ok(assemble(vec![c128_modules(&values)], width, (0, width)));
            }
            let rows = rows.min(44) as i32;
            let cols = if columns == 0 {
                0
            } else {
                columns.clamp(4, 62) as i32
            };
            let cws = cbf_codewords(&cf, &kmsg, rows, cols)?;
            let width = cws[0].len() * 11 + 2;
            let rows = cws.iter().map(|r| c128_modules(r)).collect();
            Ok(assemble(rows, width, (11, width - 13)))
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Codablock F
// ---------------------------------------------------------------------------------------------

// Internal markers above the 0..=127 data bytes.
const A_FNC1: u16 = 128;
const A_FNC2: u16 = 129;
const A_FNC3: u16 = 130;
const A_FNC4: u16 = 131;
const A_CODEA: u16 = 132;
const A_CODEB: u16 = 133;
const A_CODEC: u16 = 134;
const A_SHIFT: u16 = 135;

// Character set flags.
const CODE_A: u32 = 1;
const CODE_B: u32 = 2;
const CODE_C: u32 = 4;
const C_END: u32 = 8;
const C_SHIFT: u32 = 16;
const C_FILL: u32 = 32;
const CODE_FNC1: u32 = 64;
const CODE_FNC4: u32 = 128;
const ZT_NUM: u32 = CODE_A | CODE_B | CODE_C;
const ZT_FNC1: u32 = CODE_A | CODE_B | CODE_C | CODE_FNC1;

/// Convert ZPL field data to Codablock F data (bytes + markers) and the K1/K2 message.
/// K1/K2 follow AIM ISS-X-24 Annex F / BWIPP: function characters do not count, except a
/// non-leading FNC1 which counts as GS (29); FNC4+c counts as c+128.
fn cbf_input(data: &str) -> Result<(Vec<u16>, Vec<u32>), SymbolError> {
    let mut out = Vec::with_capacity(data.len());
    let mut kmsg = Vec::with_capacity(data.len());
    let mut chars = data.chars().peekable();
    let mut first = true;
    while let Some(ch) = chars.next() {
        match ch as u32 {
            c @ 0..=0x7f => {
                out.push(c as u16);
                kmsg.push(c);
            }
            0x80 => {
                out.push(A_FNC1);
                if !first {
                    kmsg.push(29);
                }
            }
            0x81 => out.push(A_FNC2),
            0x82 => out.push(A_FNC3),
            0x83 => match chars.next() {
                Some(n) if (n as u32) < 0x80 => {
                    out.push(A_FNC4);
                    out.push(n as u16);
                    kmsg.push(n as u32 + 128);
                }
                _ => {
                    return Err(SymbolError::InvalidData(
                        "FNC4 must be followed by an ASCII character".into(),
                    ));
                }
            },
            0x84 => {
                out.push(0);
                kmsg.push(0);
            }
            _ => return Err(SymbolError::InvalidCharacter(ch)),
        }
        first = false;
    }
    Ok((out, kmsg))
}

#[derive(Clone, Copy, Default)]
struct CharSet {
    set: u32,
    a_following: i32,
    b_following: i32,
    c_following: i32,
}

fn possible_set(c: u16, prev_not_fnc4: bool) -> u32 {
    if c <= 0x1f {
        CODE_A
    } else if (u16::from(b'0')..=u16::from(b'9')).contains(&c) && prev_not_fnc4 {
        ZT_NUM
    } else if c == A_FNC1 {
        ZT_FNC1
    } else if c == A_FNC4 {
        CODE_A | CODE_B | CODE_FNC4
    } else if (0x60..=0x7f).contains(&c) {
        CODE_B
    } else {
        CODE_A | CODE_B
    }
}

fn char_set_table(data: &[u16]) -> Vec<CharSet> {
    let n = data.len();
    let mut t = vec![CharSet::default(); n];
    for i in (0..n).rev() {
        let set = possible_set(data[i], i == 0 || data[i - 1] != A_FNC4);
        let next = t.get(i + 1).copied().unwrap_or_default();
        t[i].set = set;
        t[i].a_following = if set & CODE_A != 0 {
            next.a_following + 1
        } else {
            0
        };
        t[i].b_following = if set & CODE_B != 0 {
            next.b_following + 1
        } else {
            0
        };
    }
    // Code C chains (counted in source characters). Mirrors zint, including that only every
    // second position of a chain is checked for being a digit; this only affects decisions,
    // actual Code C pairs are re-checked per position.
    for i in 0..n {
        if t[i].set & CODE_C == 0 {
            continue;
        }
        let mut run = i;
        let mut cf = 0;
        loop {
            if t[run].set == ZT_FNC1 {
                cf += 1;
            } else {
                run += 1;
                if run >= n || t[run].set != ZT_NUM {
                    break;
                }
                cf += 2;
            }
            run += 1;
            if run >= n {
                break;
            }
        }
        t[i].c_following = cf;
    }
    t
}

/// Number of digits that fit as Code C pairs in `empty` columns starting at `cur`.
fn remaining_digits(t: &[CharSet], cur: usize, mut empty: i32) -> i32 {
    let Some(start) = t.get(cur) else { return 0 };
    let end = cur + start.c_following as usize;
    let mut digits = 0;
    let mut run = cur;
    while empty > 0 && run < end {
        if t[run].set != ZT_FNC1 {
            digits += 2;
            run += 1;
        }
        run += 1;
        empty -= 1;
    }
    digits
}

/// Distribute the data over rows of `use_columns` data columns, widening the rows when more
/// than 44 rows would be needed. Returns (rows, data columns, per-character set flags).
fn columns_to_rows(t: &[CharSet], mut use_columns: i32) -> Option<(i32, i32, Vec<u32>)> {
    let n = t.len();
    loop {
        let mut set = vec![0u32; n];
        let mut cur = 0usize;
        let mut rows = 0;
        let mut empty;
        loop {
            empty = use_columns;
            let mut cs = if t[cur].a_following > t[cur].b_following {
                CODE_A
            } else {
                CODE_B
            };
            set[cur] = cs;
            let pairs = remaining_digits(t, cur, empty);
            if pairs >= 4 {
                // Would starting one character later find more digits?
                let mut run = cur;
                let mut empty2 = empty;
                while run < n && t[run].set == ZT_FNC1 {
                    run += 1;
                    empty2 -= 1;
                }
                if pairs >= remaining_digits(t, run + 1, empty2 - 1) {
                    cs = CODE_C;
                    set[cur] = CODE_C;
                    if t[cur].set != ZT_FNC1 {
                        cur += 1;
                    }
                }
            }
            cur += 1;
            empty -= 1;

            while empty > 0 && cur < n {
                let is_fnc4 = t[cur].set & CODE_FNC4 != 0;
                if cs == CODE_C {
                    if t[cur].c_following > 0 {
                        cur += if t[cur].set == ZT_FNC1 { 1 } else { 2 };
                        empty -= 1;
                    } else if empty == 1 || (is_fnc4 && empty == 2) {
                        set[cur - 1] |= C_END | C_FILL;
                        empty = 0;
                    } else {
                        cs = if t[cur].a_following > t[cur].b_following {
                            CODE_A
                        } else {
                            CODE_B
                        };
                        set[cur] = cs;
                        empty -= 2;
                        cur += 1;
                    }
                    continue;
                }
                // Code A or B: switch to C when 4+ digits fit and an odd start isn't better.
                if t[cur].set == ZT_NUM {
                    let pairs = remaining_digits(t, cur, empty - 1);
                    if pairs >= 4 && pairs > remaining_digits(t, cur + 1, empty - 2) {
                        cs = CODE_C;
                        set[cur] = CODE_C;
                        cur += 2;
                        empty -= 2;
                        continue;
                    }
                }
                let (this_following, other_following, other) = if cs == CODE_A {
                    (t[cur].a_following, t[cur].b_following, CODE_B)
                } else {
                    (t[cur].b_following, t[cur].a_following, CODE_A)
                };
                if this_following == 0 || (is_fnc4 && this_following == 1) {
                    // Must change set.
                    if empty == 1 || (is_fnc4 && empty == 2) {
                        set[cur - 1] |= C_END | C_FILL;
                        empty = 0;
                    } else {
                        if other_following == 1 || (is_fnc4 && other_following == 2) {
                            // "FNC4 Shift char" order, as Code 128.
                            if is_fnc4 {
                                empty -= 1;
                                cur += 1;
                            }
                            set[cur] |= C_SHIFT;
                        } else {
                            set[cur] |= other;
                            cs = other;
                        }
                        empty -= 2;
                        cur += 1;
                    }
                } else if is_fnc4 && empty == 1 {
                    // Extended character doesn't fit on this row.
                    set[cur - 1] |= C_END | C_FILL;
                    empty = 0;
                } else {
                    empty -= 1;
                    cur += 1;
                }
            }
            set[cur - 1] |= C_END;
            rows += 1;
            if cur >= n {
                break;
            }
        }
        // Room for the K1/K2 check characters.
        match empty {
            0 => rows += 1,
            1 => {
                set[n - 1] |= C_FILL;
                rows += 1;
            }
            2 => {}
            _ => set[n - 1] |= C_FILL,
        }
        if rows > 44 {
            use_columns += 1;
            if use_columns > 62 {
                return None;
            }
            continue;
        }
        return Some((rows.max(2), use_columns, set));
    }
}

/// Find the narrowest row width giving at most `rows_requested` rows (`columns_requested`, if
/// >= 4, is tried first). Fewer rows are padded up to `rows_requested`.
fn rows_to_columns(
    t: &[CharSet],
    rows_requested: i32,
    columns_requested: i32,
) -> Option<(i32, i32, Vec<u32>)> {
    let columns_requested = if columns_requested >= 4 {
        columns_requested
    } else {
        0
    };
    let mut test = if columns_requested > 0 {
        columns_requested
    } else {
        (t.len() as i32 / rows_requested).clamp(4, 62)
    };
    let mut tested = Vec::new();
    loop {
        tested.push(test);
        let (rows, use_columns, set) = columns_to_rows(t, test)?;
        if rows <= rows_requested {
            if rows == 2 || test == 4 || test == columns_requested || tested.contains(&(test - 1)) {
                return Some((rows.max(rows_requested), use_columns, set));
            }
            test -= 1;
        } else {
            test += 1;
            if test > 62 {
                return None;
            }
        }
    }
}

/// Code 128 value of `c` in set A.
fn c128_a(c: u16) -> u8 {
    match c {
        A_CODEB => 100,
        A_FNC4 => 101,
        A_FNC1 => 102,
        A_FNC2 => 97,
        A_FNC3 => 96,
        A_CODEC => 99,
        A_SHIFT => 98,
        32..=95 => (c - 32) as u8,
        _ => (c + 64) as u8,
    }
}

/// Code 128 value of `c` in set B.
fn c128_b(c: u16) -> u8 {
    match c {
        A_FNC1 => 102,
        A_FNC2 => 97,
        A_FNC3 => 96,
        A_FNC4 => 100,
        A_CODEA => 101,
        A_CODEC => 99,
        A_SHIFT => 98,
        _ => (c - 32) as u8,
    }
}

/// Code 128 value of a set switch / FNC1 in set C.
fn c128_c(c: u16) -> u8 {
    match c {
        A_FNC1 => 102,
        A_CODEB => 100,
        A_CODEA => 101,
        _ => unreachable!("not a Code C function character"),
    }
}

fn c128_in(cs: u32, c: u16) -> u8 {
    match cs {
        CODE_A => c128_a(c),
        CODE_B => c128_b(c),
        _ => c128_c(c),
    }
}

/// Row indicator / K1 / K2 value (ISS-X-24 tables D.2, D.3, F.1).
fn sum_value(sum: i32, cs: u32) -> u8 {
    if cs == CODE_C {
        sum as u8
    } else if sum <= 31 {
        (sum + 64) as u8
    } else if sum <= 47 {
        (sum - 32) as u8
    } else {
        (sum - 22) as u8
    }
}

/// Codablock F codewords per row (start, selector, row indicator, data, row check, stop 106).
/// `rows` / `columns` (data columns) are 0 for automatic; at least one must be set.
fn cbf_codewords(
    data: &[u16],
    kmsg: &[u32],
    rows: i32,
    columns: i32,
) -> Result<Vec<Vec<u8>>, SymbolError> {
    let n = data.len();
    let t = char_set_table(data);
    let found = if rows > 0 {
        rows_to_columns(&t, rows, columns)
    } else {
        columns_to_rows(&t, columns.max(4))
    };
    let (rows, use_columns, set) = found.ok_or(SymbolError::InvalidLength(n))?;

    let (mut k1, mut k2) = (0u32, 0u32);
    for (i, &c) in kmsg.iter().enumerate() {
        k1 = (k1 + (i as u32 + 1) * c) % 86;
        k2 = (k2 + i as u32 * c) % 86;
    }

    let mut out = Vec::with_capacity(rows as usize);
    let mut cur = 0usize;
    for row in 0..rows {
        let mut r: Vec<u8> = Vec::with_capacity(use_columns as usize + 5);
        let mut cs;
        if cur >= n {
            // Padding row: StartA, CodeB, row indicator, alternating CodeC/CodeB fillers.
            r.extend([103, 100]);
            cs = CODE_B;
            r.push(sum_value(row + 42, cs));
            let mut empty = use_columns - if row == rows - 1 { 2 } else { 0 };
            while empty > 0 {
                if cs == CODE_C {
                    r.push(100);
                    cs = CODE_B;
                } else {
                    r.push(99);
                    cs = CODE_C;
                }
                empty -= 1;
            }
        } else {
            cs = match set[cur] & (CODE_A | CODE_B | CODE_C) {
                CODE_A => CODE_A,
                CODE_B => CODE_B,
                _ => CODE_C,
            };
            // Start A followed by the code set selector (Shift for A, Code B, Code C).
            r.extend([
                103,
                match cs {
                    CODE_A => 98,
                    CODE_B => 100,
                    _ => 99,
                },
            ]);
            r.push(sum_value(if row == 0 { rows - 2 } else { row + 42 }, cs));
            let mut empty = use_columns;
            while empty > 0 && cur < n {
                if empty < use_columns {
                    for (flag, code) in [(CODE_A, A_CODEA), (CODE_B, A_CODEB), (CODE_C, A_CODEC)] {
                        if set[cur] & flag != 0 {
                            r.push(c128_in(cs, code));
                            empty -= 1;
                            cs = flag;
                            break;
                        }
                    }
                }
                if set[cur] & C_SHIFT != 0 {
                    r.push(c128_in(cs, A_SHIFT));
                    empty -= 2;
                    let other = if cs == CODE_B { CODE_A } else { CODE_B };
                    r.push(c128_in(other, data[cur]));
                } else {
                    if cs == CODE_C {
                        if data[cur] == A_FNC1 {
                            r.push(102);
                        } else {
                            r.push(((data[cur] - 48) * 10 + (data[cur + 1] - 48)) as u8);
                            cur += 1;
                        }
                    } else {
                        r.push(c128_in(cs, data[cur]));
                    }
                    empty -= 1;
                }
                if cur < n && set[cur] & (C_FILL | C_END) != 0 {
                    // Fill the row, leaving room for K1/K2 in the last row.
                    if row == rows - 1 {
                        empty -= 2;
                    }
                    while empty > 0 {
                        if cs == CODE_C {
                            r.push(100);
                            cs = CODE_B;
                        } else {
                            r.push(99);
                            cs = CODE_C;
                        }
                        empty -= 1;
                    }
                }
                cur += 1;
            }
        }
        if row == rows - 1 {
            r.push(sum_value(k1 as i32, cs));
            r.push(sum_value(k2 as i32, cs));
        }
        debug_assert_eq!(r.len(), use_columns as usize + 3);
        let check = r
            .iter()
            .enumerate()
            .map(|(i, &v)| i.max(1) as u32 * v as u32)
            .sum::<u32>()
            % 103;
        r.push(check as u8);
        r.push(106);
        out.push(r);
    }
    Ok(out)
}

/// Single-row Code 128 (used for one-row Codablock F/E): greedy set selection.
/// Returns start, data and check values (no stop).
fn code128_values(data: &[u16]) -> Vec<u8> {
    let n = data.len();
    let is_digit =
        |i: usize| i < n && (48..=57).contains(&data[i]) && (i == 0 || data[i - 1] != A_FNC4);
    let digit_run = |i: usize| (i..n).take_while(|&j| is_digit(j)).count();
    let fits = |cs: u32, c: u16| match c {
        A_FNC1..=A_FNC4 => true,
        _ if cs == CODE_A => c < 96,
        _ => c >= 32,
    };
    // A if a control character comes before any lowercase character.
    let prefer_a = |i: usize| {
        data[i..]
            .iter()
            .find(|&&c| c < 32 || (96..128).contains(&c))
            .is_some_and(|&c| c < 32)
    };
    let mut v = Vec::with_capacity(n + 3);
    let mut cs;
    let run = digit_run(0);
    if run >= 4 || (run == 2 && n == 2) {
        v.push(105);
        cs = CODE_C;
    } else if prefer_a(0) {
        v.push(103);
        cs = CODE_A;
    } else {
        v.push(104);
        cs = CODE_B;
    }
    let mut i = 0;
    while i < n {
        let c = data[i];
        if cs == CODE_C {
            if digit_run(i) >= 2 {
                v.push(((c - 48) * 10 + (data[i + 1] - 48)) as u8);
                i += 2;
            } else if c == A_FNC1 {
                v.push(102);
                i += 1;
            } else {
                cs = if prefer_a(i) { CODE_A } else { CODE_B };
                v.push(if cs == CODE_A { 101 } else { 100 });
            }
            continue;
        }
        let run = digit_run(i);
        if run >= 4 {
            if run % 2 == 1 {
                v.push(c128_in(cs, c));
                i += 1;
            }
            v.push(99);
            cs = CODE_C;
            continue;
        }
        if fits(cs, c) {
            v.push(c128_in(cs, c));
            i += 1;
            continue;
        }
        let other = if cs == CODE_A { CODE_B } else { CODE_A };
        if i + 1 < n && fits(cs, data[i + 1]) {
            v.push(98);
            v.push(c128_in(other, c));
            i += 1;
        } else {
            v.push(if other == CODE_A { 101 } else { 100 });
            cs = other;
        }
    }
    let check = v
        .iter()
        .enumerate()
        .map(|(i, &x)| i.max(1) as u32 * x as u32)
        .sum::<u32>()
        % 103;
    v.push(check as u8);
    v
}

/// Modules of a Code 128 row; a trailing 106 (or a missing one) becomes the 13-module stop.
fn c128_modules(values: &[u8]) -> Vec<bool> {
    let body = values.strip_suffix(&[106]).unwrap_or(values);
    let mut m = Vec::with_capacity(body.len() * 11 + 13);
    for &v in body {
        for (i, &w) in C128_WIDTHS[v as usize].iter().enumerate() {
            push_run(&mut m, i % 2 == 0, w);
        }
    }
    for (i, w) in [2u8, 3, 3, 1, 1, 1, 2].into_iter().enumerate() {
        push_run(&mut m, i % 2 == 0, w);
    }
    m
}

// ---------------------------------------------------------------------------------------------
// Codablock A
// ---------------------------------------------------------------------------------------------
//
// No public specification was available; the layout below mirrors Codablock F with Code 39:
// every row is `* RI data *` (start/stop `*`, 1-module inter-character gaps, 3:1 ratio), the
// row indicator RI is the Code 39 value `rows - 2` in the first row and `row + 21` in the others
// (so 22 rows fit the 43 values, matching the documented 22-row maximum), and with security the
// last row ends with two mod-43 check characters K1 = sum(v) and K2 = sum((i + 1) * v). The last
// row is not padded with data; it simply ends early (light modules after its stop character).

const CODE39_CHARS: &[u8; 43] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ-. $/+%";
/// 9 elements (bar first, MSB first), 1 = wide; index 43 = `*`.
const CODE39_PATTERNS: [u16; 44] = [
    0x034, 0x121, 0x061, 0x160, 0x031, 0x130, 0x070, 0x025, 0x124, 0x064, 0x109, 0x049, 0x148,
    0x019, 0x118, 0x058, 0x00D, 0x10C, 0x04C, 0x01C, 0x103, 0x043, 0x142, 0x013, 0x112, 0x052,
    0x007, 0x106, 0x046, 0x016, 0x181, 0x0C1, 0x1C0, 0x091, 0x190, 0x0D0, 0x085, 0x184, 0x0C4,
    0x0A8, 0x0A2, 0x08A, 0x02A, 0x094,
];
const CODE39_WIDE: u8 = 3;
/// Modules per Code 39 character including the inter-character gap.
const CODE39_PITCH: usize = 6 + 3 * CODE39_WIDE as usize + 1;

fn code39_row(values: &[usize]) -> Vec<bool> {
    let mut m = Vec::new();
    for (k, &v) in std::iter::once(&43)
        .chain(values)
        .chain(std::iter::once(&43))
        .enumerate()
    {
        if k > 0 {
            m.push(false);
        }
        let p = CODE39_PATTERNS[v];
        for e in 0..9 {
            let w = if p >> (8 - e) & 1 == 1 {
                CODE39_WIDE
            } else {
                1
            };
            push_run(&mut m, e % 2 == 0, w);
        }
    }
    m
}

fn codablock_a(
    data: &str,
    columns: u32,
    rows: u32,
    security: bool,
    single: bool,
) -> Result<Stacked, SymbolError> {
    let values = data
        .chars()
        .map(|c| {
            let u = c.to_ascii_uppercase();
            CODE39_CHARS
                .iter()
                .position(|&x| x as char == u)
                .ok_or(SymbolError::InvalidCharacter(c))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let n = values.len();
    let one_row = |values: &[usize]| {
        let row = code39_row(values);
        let w = row.len();
        assemble(vec![row], w, (0, w))
    };
    if single {
        return Ok(one_row(&values));
    }
    let checks = if security { 2 } else { 0 };
    let total = n + checks;
    let rows = rows.min(22) as usize;
    let cols = if columns > 0 {
        (columns as usize).clamp(1, 61)
    } else {
        total.div_ceil(rows).clamp(1, 61)
    };
    let nrows = total.div_ceil(cols);
    if nrows <= 1 {
        return Ok(one_row(&values));
    }
    if nrows > 22 {
        return Err(SymbolError::InvalidLength(n));
    }
    let mut all = values.clone();
    if security {
        let k1 = values.iter().sum::<usize>() % 43;
        let k2 = values
            .iter()
            .enumerate()
            .map(|(i, &v)| (i + 1) * v)
            .sum::<usize>()
            % 43;
        all.extend([k1, k2]);
    }
    let width = (cols + 3) * CODE39_PITCH - 1;
    let data_rows = all
        .chunks(cols)
        .enumerate()
        .map(|(r, chunk)| {
            let ri = if r == 0 { nrows - 2 } else { r + 21 };
            let mut row = vec![ri];
            row.extend_from_slice(chunk);
            code39_row(&row)
        })
        .collect();
    Ok(assemble(
        data_rows,
        width,
        (CODE39_PITCH - 1, width - (CODE39_PITCH - 1)),
    ))
}

// =============================================================================================
// Code 49 (^B4)
// =============================================================================================

/// Code 49 alphanumeric set: values 0-42 (Code 39 set); 43 = Shift 1, 44 = Shift 2,
/// 45-47 = FNC1-FNC3, 48 = Numeric Shift.
const C49_SET: &[u8; 43] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ-. $/+%";
const C49_S1: u8 = 43;
const C49_S2: u8 = 44;
const C49_NS: u8 = 48;

/// ASCII to Code 49 (ANSI/AIM BC6-2000 table 7): `(shift, char)`; shift 0 = none.
fn c49_ascii(c: u8) -> (u8, u8) {
    let v = |ch: u8| C49_SET.iter().position(|&x| x == ch).unwrap() as u8;
    match c {
        0 => (C49_S1, v(b' ')),
        1..=26 => (C49_S1, v(b'A' + c - 1)),
        27..=31 => (C49_S1, v(b'1' + c - 27)),
        b' ' | b'$' | b'%' | b'+' | b'-' | b'.' | b'/' | b'0'..=b'9' | b'A'..=b'Z' => (0, v(c)),
        b'!' | b'"' | b'#' => (C49_S1, v(b'6' + c - b'!')),
        b'&' => (C49_S1, v(b'9')),
        b'\'' => (C49_S1, v(b'0')),
        b'(' => (C49_S1, v(b'-')),
        b')' => (C49_S1, v(b'.')),
        b'*' => (C49_S1, v(b'$')),
        b',' => (C49_S1, v(b'/')),
        b':' => (C49_S1, v(b'+')),
        b';'..=b'@' => (C49_S2, v(b'1' + c - b';')),
        b'[' | b'\\' | b']' => (C49_S2, v(b'7' + c - b'[')),
        b'^' => (C49_S2, v(b'0')),
        b'_' => (C49_S2, v(b'-')),
        b'`' => (C49_S2, v(b'.')),
        b'a'..=b'z' => (C49_S2, v(b'A' + c - b'a')),
        b'{' => (C49_S2, v(b'$')),
        b'|' => (C49_S2, v(b'/')),
        b'}' => (C49_S2, v(b'+')),
        b'~' => (C49_S2, v(b'%')),
        _ => (C49_S2, v(b' ')), // DEL
    }
}

/// Inverse of [`c49_ascii`] for the interpretation line.
fn c49_to_ascii(shift: u8, v: u8) -> Option<char> {
    (0u8..128)
        .find(|&c| c49_ascii(c) == (shift, v))
        .map(char::from)
}

/// Append the numeric encodation of `digits` (ASCII) per BC6-2000 2.3 rules.
fn c49_numeric(digits: &[u8], out: &mut Vec<u8>) {
    let num = |d: &[u8]| d.iter().fold(0u32, |a, &c| a * 10 + u32::from(c - b'0'));
    let push3 = |v: u32, out: &mut Vec<u8>| {
        out.extend([(v / 2304) as u8, (v / 48 % 48) as u8, (v % 48) as u8]);
    };
    let blocks = digits.len() / 5;
    let remain = digits.len() % 5;
    let mut i = 0;
    for b in 0..blocks {
        if b == blocks - 1 && remain == 2 {
            // Rule (d): last 7 digits as 4 + 3.
            push3(100_000 + num(&digits[i..i + 4]), out);
            let v = num(&digits[i + 4..i + 7]);
            out.extend([(v / 48) as u8, (v % 48) as u8]);
            i += 7;
        } else {
            push3(num(&digits[i..i + 5]), out);
            i += 5;
        }
    }
    match remain {
        1 => out.push(digits[i] - b'0'), // Rule (a)
        3 => {
            // Rule (b)
            let v = num(&digits[i..i + 3]);
            out.extend([(v / 48) as u8, (v % 48) as u8]);
        }
        4 => push3(100_000 + num(&digits[i..i + 4]), out), // Rule (c)
        _ => {}
    }
}

/// Automatic mode codewords (zint/BC6-2000): returns (codewords incl. leading shift/NS, M).
fn c49_auto(data: &[u8]) -> Vec<u8> {
    let mut inter = Vec::with_capacity(data.len() * 2);
    for &c in data {
        let (s, v) = c49_ascii(c);
        if s != 0 {
            inter.push(s);
        }
        inter.push(v);
    }
    let is_digit = |v: u8| v < 10;
    let mut cw = Vec::with_capacity(inter.len());
    let mut i = 0;
    while i < inter.len() {
        let run = inter[i..].iter().take_while(|&&v| is_digit(v)).count();
        // A digit right after Shift 1/2 is a shifted character, not numeric data (zint
        // starts the numeric run there, producing an undecodable "shift, NS" sequence).
        let shifted = i > 0 && matches!(inter[i - 1], C49_S1 | C49_S2);
        if run >= 5 && !shifted {
            cw.push(C49_NS);
            let digits: Vec<u8> = inter[i..i + run].iter().map(|&v| v + b'0').collect();
            c49_numeric(&digits, &mut cw);
            i += run;
            if i < inter.len() {
                cw.push(C49_NS);
            }
        } else {
            cw.push(inter[i]);
            i += 1;
        }
    }
    cw
}

/// Manual modes 0-5: field data in the Zebra Code 49 internal character set (`<` Shift 1,
/// `>` Shift 2, `=` Numeric Shift). Stops at the first invalid sequence, like the printer.
/// Returns (codewords without the implied leading shift, interpretation text).
fn c49_manual(data: &str, mode: u8) -> (Vec<u8>, String) {
    let base = |c: char| {
        C49_SET
            .iter()
            .position(|&x| x as char == c)
            .map(|p| p as u8)
    };
    let chars: Vec<char> = data.chars().collect();
    let mut cw = Vec::new();
    let mut text = String::new();
    let mut numeric = mode == 2;
    let mut i = 0;
    let mut pending_shift = match mode {
        4 => C49_S1,
        5 => C49_S2,
        _ => 0,
    };
    let implied = pending_shift != 0;
    'outer: while i < chars.len() {
        if numeric {
            let run = chars[i..].iter().take_while(|c| c.is_ascii_digit()).count();
            let digits: String = chars[i..i + run].iter().collect();
            // A lone pair of digits has no numeric encodation; treat as invalid.
            let usable = if run % 5 == 2 && run < 5 { 0 } else { run };
            c49_numeric(&digits.as_bytes()[..usable], &mut cw);
            text.push_str(&digits[..usable]);
            i += usable;
            if usable < run || i >= chars.len() || chars[i] != '=' {
                break;
            }
            cw.push(C49_NS);
            numeric = false;
            i += 1;
            continue;
        }
        let c = chars[i];
        match c {
            '<' | '>' if pending_shift == 0 => {
                pending_shift = if c == '<' { C49_S1 } else { C49_S2 };
                cw.push(pending_shift);
            }
            '=' if pending_shift == 0 => {
                cw.push(C49_NS);
                numeric = true;
            }
            _ => {
                let Some(v) = base(c) else { break 'outer };
                if pending_shift != 0 {
                    let Some(ch) = c49_to_ascii(pending_shift, v) else {
                        break 'outer; // reserved (Shift 1 %)
                    };
                    text.push(ch);
                    pending_shift = 0;
                } else {
                    text.push(c);
                }
                cw.push(v);
            }
        }
        i += 1;
    }
    if pending_shift != 0 && !(implied && cw.is_empty()) {
        cw.pop(); // dangling shift
    }
    (cw, text)
}

/// Encode Code 49 (ZPL `^B4o,h,f,m`).
///
/// `start_mode` = `^B4 m`: `None` (`A`, automatic) encodes any ASCII text. `Some(0..=5)` takes
/// the field data in the Code 49 internal character set (`0-9 A-Z - . space $ / + %`, `<` =
/// Shift 1, `>` = Shift 2, `=` = Numeric Shift; in numeric mode only digits) and stops at the
/// first invalid sequence, like the printer. Modes 4/5 apply an implied Shift 1/2 to the first
/// character. Returns the symbol and the interpretation line text (the decoded ASCII).
pub fn code49(data: &str, start_mode: Option<u8>) -> Result<(Stacked, String), SymbolError> {
    let (cw, m, text) = match start_mode {
        Some(m @ 0..=5) => {
            let (cw, text) = c49_manual(data, m);
            (cw, m, text)
        }
        _ => {
            if let Some(c) = data.chars().find(|c| !c.is_ascii()) {
                return Err(SymbolError::InvalidCharacter(c));
            }
            if data.len() > 81 {
                return Err(SymbolError::InvalidLength(data.len()));
            }
            let mut cw = c49_auto(data.as_bytes());
            let m = match cw.first() {
                Some(&C49_NS) => 2,
                Some(&C49_S1) => 4,
                Some(&C49_S2) => 5,
                _ => 0,
            };
            if m != 0 {
                cw.remove(0);
            }
            (cw, m, data.to_string())
        }
    };
    if cw.is_empty() && data.is_empty() {
        return Err(SymbolError::InvalidLength(0));
    }
    if cw.len() > 49 {
        return Err(SymbolError::InvalidLength(data.chars().count()));
    }
    if cw.is_empty() && start_mode.is_some() {
        return Err(SymbolError::InvalidData("no valid Code 49 data".into()));
    }
    let grid = c49_grid(&cw, m);
    let rows: Vec<Vec<bool>> = c49_rows(&grid);
    Ok((assemble(rows, 70, (0, 70)), text))
}

const C49_X_WEIGHT: [u32; 32] = [
    1, 9, 31, 26, 2, 12, 17, 23, 37, 18, 22, 6, 27, 44, 15, 43, 39, 11, 13, 5, 41, 33, 36, 8, 4,
    32, 3, 19, 40, 25, 29, 10,
];
const C49_Y_WEIGHT: [u32; 32] = [
    9, 31, 26, 2, 12, 17, 23, 37, 18, 22, 6, 27, 44, 15, 43, 39, 11, 13, 5, 41, 33, 36, 8, 4, 32,
    3, 19, 40, 25, 29, 10, 24,
];
const C49_Z_WEIGHT: [u32; 32] = [
    31, 26, 2, 12, 17, 23, 37, 18, 22, 6, 27, 44, 15, 43, 39, 11, 13, 5, 41, 33, 36, 8, 4, 32, 3,
    19, 40, 25, 29, 10, 24, 30,
];
/// Row parity patterns (BC6-2000 table 4), `true` = odd parity.
const C49_PARITY: [[bool; 4]; 8] = [
    [true, false, false, true],
    [false, true, false, true],
    [true, true, false, false],
    [false, false, true, true],
    [true, false, true, false],
    [false, true, true, false],
    [true, true, true, true],
    [false, false, false, false],
];

/// Lay out up to 49 codewords into the code character grid (8 values per row: 7 data + row
/// check; the last row holds the Z/Y/X symbol checks and the row count/mode character).
fn c49_grid(cw: &[u8], m: u8) -> Vec<[u8; 8]> {
    let mut grid: Vec<[u8; 8]> = Vec::new();
    let mut pad_count = 0;
    loop {
        let r = grid.len();
        let mut row = [C49_NS; 8];
        for (i, slot) in row.iter_mut().take(7).enumerate() {
            match cw.get(r * 7 + i) {
                Some(&v) => *slot = v,
                None => pad_count += 1,
            }
        }
        grid.push(row);
        if grid.len() * 7 >= cw.len() {
            break;
        }
    }
    let rows = grid.len();
    if pad_count < 5 || rows > 6 || rows == 1 {
        grid.push([C49_NS; 8]);
    }
    let rows = grid.len();
    let last = rows - 1;
    grid[last][6] = (7 * (rows - 2)) as u8 + m;
    for row in grid.iter_mut().take(last) {
        row[7] = (row[..7].iter().map(|&v| u32::from(v)).sum::<u32>() % 49) as u8;
    }
    let ri = u32::from(grid[last][6]);
    let (mut x, mut y, mut z) = (ri * 20, ri * 16, ri * 38);
    let mut pos = 0;
    for row in grid.iter().take(last) {
        for j in 0..4 {
            let v = u32::from(row[2 * j]) * 49 + u32::from(row[2 * j + 1]);
            x += C49_X_WEIGHT[pos] * v;
            y += C49_Y_WEIGHT[pos] * v;
            z += C49_Z_WEIGHT[pos] * v;
            pos += 1;
        }
    }
    if rows > 6 {
        z %= 2401;
        grid[last][0] = (z / 49) as u8;
        grid[last][1] = (z % 49) as u8;
    }
    let v = u32::from(grid[last][0]) * 49 + u32::from(grid[last][1]);
    x += C49_X_WEIGHT[pos] * v;
    y += C49_Y_WEIGHT[pos] * v;
    pos += 1;
    y %= 2401;
    grid[last][2] = (y / 49) as u8;
    grid[last][3] = (y % 49) as u8;
    let v = u32::from(grid[last][2]) * 49 + u32::from(grid[last][3]);
    x += C49_X_WEIGHT[pos] * v;
    x %= 2401;
    grid[last][4] = (x / 49) as u8;
    grid[last][5] = (x % 49) as u8;
    grid[last][7] = (grid[last][..7].iter().map(|&v| u32::from(v)).sum::<u32>() % 49) as u8;
    grid
}

/// Row modules: start `10`, four 16-module symbol characters, stop `1111` (70 modules).
fn c49_rows(grid: &[[u8; 8]]) -> Vec<Vec<bool>> {
    let last = grid.len() - 1;
    grid.iter()
        .enumerate()
        .map(|(r, row)| {
            let mut m = vec![true, false];
            for j in 0..4 {
                let w = usize::from(row[2 * j]) * 49 + usize::from(row[2 * j + 1]);
                let odd = r != last && C49_PARITY[r][j];
                let p = if odd { C49_ODD[w] } else { C49_EVEN[w] };
                m.extend((0..16).map(|b| p >> (15 - b) & 1 == 1));
            }
            m.extend([true; 4]);
            m
        })
        .collect()
}

// =============================================================================================
// Tables
// =============================================================================================

/// Code 128 symbol character widths (bar, space, bar, space, bar, space), values 0-105.
const C128_WIDTHS: [[u8; 6]; 106] = [
    [2, 1, 2, 2, 2, 2],
    [2, 2, 2, 1, 2, 2],
    [2, 2, 2, 2, 2, 1],
    [1, 2, 1, 2, 2, 3],
    [1, 2, 1, 3, 2, 2],
    [1, 3, 1, 2, 2, 2],
    [1, 2, 2, 2, 1, 3],
    [1, 2, 2, 3, 1, 2],
    [1, 3, 2, 2, 1, 2],
    [2, 2, 1, 2, 1, 3],
    [2, 2, 1, 3, 1, 2],
    [2, 3, 1, 2, 1, 2],
    [1, 1, 2, 2, 3, 2],
    [1, 2, 2, 1, 3, 2],
    [1, 2, 2, 2, 3, 1],
    [1, 1, 3, 2, 2, 2],
    [1, 2, 3, 1, 2, 2],
    [1, 2, 3, 2, 2, 1],
    [2, 2, 3, 2, 1, 1],
    [2, 2, 1, 1, 3, 2],
    [2, 2, 1, 2, 3, 1],
    [2, 1, 3, 2, 1, 2],
    [2, 2, 3, 1, 1, 2],
    [3, 1, 2, 1, 3, 1],
    [3, 1, 1, 2, 2, 2],
    [3, 2, 1, 1, 2, 2],
    [3, 2, 1, 2, 2, 1],
    [3, 1, 2, 2, 1, 2],
    [3, 2, 2, 1, 1, 2],
    [3, 2, 2, 2, 1, 1],
    [2, 1, 2, 1, 2, 3],
    [2, 1, 2, 3, 2, 1],
    [2, 3, 2, 1, 2, 1],
    [1, 1, 1, 3, 2, 3],
    [1, 3, 1, 1, 2, 3],
    [1, 3, 1, 3, 2, 1],
    [1, 1, 2, 3, 1, 3],
    [1, 3, 2, 1, 1, 3],
    [1, 3, 2, 3, 1, 1],
    [2, 1, 1, 3, 1, 3],
    [2, 3, 1, 1, 1, 3],
    [2, 3, 1, 3, 1, 1],
    [1, 1, 2, 1, 3, 3],
    [1, 1, 2, 3, 3, 1],
    [1, 3, 2, 1, 3, 1],
    [1, 1, 3, 1, 2, 3],
    [1, 1, 3, 3, 2, 1],
    [1, 3, 3, 1, 2, 1],
    [3, 1, 3, 1, 2, 1],
    [2, 1, 1, 3, 3, 1],
    [2, 3, 1, 1, 3, 1],
    [2, 1, 3, 1, 1, 3],
    [2, 1, 3, 3, 1, 1],
    [2, 1, 3, 1, 3, 1],
    [3, 1, 1, 1, 2, 3],
    [3, 1, 1, 3, 2, 1],
    [3, 3, 1, 1, 2, 1],
    [3, 1, 2, 1, 1, 3],
    [3, 1, 2, 3, 1, 1],
    [3, 3, 2, 1, 1, 1],
    [3, 1, 4, 1, 1, 1],
    [2, 2, 1, 4, 1, 1],
    [4, 3, 1, 1, 1, 1],
    [1, 1, 1, 2, 2, 4],
    [1, 1, 1, 4, 2, 2],
    [1, 2, 1, 1, 2, 4],
    [1, 2, 1, 4, 2, 1],
    [1, 4, 1, 1, 2, 2],
    [1, 4, 1, 2, 2, 1],
    [1, 1, 2, 2, 1, 4],
    [1, 1, 2, 4, 1, 2],
    [1, 2, 2, 1, 1, 4],
    [1, 2, 2, 4, 1, 1],
    [1, 4, 2, 1, 1, 2],
    [1, 4, 2, 2, 1, 1],
    [2, 4, 1, 2, 1, 1],
    [2, 2, 1, 1, 1, 4],
    [4, 1, 3, 1, 1, 1],
    [2, 4, 1, 1, 1, 2],
    [1, 3, 4, 1, 1, 1],
    [1, 1, 1, 2, 4, 2],
    [1, 2, 1, 1, 4, 2],
    [1, 2, 1, 2, 4, 1],
    [1, 1, 4, 2, 1, 2],
    [1, 2, 4, 1, 1, 2],
    [1, 2, 4, 2, 1, 1],
    [4, 1, 1, 2, 1, 2],
    [4, 2, 1, 1, 1, 2],
    [4, 2, 1, 2, 1, 1],
    [2, 1, 2, 1, 4, 1],
    [2, 1, 4, 1, 2, 1],
    [4, 1, 2, 1, 2, 1],
    [1, 1, 1, 1, 4, 3],
    [1, 1, 1, 3, 4, 1],
    [1, 3, 1, 1, 4, 1],
    [1, 1, 4, 1, 1, 3],
    [1, 1, 4, 3, 1, 1],
    [4, 1, 1, 1, 1, 3],
    [4, 1, 1, 3, 1, 1],
    [1, 1, 3, 1, 4, 1],
    [1, 1, 4, 1, 3, 1],
    [3, 1, 1, 1, 4, 1],
    [4, 1, 1, 1, 3, 1],
    [2, 1, 1, 4, 1, 2],
    [2, 1, 1, 2, 1, 4],
    [2, 1, 1, 2, 3, 2],
];

const C49_EVEN: [u16; 2401] = [
    0xBE5C, 0xC16E, 0x86DC, 0xC126, 0x864C, 0x9EDC, 0xC726, 0x9E4C, 0xDF26, 0x82CC, 0x8244, 0x8ECC,
    0xC322, 0x8E44, 0xBECC, 0xCF22, 0xBE44, 0xC162, 0x86C4, 0xC762, 0x9EC4, 0xDF62, 0x812E, 0x872E,
    0x9F2E, 0x836E, 0x8326, 0x8F6E, 0x8F26, 0xBF6E, 0x8166, 0x8122, 0x8766, 0x8722, 0x9F66, 0x9F22,
    0x8362, 0x8F62, 0xBF62, 0xA2E0, 0xE8B8, 0xFA2E, 0xD370, 0xF4DC, 0xD130, 0xF44C, 0xAEE0, 0xEBB8,
    0xFAEE, 0xA660, 0xE998, 0xFA66, 0xA220, 0xE888, 0xFA22, 0xD730, 0xF5CC, 0xD310, 0xF4C4, 0xAE20,
    0xEB88, 0xFAE2, 0x9170, 0xE45C, 0xD8B8, 0xF62E, 0xC9B8, 0xF26E, 0xB370, 0xC898, 0xF226, 0xB130,
    0xEC4C, 0x9770, 0xE5DC, 0x9330, 0xE4CC, 0x9110, 0xE444, 0xD888, 0xF622, 0xCB98, 0xF2E6, 0xB730,
    0xC988, 0xF262, 0xB310, 0xECC4, 0x9710, 0xE5C4, 0xDB88, 0xF6E2, 0x88B8, 0xE22E, 0xCC5C, 0xB8B8,
    0xEE2E, 0xC4DC, 0x99B8, 0xC44C, 0x9898, 0xE626, 0xDC4C, 0x8BB8, 0xE2EE, 0x8998, 0xE266, 0xBBB8,
    0x8888, 0xE222, 0xB998, 0xCC44, 0xB888, 0xEE22, 0xC5CC, 0x9B98, 0xC4C4, 0x9988, 0xE662, 0xDCC4,
    0x8B88, 0xE2E2, 0xCDC4, 0xBB88, 0xEEE2, 0x845C, 0xC62E, 0x9C5C, 0xDE2E, 0xC26E, 0x8CDC, 0xC226,
    0x8C4C, 0xBCDC, 0xCE26, 0xBC4C, 0x85DC, 0x84CC, 0x9DDC, 0x8444, 0x9CCC, 0xC622, 0x9C44, 0xDE22,
    0xC2E6, 0x8DCC, 0xC262, 0x8CC4, 0xBDCC, 0xCE62, 0xBCC4, 0x85C4, 0xC6E2, 0x9DC4, 0xDEE2, 0x822E,
    0x8E2E, 0x866E, 0x8626, 0x9E6E, 0x9E26, 0x82EE, 0x8266, 0x8EEE, 0x8222, 0x8E66, 0xBEEE, 0x8E22,
    0xBE66, 0x86E6, 0x8662, 0x9EE6, 0x9E62, 0x82E2, 0x8EE2, 0xBEE2, 0xA170, 0xE85C, 0xD1B8, 0xF46E,
    0xD098, 0xF426, 0xA770, 0xE9DC, 0xA330, 0xE8CC, 0xA110, 0xE844, 0xD7B8, 0xF5EE, 0xD398, 0xF4E6,
    0xD188, 0xF462, 0xAF30, 0xEBCC, 0xA710, 0xE9C4, 0xD788, 0xF5E2, 0x90B8, 0xE42E, 0xD85C, 0xC8DC,
    0xB1B8, 0xC84C, 0xB098, 0xEC26, 0x93B8, 0xE4EE, 0x9198, 0xE466, 0x9088, 0xE422, 0xD844, 0xCBDC,
    0xB7B8, 0xC9CC, 0xB398, 0xC8C4, 0xB188, 0xEC62, 0x9798, 0xE5E6, 0x9388, 0xE4E2, 0xD9C4, 0xCBC4,
    0xB788, 0xEDE2, 0x885C, 0xCC2E, 0xB85C, 0xC46E, 0x98DC, 0xC426, 0x984C, 0xDC26, 0x89DC, 0x88CC,
    0xB9DC, 0x8844, 0xB8CC, 0xCC22, 0xB844, 0xC5EE, 0x9BDC, 0xC4E6, 0x99CC, 0xC462, 0x98C4, 0xDC62,
    0x8BCC, 0x89C4, 0xBBCC, 0xCCE2, 0xB9C4, 0xC5E2, 0x9BC4, 0xDDE2, 0x842E, 0x9C2E, 0x8C6E, 0x8C26,
    0xBC6E, 0x84EE, 0x8466, 0x9CEE, 0x8422, 0x9C66, 0x9C22, 0x8DEE, 0x8CE6, 0xBDEE, 0x8C62, 0xBCE6,
    0xBC62, 0x85E6, 0x84E2, 0x9DE6, 0x9CE2, 0x8DE2, 0xBDE2, 0xA0B8, 0xE82E, 0xD0DC, 0xD04C, 0xA3B8,
    0xE8EE, 0xA198, 0xE866, 0xA088, 0xE822, 0xD3DC, 0xD1CC, 0xD0C4, 0xAFB8, 0xEBEE, 0xA798, 0xE9E6,
    0xA388, 0xE8E2, 0xD7CC, 0xD3C4, 0x905C, 0xD82E, 0xC86E, 0xB0DC, 0xC826, 0xB04C, 0x91DC, 0x90CC,
    0x9044, 0xD822, 0xC9EE, 0xB3DC, 0xC8E6, 0xB1CC, 0xC862, 0xB0C4, 0x97DC, 0x93CC, 0x91C4, 0xD8E2,
    0xCBE6, 0xB7CC, 0xC9E2, 0xB3C4, 0x882E, 0x986E, 0x9826, 0x88EE, 0x8866, 0xB8EE, 0x8822, 0xB866,
    0x99EE, 0x98E6, 0x9862, 0x8BEE, 0x89E6, 0xBBEE, 0x88E2, 0xB9E6, 0xB8E2, 0x9BE6, 0x99E2, 0xA05C,
    0xD06E, 0xD026, 0xA1DC, 0xA0CC, 0xA044, 0xD1EE, 0xD0E6, 0xD062, 0xA7DC, 0xA3CC, 0xA1C4, 0xD7EE,
    0xD3E6, 0xD1E2, 0x902E, 0xB06E, 0x90EE, 0x9066, 0x9022, 0xB1EE, 0xB0E6, 0xB062, 0x93EE, 0x91E6,
    0x90E2, 0xB7EE, 0xB3E6, 0xB1E2, 0xA9C0, 0xEA70, 0xFA9C, 0xD460, 0xF518, 0xFD46, 0xA840, 0xEA10,
    0xFA84, 0xED78, 0xFB5E, 0x94E0, 0xE538, 0xF94E, 0xDA70, 0xF69C, 0xCA30, 0xF28C, 0xB460, 0xED18,
    0xFB46, 0x9420, 0xE508, 0xF942, 0xDA10, 0xF684, 0x9AF0, 0xE6BC, 0xDD78, 0xF75E, 0x8A70, 0xE29C,
    0xCD38, 0xF34E, 0xBA70, 0xEE9C, 0xC518, 0xF146, 0x9A30, 0xE68C, 0xDD18, 0xF746, 0x8A10, 0xE284,
    0xCD08, 0xF342, 0xBA10, 0xEE84, 0x8D78, 0xE35E, 0xCEBC, 0xBD78, 0xEF5E, 0x8538, 0xE14E, 0xC69C,
    0x9D38, 0xE74E, 0xDE9C, 0xC28C, 0x8D18, 0xE346, 0xCE8C, 0xBD18, 0xEF46, 0x8508, 0xE142, 0xC684,
    0x9D08, 0xE742, 0xDE84, 0x86BC, 0xC75E, 0x9EBC, 0xDF5E, 0x829C, 0xC34E, 0x8E9C, 0xCF4E, 0xBE9C,
    0xC146, 0x868C, 0xC746, 0x9E8C, 0xDF46, 0x8284, 0xC342, 0x8E84, 0xCF42, 0xBE84, 0x835E, 0x8F5E,
    0xBF5E, 0x814E, 0x874E, 0x9F4E, 0x8346, 0x8F46, 0xBF46, 0x8142, 0x8742, 0x9F42, 0xD2F0, 0xF4BC,
    0xADE0, 0xEB78, 0xFADE, 0xA4E0, 0xE938, 0xFA4E, 0xD670, 0xF59C, 0xD230, 0xF48C, 0xAC60, 0xEB18,
    0xFAC6, 0xA420, 0xE908, 0xFA42, 0xD610, 0xF584, 0xC978, 0xF25E, 0xB2F0, 0xECBC, 0x96F0, 0xE5BC,
    0x9270, 0xE49C, 0xD938, 0xF64E, 0xCB38, 0xF2CE, 0xB670, 0xC918, 0xF246, 0xB230, 0xEC8C, 0x9630,
    0xE58C, 0x9210, 0xE484, 0xD908, 0xF642, 0xCB08, 0xF2C2, 0xB610, 0xED84, 0xC4BC, 0x9978, 0xE65E,
    0xDCBC, 0x8B78, 0xE2DE, 0x8938, 0xE24E, 0xBB78, 0xCC9C, 0xB938, 0xEE4E, 0xC59C, 0x9B38, 0xC48C,
    0x9918, 0xE646, 0xDC8C, 0x8B18, 0xE2C6, 0x8908, 0xE242, 0xBB18, 0xCC84, 0xB908, 0xEE42, 0xC584,
    0x9B08, 0xE6C2, 0xDD84, 0xC25E, 0x8CBC, 0xCE5E, 0xBCBC, 0x85BC, 0x849C, 0x9DBC, 0xC64E, 0x9C9C,
    0xDE4E, 0xC2CE, 0x8D9C, 0xC246, 0x8C8C, 0xBD9C, 0xCE46, 0xBC8C, 0x858C, 0x8484, 0x9D8C, 0xC642,
    0x9C84, 0xDE42, 0xC2C2, 0x8D84, 0xCEC2, 0xBD84, 0x865E, 0x9E5E, 0x82DE, 0x824E, 0x8EDE, 0x8E4E,
    0xBEDE, 0xBE4E, 0x86CE, 0x8646, 0x9ECE, 0x9E46, 0x82C6, 0x8242, 0x8EC6, 0x8E42, 0xBEC6, 0xBE42,
    0x86C2, 0x9EC2, 0xD178, 0xF45E, 0xA6F0, 0xE9BC, 0xA270, 0xE89C, 0xD778, 0xF5DE, 0xD338, 0xF4CE,
    0xD118, 0xF446, 0xAE70, 0xEB9C, 0xA630, 0xE98C, 0xA210, 0xE884, 0xD718, 0xF5C6, 0xD308, 0xF4C2,
    0xAE10, 0xEB84, 0xC8BC, 0xB178, 0xEC5E, 0x9378, 0xE4DE, 0x9138, 0xE44E, 0xD89C, 0xCBBC, 0xB778,
    0xC99C, 0xB338, 0xC88C, 0xB118, 0xEC46, 0x9738, 0xE5CE, 0x9318, 0xE4C6, 0x9108, 0xE442, 0xD884,
    0xCB8C, 0xB718, 0xC984, 0xB308, 0xECC2, 0x9708, 0xE5C2, 0xDB84, 0xC45E, 0x98BC, 0xDC5E, 0x89BC,
    0x889C, 0xB9BC, 0xCC4E, 0xB89C, 0xC5DE, 0x9BBC, 0xC4CE, 0x999C, 0xC446, 0x988C, 0xDC46, 0x8B9C,
    0x898C, 0xBB9C, 0x8884, 0xB98C, 0xCC42, 0xB884, 0xC5C6, 0x9B8C, 0xC4C2, 0x9984, 0xDCC2, 0x8B84,
    0xCDC2, 0xBB84, 0x8C5E, 0xBC5E, 0x84DE, 0x844E, 0x9CDE, 0x9C4E, 0x8DDE, 0x8CCE, 0xBDDE, 0x8C46,
    0xBCCE, 0xBC46, 0x85CE, 0x84C6, 0x9DCE, 0x8442, 0x9CC6, 0x9C42, 0x8DC6, 0x8CC2, 0xBDC6, 0xBCC2,
    0x85C2, 0x9DC2, 0xD0BC, 0xA378, 0xE8DE, 0xA138, 0xE84E, 0xD3BC, 0xD19C, 0xD08C, 0xAF78, 0xEBDE,
    0xA738, 0xE9CE, 0xA318, 0xE8C6, 0xA108, 0xE842, 0xD79C, 0xD38C, 0xD184, 0xAF18, 0xEBC6, 0xA708,
    0xE9C2, 0xC85E, 0xB0BC, 0x91BC, 0x909C, 0xD84E, 0xC9DE, 0xB3BC, 0xC8CE, 0xB19C, 0xC846, 0xB08C,
    0x97BC, 0x939C, 0x918C, 0x9084, 0xD842, 0xCBCE, 0xB79C, 0xC9C6, 0xB38C, 0xC8C2, 0xB184, 0x978C,
    0x9384, 0xD9C2, 0x985E, 0x88DE, 0x884E, 0xB8DE, 0xB84E, 0x99DE, 0x98CE, 0x9846, 0x8BDE, 0x89CE,
    0xBBDE, 0x88C6, 0xB9CE, 0x8842, 0xB8C6, 0xB842, 0x9BCE, 0x99C6, 0x98C2, 0x8BC6, 0x89C2, 0xBBC6,
    0xB9C2, 0xD05E, 0xA1BC, 0xA09C, 0xD1DE, 0xD0CE, 0xD046, 0xA7BC, 0xA39C, 0xA18C, 0xA084, 0xD7DE,
    0xD3CE, 0xD1C6, 0xD0C2, 0xAF9C, 0xA78C, 0xA384, 0xB05E, 0x90DE, 0x904E, 0xB1DE, 0xB0CE, 0xB046,
    0x93DE, 0x91CE, 0x90C6, 0x9042, 0xB7DE, 0xB3CE, 0xB1C6, 0xB0C2, 0x97CE, 0x93C6, 0x91C2, 0xA0DE,
    0xA04E, 0xA3DE, 0xA1CE, 0xA0C6, 0xA042, 0xAFDE, 0xA7CE, 0xA3C6, 0xA1C2, 0xD4F0, 0xF53C, 0xA8E0,
    0xEA38, 0xFA8E, 0xD430, 0xF50C, 0xA820, 0xEA08, 0xFA82, 0xDAF8, 0xF6BE, 0xCA78, 0xF29E, 0xB4F0,
    0xED3C, 0x9470, 0xE51C, 0xDA38, 0xF68E, 0xCA18, 0xF286, 0xB430, 0xED0C, 0x9410, 0xE504, 0xDA08,
    0xF682, 0xCD7C, 0xBAF8, 0xEEBE, 0xC53C, 0x9A78, 0xE69E, 0xDD3C, 0x8A38, 0xE28E, 0xCD1C, 0xBA38,
    0xEE8E, 0xC50C, 0x9A18, 0xE686, 0xDD0C, 0x8A08, 0xE282, 0xCD04, 0xBA08, 0xEE82, 0xC6BE, 0x9D7C,
    0xDEBE, 0xC29E, 0x8D3C, 0xCE9E, 0xBD3C, 0x851C, 0xC68E, 0x9D1C, 0xDE8E, 0xC286, 0x8D0C, 0xCE86,
    0xBD0C, 0x8504, 0xC682, 0x9D04, 0xDE82, 0x8EBE, 0xBEBE, 0x869E, 0x9E9E, 0x828E, 0x8E8E, 0xBE8E,
    0x8686, 0x9E86, 0x8282, 0x8E82, 0xBE82, 0xE97C, 0xD6F8, 0xF5BE, 0xD278, 0xF49E, 0xACF0, 0xEB3C,
    0xA470, 0xE91C, 0xD638, 0xF58E, 0xD218, 0xF486, 0xAC30, 0xEB0C, 0xA410, 0xE904, 0xD608, 0xF582,
    0x92F8, 0xE4BE, 0xD97C, 0xCB7C, 0xB6F8, 0xC93C, 0xB278, 0xEC9E, 0x9678, 0xE59E, 0x9238, 0xE48E,
    0xD91C, 0xCB1C, 0xB638, 0xC90C, 0xB218, 0xEC86, 0x9618, 0xE586, 0x9208, 0xE482, 0xD904, 0xCB04,
    0xB608, 0xED82, 0x897C, 0xCCBE, 0xB97C, 0xC5BE, 0x9B7C, 0xC49E, 0x993C, 0xDC9E, 0x8B3C, 0x891C,
    0xBB3C, 0xCC8E, 0xB91C, 0xC58E, 0x9B1C, 0xC486, 0x990C, 0xDC86, 0x8B0C, 0x8904, 0xBB0C, 0xCC82,
    0xB904, 0xC582, 0x9B04, 0xDD82, 0x84BE, 0x9CBE, 0x8DBE, 0x8C9E, 0xBDBE, 0xBC9E, 0x859E, 0x848E,
    0x9D9E, 0x9C8E, 0x8D8E, 0x8C86, 0xBD8E, 0xBC86, 0x8586, 0x8482, 0x9D86, 0x9C82, 0x8D82, 0xBD82,
    0xA2F8, 0xE8BE, 0xD37C, 0xD13C, 0xAEF8, 0xEBBE, 0xA678, 0xE99E, 0xA238, 0xE88E, 0xD73C, 0xD31C,
    0xD10C, 0xAE38, 0xEB8E, 0xA618, 0xE986, 0xA208, 0xE882, 0xD70C, 0xD304, 0x917C, 0xD8BE, 0xC9BE,
    0xB37C, 0xC89E, 0xB13C, 0x977C, 0x933C, 0x911C, 0xD88E, 0xCB9E, 0xB73C, 0xC98E, 0xB31C, 0xC886,
    0xB10C, 0x971C, 0x930C, 0x9104, 0xD882, 0xCB86, 0xB70C, 0xC982, 0xB304, 0x88BE, 0xB8BE, 0x99BE,
    0x989E, 0x8BBE, 0x899E, 0xBBBE, 0x888E, 0xB99E, 0xB88E, 0x9B9E, 0x998E, 0x9886, 0x8B8E, 0x8986,
    0xBB8E, 0x8882, 0xB986, 0xB882, 0x9B86, 0x9982, 0xA17C, 0xD1BE, 0xD09E, 0xA77C, 0xA33C, 0xA11C,
    0xD7BE, 0xD39E, 0xD18E, 0xD086, 0xAF3C, 0xA71C, 0xA30C, 0xA104, 0xD78E, 0xD386, 0xD182, 0x90BE,
    0xB1BE, 0xB09E, 0x93BE, 0x919E, 0x908E, 0xB7BE, 0xB39E, 0xB18E, 0xB086, 0x979E, 0x938E, 0x9186,
    0x9082, 0xB78E, 0xB386, 0xB182, 0xA0BE, 0xA3BE, 0xA19E, 0xA08E, 0xAFBE, 0xA79E, 0xA38E, 0xA186,
    0xA082, 0xA9F0, 0xEA7C, 0xD478, 0xF51E, 0xA870, 0xEA1C, 0xD418, 0xF506, 0xA810, 0xEA04, 0xED7E,
    0x94F8, 0xE53E, 0xDA7C, 0xCA3C, 0xB478, 0xED1E, 0x9438, 0xE50E, 0xDA1C, 0xCA0C, 0xB418, 0xED06,
    0x9408, 0xE502, 0xDA04, 0x9AFC, 0xDD7E, 0x8A7C, 0xCD3E, 0xBA7C, 0xC51E, 0x9A3C, 0xDD1E, 0x8A1C,
    0xCD0E, 0xBA1C, 0xC506, 0x9A0C, 0xDD06, 0x8A04, 0xCD02, 0xBA04, 0x8D7E, 0xBD7E, 0x853E, 0x9D3E,
    0x8D1E, 0xBD1E, 0x850E, 0x9D0E, 0x8D06, 0xBD06, 0x8502, 0x9D02, 0xD2FC, 0xADF8, 0xEB7E, 0xA4F8,
    0xE93E, 0xD67C, 0xD23C, 0xAC78, 0xEB1E, 0xA438, 0xE90E, 0xD61C, 0xD20C, 0xAC18, 0xEB06, 0xA408,
    0xE902, 0xC97E, 0xB2FC, 0x96FC, 0x927C, 0xD93E, 0xCB3E, 0xB67C, 0xC91E, 0xB23C, 0x963C, 0x921C,
    0xD90E, 0xCB0E, 0xB61C, 0xC906, 0xB20C, 0x960C, 0x9204, 0xD902, 0x997E, 0x8B7E, 0x893E, 0xBB7E,
    0xB93E, 0xE4A0, 0xF928, 0xD940, 0xF650, 0xFD94, 0xCB40, 0xF2D0, 0xEDA0, 0xFB68, 0x8940, 0xE250,
    0xCCA0, 0xF328, 0xB940, 0xEE50, 0xFB94, 0xC5A0, 0xF168, 0x9B40, 0xE6D0, 0xF9B4, 0xDDA0, 0xF768,
    0xFDDA, 0x84A0, 0xE128, 0xC650, 0xF194, 0x9CA0, 0xE728, 0xF9CA, 0xDE50, 0xF794, 0xC2D0, 0x8DA0,
    0xE368, 0xCED0, 0xF3B4, 0xBDA0, 0xEF68, 0xFBDA, 0x8250, 0xC328, 0x8E50, 0xE394, 0xCF28, 0xF3CA,
    0xBE50, 0xEF94, 0xC168, 0x86D0, 0xE1B4, 0xC768, 0xF1DA, 0x9ED0, 0xE7B4, 0xDF68, 0xF7DA, 0x8128,
    0xC194, 0x8728, 0xE1CA, 0xC794, 0x9F28, 0xE7CA, 0x8368, 0xC3B4, 0x8F68, 0xE3DA, 0xCFB4, 0xBF68,
    0xEFDA, 0xE8A0, 0xFA28, 0xD340, 0xF4D0, 0xFD34, 0xEBA0, 0xFAE8, 0x9140, 0xE450, 0xF914, 0xD8A0,
    0xF628, 0xFD8A, 0xC9A0, 0xF268, 0xB340, 0xECD0, 0xFB34, 0x9740, 0xE5D0, 0xF974, 0xDBA0, 0xF6E8,
    0xFDBA, 0x88A0, 0xE228, 0xCC50, 0xF314, 0xB8A0, 0xEE28, 0xFB8A, 0xC4D0, 0xF134, 0x99A0, 0xE668,
    0xF99A, 0xDCD0, 0xF734, 0x8BA0, 0xE2E8, 0xCDD0, 0xF374, 0xBBA0, 0xEEE8, 0xFBBA, 0x8450, 0xE114,
    0xC628, 0xF18A, 0x9C50, 0xE714, 0xDE28, 0xF78A, 0xC268, 0x8CD0, 0xE334, 0xCE68, 0xF39A, 0xBCD0,
    0xEF34, 0x85D0, 0xE174, 0xC6E8, 0xF1BA, 0x9DD0, 0xE774, 0xDEE8, 0xF7BA, 0x8228, 0xC314, 0x8E28,
    0xE38A, 0xCF14, 0xC134, 0x8668, 0xE19A, 0xC734, 0x9E68, 0xE79A, 0xDF34, 0x82E8, 0xC374, 0x8EE8,
    0xE3BA, 0xCF74, 0xBEE8, 0xEFBA, 0x8114, 0xC18A, 0x8714, 0xC78A, 0x8334, 0xC39A, 0x8F34, 0xCF9A,
    0x8174, 0xC1BA, 0x8774, 0xC7BA, 0x9F74, 0xDFBA, 0xA140, 0xE850, 0xFA14, 0xD1A0, 0xF468, 0xFD1A,
    0xA740, 0xE9D0, 0xFA74, 0xD7A0, 0xF5E8, 0xFD7A, 0x90A0, 0xE428, 0xF90A, 0xD850, 0xF614, 0xC8D0,
    0xF234, 0xB1A0, 0xEC68, 0xFB1A, 0x93A0, 0xE4E8, 0xF93A, 0xD9D0, 0xF674, 0xCBD0, 0xF2F4, 0xB7A0,
    0xEDE8, 0xFB7A, 0x8850, 0xE214, 0xCC28, 0xF30A, 0xB850, 0xEE14, 0xC468, 0xF11A, 0x98D0, 0xE634,
    0xDC68, 0xF71A, 0x89D0, 0xE274, 0xCCE8, 0xF33A, 0xB9D0, 0xEE74, 0xC5E8, 0xF17A, 0x9BD0, 0xE6F4,
    0xDDE8, 0xF77A, 0x8428, 0xE10A, 0xC614, 0x9C28, 0xE70A, 0xC234, 0x8C68, 0xE31A, 0xCE34, 0xBC68,
    0xEF1A, 0x84E8, 0xE13A, 0xC674, 0x9CE8, 0xE73A, 0xDE74, 0xC2F4, 0x8DE8, 0xE37A, 0xCEF4, 0xBDE8,
    0xEF7A, 0x8214, 0xC30A, 0x8E14, 0xC11A, 0x8634, 0xC71A, 0x9E34, 0x8274, 0xC33A, 0x8E74, 0xCF3A,
    0xBE74, 0xC17A, 0x86F4, 0xC77A, 0x9EF4, 0xDF7A, 0x810A, 0x870A, 0x831A, 0x8F1A, 0x813A, 0x873A,
    0x9F3A, 0x837A, 0x8F7A, 0xBF7A, 0xA0A0, 0xE828, 0xFA0A, 0xD0D0, 0xF434, 0xA3A0, 0xE8E8, 0xFA3A,
    0xD3D0, 0xF4F4, 0xAFA0, 0xEBE8, 0xFAFA, 0x9050, 0xE414, 0xD828, 0xF60A, 0xC868, 0xF21A, 0xB0D0,
    0xEC34, 0x91D0, 0xE474, 0xD8E8, 0xF63A, 0xC9E8, 0xF27A, 0xB3D0, 0xECF4, 0x97D0, 0xE5F4, 0xDBE8,
    0xF6FA, 0x8828, 0xE20A, 0xCC14, 0xC434, 0x9868, 0xE61A, 0xDC34, 0x88E8, 0xE23A, 0xCC74, 0xB8E8,
    0xEE3A, 0xC4F4, 0x99E8, 0xE67A, 0xDCF4, 0x8BE8, 0xE2FA, 0xCDF4, 0xBBE8, 0xEEFA, 0x8414, 0xC60A,
    0xC21A, 0x8C34, 0xCE1A, 0x8474, 0xC63A, 0x9C74, 0xDE3A, 0xC27A, 0x8CF4, 0xCE7A, 0xBCF4, 0x85F4,
    0xC6FA, 0x9DF4, 0xDEFA, 0x820A, 0x861A, 0x823A, 0x8E3A, 0x867A, 0x9E7A, 0x82FA, 0x8EFA, 0xBEFA,
    0xA050, 0xE814, 0xD068, 0xF41A, 0xA1D0, 0xE874, 0xD1E8, 0xF47A, 0xA7D0, 0xE9F4, 0xD7E8, 0xF5FA,
    0x9028, 0xE40A, 0xC834, 0xB068, 0xEC1A, 0x90E8, 0xE43A, 0xD874, 0xC8F4, 0xB1E8, 0xEC7A, 0x93E8,
    0xE4FA, 0xD9F4, 0xCBF4, 0xB7E8, 0xEDFA, 0x8814, 0xC41A, 0x9834, 0x8874, 0xCC3A, 0xB874, 0xC47A,
    0x98F4, 0xDC7A, 0x89F4, 0xCCFA, 0xB9F4, 0xC5FA, 0x9BF4, 0xDDFA, 0x840A, 0x8C1A, 0x843A, 0x9C3A,
    0x8C7A, 0xBC7A, 0x84FA, 0x9CFA, 0x8DFA, 0xBDFA, 0xEA40, 0xFA90, 0xED60, 0xFB58, 0xE520, 0xF948,
    0xDA40, 0xF690, 0xFDA4, 0x9AC0, 0xE6B0, 0xF9AC, 0xDD60, 0xF758, 0xFDD6, 0x8A40, 0xE290, 0xCD20,
    0xF348, 0xBA40, 0xEE90, 0xFBA4, 0x8D60, 0xE358, 0xCEB0, 0xF3AC, 0xBD60, 0xEF58, 0xFBD6, 0x8520,
    0xE148, 0xC690, 0xF1A4, 0x9D20, 0xE748, 0xF9D2, 0xDE90, 0xF7A4, 0x86B0, 0xE1AC, 0xC758, 0xF1D6,
    0x9EB0, 0xE7AC, 0xDF58, 0xF7D6, 0x8290, 0xC348, 0x8E90, 0xE3A4, 0xCF48, 0xF3D2, 0xBE90, 0xEFA4,
    0x8358, 0xC3AC, 0x8F58, 0xE3D6, 0xCFAC, 0xBF58, 0xEFD6, 0x8148, 0xC1A4, 0x8748, 0xE1D2, 0xC7A4,
    0x9F48, 0xE7D2, 0xDFA4, 0xD2C0, 0xF4B0, 0xFD2C, 0xEB60, 0xFAD8, 0xE920, 0xFA48, 0xD640, 0xF590,
    0xFD64, 0xC960, 0xF258, 0xB2C0, 0xECB0, 0xFB2C, 0x96C0, 0xE5B0, 0xF96C, 0x9240, 0xE490, 0xF924,
    0xD920, 0xF648, 0xFD92, 0xCB20, 0xF2C8, 0xB640, 0xED90, 0xFB64, 0xC4B0, 0xF12C, 0x9960, 0xE658,
    0xF996, 0xDCB0, 0xF72C, 0x8B60, 0xE2D8, 0x8920, 0xE248, 0xBB60, 0xCC90, 0xF324, 0xB920, 0xEE48,
    0xFB92, 0xC590, 0xF164, 0x9B20, 0xE6C8, 0xF9B2, 0xDD90, 0xF764, 0xC258, 0x8CB0, 0xE32C, 0xCE58,
    0xF396, 0xBCB0, 0xEF2C, 0x85B0, 0xE16C, 0x8490, 0xE124, 0x9DB0, 0xC648, 0xF192, 0x9C90, 0xE724,
    0xDE48, 0xF792, 0xC2C8, 0x8D90, 0xE364, 0xCEC8, 0xF3B2, 0xBD90, 0xEF64, 0xC12C, 0x8658, 0xE196,
    0xC72C, 0x9E58, 0xE796, 0xDF2C, 0x82D8, 0x8248, 0x8ED8, 0xC324, 0x8E48, 0xE392, 0xBED8, 0xCF24,
    0xBE48, 0xEF92, 0xC164, 0x86C8, 0xE1B2, 0xC764, 0x9EC8, 0xE7B2, 0xDF64, 0x832C, 0xC396, 0x8F2C,
    0xCF96, 0x816C, 0x8124, 0x876C, 0xC192, 0x8724, 0x9F6C, 0xC792, 0x9F24, 0x8364, 0xC3B2, 0x8F64,
    0xCFB2, 0xBF64, 0xD160, 0xF458, 0xFD16, 0xA6C0, 0xE9B0, 0xFA6C, 0xA240, 0xE890, 0xFA24, 0xD760,
    0xF5D8, 0xFD76, 0xD320, 0xF4C8, 0xFD32, 0xAE40, 0xEB90, 0xFAE4, 0xC8B0, 0xF22C, 0xB160, 0xEC58,
    0xFB16, 0x9360, 0xE4D8, 0xF936, 0x9120, 0xE448, 0xF912, 0xD890, 0xF624, 0xCBB0, 0xF2EC, 0xB760,
    0xC990, 0xF264, 0xB320, 0xECC8, 0xFB32, 0x9720, 0xE5C8, 0xF972, 0xDB90, 0xF6E4, 0xC458, 0xF116,
    0x98B0, 0xE62C, 0xDC58, 0xF716, 0x89B0, 0xE26C, 0x8890, 0xE224, 0xB9B0, 0xCC48, 0xF312, 0xB890,
    0xEE24, 0xC5D8, 0xF176, 0x9BB0, 0xC4C8, 0xF132, 0x9990, 0xE664, 0xDCC8, 0xF732, 0x8B90, 0xE2E4,
    0xCDC8, 0xF372, 0xBB90, 0xEEE4, 0xC22C, 0x8C58, 0xE316, 0xCE2C, 0xBC58, 0xEF16, 0x84D8, 0xE136,
    0x8448, 0xE112, 0x9CD8, 0xC624, 0x9C48, 0xE712, 0xDE24, 0xC2EC, 0x8DD8, 0xC264, 0x8CC8, 0xE332,
    0xBDD8, 0xCE64, 0xBCC8, 0xEF32, 0x85C8, 0xE172, 0xC6E4, 0x9DC8, 0xE772, 0xDEE4, 0xC116, 0x862C,
    0xC716, 0x9E2C, 0x826C, 0x8224, 0x8E6C, 0xC312, 0x8E24, 0xBE6C, 0xCF12, 0xC176, 0x86EC, 0xC132,
    0x8664, 0x9EEC, 0xC732, 0x9E64, 0xDF32, 0x82E4, 0xC372, 0x8EE4, 0xCF72, 0xBEE4, 0x8316, 0x8F16,
    0x8136, 0x8112, 0x8736, 0x8712, 0x9F36, 0x8376, 0x8332, 0x8F76, 0x8F32, 0xBF76, 0x8172, 0x8772,
    0x9F72, 0xD0B0, 0xF42C, 0xA360, 0xE8D8, 0xFA36, 0xA120, 0xE848, 0xFA12, 0xD3B0, 0xF4EC, 0xD190,
    0xF464, 0xAF60, 0xEBD8, 0xFAF6, 0xA720, 0xE9C8, 0xFA72, 0xD790, 0xF5E4, 0xC858, 0xF216, 0xB0B0,
    0xEC2C, 0x91B0, 0xE46C, 0x9090, 0xE424, 0xD848, 0xF612, 0xC9D8, 0xF276, 0xB3B0, 0xC8C8, 0xF232,
    0xB190, 0xEC64, 0x97B0, 0xE5EC, 0x9390, 0xE4E4, 0xD9C8, 0xF672, 0xCBC8, 0xF2F2, 0xB790, 0xEDE4,
    0xC42C, 0x9858, 0xE616, 0xDC2C, 0x88D8, 0xE236, 0x8848, 0xE212, 0xB8D8, 0xCC24, 0xB848, 0xEE12,
    0xC4EC, 0x99D8, 0xC464, 0x98C8, 0xE632, 0xDC64, 0x8BD8, 0xE2F6, 0x89C8, 0xE272, 0xBBD8, 0xCCE4,
    0xB9C8, 0xEE72, 0xC5E4, 0x9BC8, 0xE6F2, 0xDDE4, 0xC216, 0x8C2C, 0xCE16, 0x846C, 0x8424, 0x9C6C,
    0xC612, 0x9C24, 0xC276, 0x8CEC, 0xC232, 0x8C64, 0xBCEC, 0xCE32, 0xBC64, 0x85EC, 0x84E4, 0x9DEC,
    0xC672, 0x9CE4, 0xDE72, 0xC2F2, 0x8DE4, 0xCEF2, 0xBDE4, 0x8616, 0x8236, 0x8212, 0x8E36, 0x8E12,
    0x8676, 0x8632, 0x9E76, 0x9E32, 0x82F6, 0x8272, 0x8EF6, 0x8E72, 0xBEF6, 0xBE72, 0x86F2, 0x9EF2,
    0xD058, 0xF416, 0xA1B0, 0xE86C, 0xA090, 0xE824, 0xD1D8, 0xF476, 0xD0C8, 0xF432, 0xA7B0, 0xE9EC,
    0xA390, 0xE8E4, 0xD7D8, 0xF5F6, 0xD3C8, 0xF4F2, 0xAF90, 0xEBE4, 0xC82C, 0xB058, 0xEC16, 0x90D8,
    0xE436, 0x9048, 0xE412, 0xD824, 0xC8EC, 0xB1D8, 0xC864, 0xB0C8, 0xEC32, 0x93D8, 0xE4F6, 0x91C8,
    0xE472, 0xD8E4, 0xCBEC, 0xB7D8, 0xC9E4, 0xB3C8, 0xECF2, 0x97C8, 0xE5F2, 0xDBE4, 0xC416, 0x982C,
    0x886C, 0x8824, 0xB86C, 0xCC12, 0xC476, 0x98EC, 0xC432, 0x9864, 0xDC32, 0x89EC, 0x88E4, 0xB9EC,
    0xCC72, 0xB8E4, 0xC5F6, 0x9BEC, 0xC4F2, 0x99E4, 0xDCF2, 0x8BE4, 0xCDF2, 0xBBE4, 0x8C16, 0x8436,
    0x8412, 0x9C36, 0x8C76, 0x8C32, 0xBC76, 0x84F6, 0x8472, 0x9CF6, 0x9C72, 0x8DF6, 0x8CF2, 0xBDF6,
    0xBCF2, 0x85F2, 0x9DF2, 0xD02C, 0xA0D8, 0xE836, 0xA048, 0xE812, 0xD0EC, 0xD064, 0xA3D8, 0xE8F6,
    0xA1C8, 0xE872, 0xD3EC, 0xD1E4, 0xAFD8, 0xEBF6, 0xA7C8, 0xE9F2, 0xC816, 0x906C, 0x9024, 0xC876,
    0xB0EC, 0xC832, 0xB064, 0x91EC, 0x90E4, 0xD872, 0xC9F6, 0xB3EC, 0xC8F2, 0xB1E4, 0x97EC, 0x93E4,
    0xD9F2, 0x8836, 0x8812, 0x9876, 0x9832, 0x88F6, 0x8872, 0xB8F6, 0xB872, 0x99F6, 0x98F2, 0x8BF6,
    0x89F2, 0xBBF6, 0xB9F2, 0xD4C0, 0xF530, 0xFD4C, 0xEA20, 0xFA88, 0xDAE0, 0xF6B8, 0xFDAE, 0xCA60,
    0xF298, 0xB4C0, 0xED30, 0xFB4C, 0x9440, 0xE510, 0xF944, 0xDA20, 0xF688, 0xFDA2, 0xCD70, 0xF35C,
    0xBAE0, 0xEEB8, 0xFBAE, 0xC530, 0xF14C, 0x9A60, 0xE698, 0xF9A6, 0xDD30, 0xF74C, 0x8A20, 0xE288,
    0xCD10, 0xF344, 0xBA20, 0xEE88, 0xFBA2, 0xC6B8, 0xF1AE, 0x9D70, 0xE75C, 0xDEB8, 0xF7AE, 0xC298,
    0x8D30, 0xE34C, 0xCE98, 0xF3A6, 0xBD30, 0xEF4C, 0x8510, 0xE144, 0xC688, 0xF1A2, 0x9D10, 0xE744,
    0xDE88, 0xF7A2, 0xC35C, 0x8EB8, 0xE3AE, 0xCF5C, 0xBEB8, 0xEFAE, 0xC14C, 0x8698, 0xE1A6, 0xC74C,
    0x9E98, 0xE7A6, 0xDF4C, 0x8288, 0xC344, 0x8E88, 0xE3A2, 0xCF44, 0xBE88, 0xEFA2, 0xC1AE, 0x875C,
    0xC7AE, 0x9F5C, 0xDFAE, 0x834C, 0xC3A6, 0x8F4C, 0xCFA6, 0xBF4C, 0x8144, 0xC1A2, 0x8744, 0xC7A2,
    0x9F44, 0xDFA2, 0xE970, 0xFA5C, 0xD6E0, 0xF5B8, 0xFD6E, 0xD260, 0xF498, 0xFD26, 0xACC0, 0xEB30,
    0xFACC, 0xA440, 0xE910, 0xFA44, 0xD620, 0xF588, 0xFD62, 0x92E0, 0xE4B8, 0xF92E, 0xD970, 0xF65C,
    0xCB70, 0xF2DC, 0xB6E0, 0xC930, 0xF24C, 0xB260, 0xEC98, 0xFB26, 0x9660, 0xE598, 0xF966, 0x9220,
    0xE488, 0xF922, 0xD910, 0xF644, 0xCB10, 0xF2C4, 0xB620, 0xED88, 0xFB62, 0x8970, 0xE25C, 0xCCB8,
    0xF32E, 0xB970, 0xEE5C, 0xC5B8, 0xF16E, 0x9B70, 0xC498, 0xF126, 0x9930, 0xE64C, 0xDC98, 0xF726,
    0x8B30, 0xE2CC, 0x8910, 0xE244, 0xBB30, 0xCC88, 0xF322, 0xB910, 0xEE44, 0xC588, 0xF162, 0x9B10,
    0xE6C4, 0xDD88, 0xF762, 0x84B8, 0xE12E, 0xC65C, 0x9CB8, 0xE72E, 0xDE5C, 0xC2DC, 0x8DB8, 0xC24C,
    0x8C98, 0xE326, 0xBDB8, 0xCE4C, 0xBC98, 0xEF26, 0x8598, 0xE166, 0x8488, 0xE122, 0x9D98, 0xC644,
    0x9C88, 0xE722, 0xDE44, 0xC2C4, 0x8D88, 0xE362, 0xCEC4, 0xBD88, 0xEF62, 0x825C, 0xC32E, 0x8E5C,
    0xCF2E,
];

const C49_ODD: [u16; 2401] = [
    0xC940, 0xF250, 0xECA0, 0xFB28, 0xE5A0, 0xF968, 0xDB40, 0xF6D0, 0xFDB4, 0xC4A0, 0xF128, 0x9940,
    0xE650, 0xF994, 0xDCA0, 0xF728, 0xFDCA, 0x8B40, 0xE2D0, 0xCDA0, 0xF368, 0xBB40, 0xEED0, 0xFBB4,
    0xC250, 0x8CA0, 0xE328, 0xCE50, 0xF394, 0xBCA0, 0xEF28, 0xFBCA, 0x85A0, 0xE168, 0xC6D0, 0xF1B4,
    0x9DA0, 0xE768, 0xF9DA, 0xDED0, 0xF7B4, 0xC128, 0x8650, 0xE194, 0xC728, 0xF1CA, 0x9E50, 0xE794,
    0xDF28, 0xF7CA, 0x82D0, 0xC368, 0x8ED0, 0xE3B4, 0xCF68, 0xF3DA, 0xBED0, 0xEFB4, 0x8328, 0xC394,
    0x8F28, 0xE3CA, 0xCF94, 0x8168, 0xC1B4, 0x8768, 0xE1DA, 0xC7B4, 0x9F68, 0xE7DA, 0xDFB4, 0xD140,
    0xF450, 0xFD14, 0xE9A0, 0xFA68, 0xD740, 0xF5D0, 0xFD74, 0xC8A0, 0xF228, 0xB140, 0xEC50, 0xFB14,
    0x9340, 0xE4D0, 0xF934, 0xD9A0, 0xF668, 0xFD9A, 0xCBA0, 0xF2E8, 0xB740, 0xEDD0, 0xFB74, 0xC450,
    0xF114, 0x98A0, 0xE628, 0xF98A, 0xDC50, 0xF714, 0x89A0, 0xE268, 0xCCD0, 0xF334, 0xB9A0, 0xEE68,
    0xFB9A, 0xC5D0, 0xF174, 0x9BA0, 0xE6E8, 0xF9BA, 0xDDD0, 0xF774, 0xC228, 0x8C50, 0xE314, 0xCE28,
    0xF38A, 0xBC50, 0xEF14, 0x84D0, 0xE134, 0xC668, 0xF19A, 0x9CD0, 0xE734, 0xDE68, 0xF79A, 0xC2E8,
    0x8DD0, 0xE374, 0xCEE8, 0xF3BA, 0xBDD0, 0xEF74, 0xC114, 0x8628, 0xE18A, 0xC714, 0x9E28, 0xE78A,
    0x8268, 0xC334, 0x8E68, 0xE39A, 0xCF34, 0xBE68, 0xEF9A, 0xC174, 0x86E8, 0xE1BA, 0xC774, 0x9EE8,
    0xE7BA, 0xDF74, 0x8314, 0xC38A, 0x8F14, 0x8134, 0xC19A, 0x8734, 0xC79A, 0x9F34, 0x8374, 0xC3BA,
    0x8F74, 0xCFBA, 0xBF74, 0xD0A0, 0xF428, 0xFD0A, 0xA340, 0xE8D0, 0xFA34, 0xD3A0, 0xF4E8, 0xFD3A,
    0xAF40, 0xEBD0, 0xFAF4, 0xC850, 0xF214, 0xB0A0, 0xEC28, 0xFB0A, 0x91A0, 0xE468, 0xF91A, 0xD8D0,
    0xF634, 0xC9D0, 0xF274, 0xB3A0, 0xECE8, 0xFB3A, 0x97A0, 0xE5E8, 0xF97A, 0xDBD0, 0xF6F4, 0xC428,
    0xF10A, 0x9850, 0xE614, 0xDC28, 0xF70A, 0x88D0, 0xE234, 0xCC68, 0xF31A, 0xB8D0, 0xEE34, 0xC4E8,
    0xF13A, 0x99D0, 0xE674, 0xDCE8, 0xF73A, 0x8BD0, 0xE2F4, 0xCDE8, 0xF37A, 0xBBD0, 0xEEF4, 0xC214,
    0x8C28, 0xE30A, 0xCE14, 0x8468, 0xE11A, 0xC634, 0x9C68, 0xE71A, 0xDE34, 0xC274, 0x8CE8, 0xE33A,
    0xCE74, 0xBCE8, 0xEF3A, 0x85E8, 0xE17A, 0xC6F4, 0x9DE8, 0xE77A, 0xDEF4, 0xC10A, 0x8614, 0xC70A,
    0x8234, 0xC31A, 0x8E34, 0xCF1A, 0xC13A, 0x8674, 0xC73A, 0x9E74, 0xDF3A, 0x82F4, 0xC37A, 0x8EF4,
    0xCF7A, 0xBEF4, 0x830A, 0x811A, 0x871A, 0x833A, 0x8F3A, 0x817A, 0x877A, 0x9F7A, 0xD050, 0xF414,
    0xA1A0, 0xE868, 0xFA1A, 0xD1D0, 0xF474, 0xA7A0, 0xE9E8, 0xFA7A, 0xD7D0, 0xF5F4, 0xC828, 0xF20A,
    0xB050, 0xEC14, 0x90D0, 0xE434, 0xD868, 0xF61A, 0xC8E8, 0xF23A, 0xB1D0, 0xEC74, 0x93D0, 0xE4F4,
    0xD9E8, 0xF67A, 0xCBE8, 0xF2FA, 0xB7D0, 0xEDF4, 0xC414, 0x9828, 0xE60A, 0x8868, 0xE21A, 0xCC34,
    0xB868, 0xEE1A, 0xC474, 0x98E8, 0xE63A, 0xDC74, 0x89E8, 0xE27A, 0xCCF4, 0xB9E8, 0xEE7A, 0xC5F4,
    0x9BE8, 0xE6FA, 0xDDF4, 0xC20A, 0x8C14, 0x8434, 0xC61A, 0x9C34, 0xC23A, 0x8C74, 0xCE3A, 0xBC74,
    0x84F4, 0xC67A, 0x9CF4, 0xDE7A, 0xC2FA, 0x8DF4, 0xCEFA, 0xBDF4, 0x860A, 0x821A, 0x8E1A, 0x863A,
    0x9E3A, 0x827A, 0x8E7A, 0xBE7A, 0x86FA, 0x9EFA, 0xD028, 0xF40A, 0xA0D0, 0xE834, 0xD0E8, 0xF43A,
    0xA3D0, 0xE8F4, 0xD3E8, 0xF4FA, 0xAFD0, 0xEBF4, 0xC814, 0x9068, 0xE41A, 0xD834, 0xC874, 0xB0E8,
    0xEC3A, 0x91E8, 0xE47A, 0xD8F4, 0xC9F4, 0xB3E8, 0xECFA, 0x97E8, 0xE5FA, 0xDBF4, 0xC40A, 0x8834,
    0xCC1A, 0xC43A, 0x9874, 0xDC3A, 0x88F4, 0xCC7A, 0xB8F4, 0xC4FA, 0x99F4, 0xDCFA, 0x8BF4, 0xCDFA,
    0xBBF4, 0x841A, 0x8C3A, 0x847A, 0x9C7A, 0x8CFA, 0xBCFA, 0x85FA, 0x9DFA, 0xF520, 0xFD48, 0xDAC0,
    0xF6B0, 0xFDAC, 0xCA40, 0xF290, 0xED20, 0xFB48, 0xCD60, 0xF358, 0xBAC0, 0xEEB0, 0xFBAC, 0xC520,
    0xF148, 0x9A40, 0xE690, 0xF9A4, 0xDD20, 0xF748, 0xFDD2, 0xC6B0, 0xF1AC, 0x9D60, 0xE758, 0xF9D6,
    0xDEB0, 0xF7AC, 0xC290, 0x8D20, 0xE348, 0xCE90, 0xF3A4, 0xBD20, 0xEF48, 0xFBD2, 0xC358, 0x8EB0,
    0xE3AC, 0xCF58, 0xF3D6, 0xBEB0, 0xEFAC, 0xC148, 0x8690, 0xE1A4, 0xC748, 0xF1D2, 0x9E90, 0xE7A4,
    0xDF48, 0xF7D2, 0xC1AC, 0x8758, 0xE1D6, 0xC7AC, 0x9F58, 0xE7D6, 0xDFAC, 0x8348, 0xC3A4, 0x8F48,
    0xE3D2, 0xCFA4, 0xBF48, 0xEFD2, 0xE960, 0xFA58, 0xD6C0, 0xF5B0, 0xFD6C, 0xD240, 0xF490, 0xFD24,
    0xEB20, 0xFAC8, 0x92C0, 0xE4B0, 0xF92C, 0xD960, 0xF658, 0xFD96, 0xCB60, 0xF2D8, 0xB6C0, 0xC920,
    0xF248, 0xB240, 0xEC90, 0xFB24, 0x9640, 0xE590, 0xF964, 0xDB20, 0xF6C8, 0xFDB2, 0x8960, 0xE258,
    0xCCB0, 0xF32C, 0xB960, 0xEE58, 0xFB96, 0xC5B0, 0xF16C, 0x9B60, 0xC490, 0xF124, 0x9920, 0xE648,
    0xF992, 0xDC90, 0xF724, 0x8B20, 0xE2C8, 0xCD90, 0xF364, 0xBB20, 0xEEC8, 0xFBB2, 0x84B0, 0xE12C,
    0xC658, 0xF196, 0x9CB0, 0xE72C, 0xDE58, 0xF796, 0xC2D8, 0x8DB0, 0xC248, 0x8C90, 0xE324, 0xBDB0,
    0xCE48, 0xF392, 0xBC90, 0xEF24, 0x8590, 0xE164, 0xC6C8, 0xF1B2, 0x9D90, 0xE764, 0xDEC8, 0xF7B2,
    0x8258, 0xC32C, 0x8E58, 0xE396, 0xCF2C, 0xBE58, 0xEF96, 0xC16C, 0x86D8, 0xC124, 0x8648, 0xE192,
    0x9ED8, 0xC724, 0x9E48, 0xE792, 0xDF24, 0x82C8, 0xC364, 0x8EC8, 0xE3B2, 0xCF64, 0xBEC8, 0xEFB2,
    0x812C, 0xC196, 0x872C, 0xC796, 0x9F2C, 0x836C, 0x8324, 0x8F6C, 0xC392, 0x8F24, 0xBF6C, 0xCF92,
    0x8164, 0xC1B2, 0x8764, 0xC7B2, 0x9F64, 0xDFB2, 0xA2C0, 0xE8B0, 0xFA2C, 0xD360, 0xF4D8, 0xFD36,
    0xD120, 0xF448, 0xFD12, 0xAEC0, 0xEBB0, 0xFAEC, 0xA640, 0xE990, 0xFA64, 0xD720, 0xF5C8, 0xFD72,
    0x9160, 0xE458, 0xF916, 0xD8B0, 0xF62C, 0xC9B0, 0xF26C, 0xB360, 0xC890, 0xF224, 0xB120, 0xEC48,
    0xFB12, 0x9760, 0xE5D8, 0xF976, 0x9320, 0xE4C8, 0xF932, 0xD990, 0xF664, 0xCB90, 0xF2E4, 0xB720,
    0xEDC8, 0xFB72, 0x88B0, 0xE22C, 0xCC58, 0xF316, 0xB8B0, 0xEE2C, 0xC4D8, 0xF136, 0x99B0, 0xC448,
    0xF112, 0x9890, 0xE624, 0xDC48, 0xF712, 0x8BB0, 0xE2EC, 0x8990, 0xE264, 0xBBB0, 0xCCC8, 0xF332,
    0xB990, 0xEE64, 0xC5C8, 0xF172, 0x9B90, 0xE6E4, 0xDDC8, 0xF772, 0x8458, 0xE116, 0xC62C, 0x9C58,
    0xE716, 0xDE2C, 0xC26C, 0x8CD8, 0xC224, 0x8C48, 0xE312, 0xBCD8, 0xCE24, 0xBC48, 0xEF12, 0x85D8,
    0xE176, 0x84C8, 0xE132, 0x9DD8, 0xC664, 0x9CC8, 0xE732, 0xDE64, 0xC2E4, 0x8DC8, 0xE372, 0xCEE4,
    0xBDC8, 0xEF72, 0x822C, 0xC316, 0x8E2C, 0xCF16, 0xC136, 0x866C, 0xC112, 0x8624, 0x9E6C, 0xC712,
    0x9E24, 0x82EC, 0x8264, 0x8EEC, 0xC332, 0x8E64, 0xBEEC, 0xCF32, 0xBE64, 0xC172, 0x86E4, 0xC772,
    0x9EE4, 0xDF72, 0x8116, 0x8716, 0x8336, 0x8312, 0x8F36, 0x8F12, 0x8176, 0x8132, 0x8776, 0x8732,
    0x9F76, 0x9F32, 0x8372, 0x8F72, 0xBF72, 0xA160, 0xE858, 0xFA16, 0xD1B0, 0xF46C, 0xD090, 0xF424,
    0xA760, 0xE9D8, 0xFA76, 0xA320, 0xE8C8, 0xFA32, 0xD7B0, 0xF5EC, 0xD390, 0xF4E4, 0xAF20, 0xEBC8,
    0xFAF2, 0x90B0, 0xE42C, 0xD858, 0xF616, 0xC8D8, 0xF236, 0xB1B0, 0xC848, 0xF212, 0xB090, 0xEC24,
    0x93B0, 0xE4EC, 0x9190, 0xE464, 0xD8C8, 0xF632, 0xCBD8, 0xF2F6, 0xB7B0, 0xC9C8, 0xF272, 0xB390,
    0xECE4, 0x9790, 0xE5E4, 0xDBC8, 0xF6F2, 0x8858, 0xE216, 0xCC2C, 0xB858, 0xEE16, 0xC46C, 0x98D8,
    0xC424, 0x9848, 0xE612, 0xDC24, 0x89D8, 0xE276, 0x88C8, 0xE232, 0xB9D8, 0xCC64, 0xB8C8, 0xEE32,
    0xC5EC, 0x9BD8, 0xC4E4, 0x99C8, 0xE672, 0xDCE4, 0x8BC8, 0xE2F2, 0xCDE4, 0xBBC8, 0xEEF2, 0x842C,
    0xC616, 0x9C2C, 0xC236, 0x8C6C, 0xC212, 0x8C24, 0xBC6C, 0xCE12, 0x84EC, 0x8464, 0x9CEC, 0xC632,
    0x9C64, 0xDE32, 0xC2F6, 0x8DEC, 0xC272, 0x8CE4, 0xBDEC, 0xCE72, 0xBCE4, 0x85E4, 0xC6F2, 0x9DE4,
    0xDEF2, 0x8216, 0x8E16, 0x8636, 0x8612, 0x9E36, 0x8276, 0x8232, 0x8E76, 0x8E32, 0xBE76, 0x86F6,
    0x8672, 0x9EF6, 0x9E72, 0x82F2, 0x8EF2, 0xBEF2, 0xA0B0, 0xE82C, 0xD0D8, 0xF436, 0xD048, 0xF412,
    0xA3B0, 0xE8EC, 0xA190, 0xE864, 0xD3D8, 0xF4F6, 0xD1C8, 0xF472, 0xAFB0, 0xEBEC, 0xA790, 0xE9E4,
    0xD7C8, 0xF5F2, 0x9058, 0xE416, 0xD82C, 0xC86C, 0xB0D8, 0xC824, 0xB048, 0xEC12, 0x91D8, 0xE476,
    0x90C8, 0xE432, 0xD864, 0xC9EC, 0xB3D8, 0xC8E4, 0xB1C8, 0xEC72, 0x97D8, 0xE5F6, 0x93C8, 0xE4F2,
    0xD9E4, 0xCBE4, 0xB7C8, 0xEDF2, 0x882C, 0xCC16, 0xC436, 0x986C, 0xC412, 0x9824, 0x88EC, 0x8864,
    0xB8EC, 0xCC32, 0xB864, 0xC4F6, 0x99EC, 0xC472, 0x98E4, 0xDC72, 0x8BEC, 0x89E4, 0xBBEC, 0xCCF2,
    0xB9E4, 0xC5F2, 0x9BE4, 0xDDF2, 0x8416, 0x8C36, 0x8C12, 0x8476, 0x8432, 0x9C76, 0x9C32, 0x8CF6,
    0x8C72, 0xBCF6, 0xBC72, 0x85F6, 0x84F2, 0x9DF6, 0x9CF2, 0x8DF2, 0xBDF2, 0xA058, 0xE816, 0xD06C,
    0xD024, 0xA1D8, 0xE876, 0xA0C8, 0xE832, 0xD1EC, 0xD0E4, 0xA7D8, 0xE9F6, 0xA3C8, 0xE8F2, 0xD7EC,
    0xD3E4, 0x902C, 0xC836, 0xB06C, 0xC812, 0x90EC, 0x9064, 0xD832, 0xC8F6, 0xB1EC, 0xC872, 0xB0E4,
    0x93EC, 0x91E4, 0xD8F2, 0xCBF6, 0xB7EC, 0xC9F2, 0xB3E4, 0x8816, 0x9836, 0x8876, 0x8832, 0xB876,
    0x98F6, 0x9872, 0x89F6, 0x88F2, 0xB9F6, 0xB8F2, 0x9BF6, 0x99F2, 0xEA60, 0xFA98, 0xD440, 0xF510,
    0xFD44, 0xED70, 0xFB5C, 0x94C0, 0xE530, 0xF94C, 0xDA60, 0xF698, 0xFDA6, 0xCA20, 0xF288, 0xB440,
    0xED10, 0xFB44, 0x9AE0, 0xE6B8, 0xF9AE, 0xDD70, 0xF75C, 0x8A60, 0xE298, 0xCD30, 0xF34C, 0xBA60,
    0xEE98, 0xFBA6, 0xC510, 0xF144, 0x9A20, 0xE688, 0xF9A2, 0xDD10, 0xF744, 0x8D70, 0xE35C, 0xCEB8,
    0xF3AE, 0xBD70, 0xEF5C, 0x8530, 0xE14C, 0xC698, 0xF1A6, 0x9D30, 0xE74C, 0xDE98, 0xF7A6, 0xC288,
    0x8D10, 0xE344, 0xCE88, 0xF3A2, 0xBD10, 0xEF44, 0x86B8, 0xE1AE, 0xC75C, 0x9EB8, 0xE7AE, 0xDF5C,
    0x8298, 0xC34C, 0x8E98, 0xE3A6, 0xCF4C, 0xBE98, 0xEFA6, 0xC144, 0x8688, 0xE1A2, 0xC744, 0x9E88,
    0xE7A2, 0xDF44, 0x835C, 0xC3AE, 0x8F5C, 0xCFAE, 0xBF5C, 0x814C, 0xC1A6, 0x874C, 0xC7A6, 0x9F4C,
    0xDFA6, 0x8344, 0xC3A2, 0x8F44, 0xCFA2, 0xBF44, 0xD2E0, 0xF4B8, 0xFD2E, 0xADC0, 0xEB70, 0xFADC,
    0xA4C0, 0xE930, 0xFA4C, 0xD660, 0xF598, 0xFD66, 0xD220, 0xF488, 0xFD22, 0xAC40, 0xEB10, 0xFAC4,
    0xC970, 0xF25C, 0xB2E0, 0xECB8, 0xFB2E, 0x96E0, 0xE5B8, 0xF96E, 0x9260, 0xE498, 0xF926, 0xD930,
    0xF64C, 0xCB30, 0xF2CC, 0xB660, 0xC910, 0xF244, 0xB220, 0xEC88, 0xFB22, 0x9620, 0xE588, 0xF962,
    0xDB10, 0xF6C4, 0xC4B8, 0xF12E, 0x9970, 0xE65C, 0xDCB8, 0xF72E, 0x8B70, 0xE2DC, 0x8930, 0xE24C,
    0xBB70, 0xCC98, 0xF326, 0xB930, 0xEE4C, 0xC598, 0xF166, 0x9B30, 0xC488, 0xF122, 0x9910, 0xE644,
    0xDC88, 0xF722, 0x8B10, 0xE2C4, 0xCD88, 0xF362, 0xBB10, 0xEEC4, 0xC25C, 0x8CB8, 0xE32E, 0xCE5C,
    0xBCB8, 0xEF2E, 0x85B8, 0xE16E, 0x8498, 0xE126, 0x9DB8, 0xC64C, 0x9C98, 0xE726, 0xDE4C, 0xC2CC,
    0x8D98, 0xC244, 0x8C88, 0xE322, 0xBD98, 0xCE44, 0xBC88, 0xEF22, 0x8588, 0xE162, 0xC6C4, 0x9D88,
    0xE762, 0xDEC4, 0xC12E, 0x865C, 0xC72E, 0x9E5C, 0xDF2E, 0x82DC, 0x824C, 0x8EDC, 0xC326, 0x8E4C,
    0xBEDC, 0xCF26, 0xBE4C, 0xC166, 0x86CC, 0xC122, 0x8644, 0x9ECC, 0xC722, 0x9E44, 0xDF22, 0x82C4,
    0xC362, 0x8EC4, 0xCF62, 0xBEC4, 0x832E, 0x8F2E, 0x816E, 0x8126, 0x876E, 0x8726, 0x9F6E, 0x9F26,
    0x8366, 0x8322, 0x8F66, 0x8F22, 0xBF66, 0x8162, 0x8762, 0x9F62, 0xD170, 0xF45C, 0xA6E0, 0xE9B8,
    0xFA6E, 0xA260, 0xE898, 0xFA26, 0xD770, 0xF5DC, 0xD330, 0xF4CC, 0xD110, 0xF444, 0xAE60, 0xEB98,
    0xFAE6, 0xA620, 0xE988, 0xFA62, 0xD710, 0xF5C4, 0xC8B8, 0xF22E, 0xB170, 0xEC5C, 0x9370, 0xE4DC,
    0x9130, 0xE44C, 0xD898, 0xF626, 0xCBB8, 0xF2EE, 0xB770, 0xC998, 0xF266, 0xB330, 0xC888, 0xF222,
    0xB110, 0xEC44, 0x9730, 0xE5CC, 0x9310, 0xE4C4, 0xD988, 0xF662, 0xCB88, 0xF2E2, 0xB710, 0xEDC4,
    0xC45C, 0x98B8, 0xE62E, 0xDC5C, 0x89B8, 0xE26E, 0x8898, 0xE226, 0xB9B8, 0xCC4C, 0xB898, 0xEE26,
    0xC5DC, 0x9BB8, 0xC4CC, 0x9998, 0xC444, 0x9888, 0xE622, 0xDC44, 0x8B98, 0xE2E6, 0x8988, 0xE262,
    0xBB98, 0xCCC4, 0xB988, 0xEE62, 0xC5C4, 0x9B88, 0xE6E2, 0xDDC4, 0xC22E, 0x8C5C, 0xCE2E, 0xBC5C,
    0x84DC, 0x844C, 0x9CDC, 0xC626, 0x9C4C, 0xDE26, 0xC2EE, 0x8DDC, 0xC266, 0x8CCC, 0xC222, 0xBDDC,
    0x8C44, 0xBCCC, 0xCE22, 0xBC44, 0x85CC, 0x84C4, 0x9DCC, 0xC662, 0x9CC4, 0xDE62, 0xC2E2, 0x8DC4,
    0xCEE2, 0xBDC4, 0x862E, 0x9E2E, 0x826E, 0x8226, 0x8E6E, 0x8E26, 0xBE6E, 0x86EE, 0x8666, 0x9EEE,
    0x8622, 0x9E66, 0x9E22, 0x82E6, 0x8262, 0x8EE6, 0x8E62, 0xBEE6, 0xBE62, 0x86E2, 0x9EE2, 0xD0B8,
    0xF42E, 0xA370, 0xE8DC, 0xA130, 0xE84C, 0xD3B8, 0xF4EE, 0xD198, 0xF466, 0xD088, 0xF422, 0xAF70,
    0xEBDC, 0xA730, 0xE9CC, 0xA310, 0xE8C4, 0xD798, 0xF5E6, 0xD388, 0xF4E2, 0xAF10, 0xEBC4, 0xC85C,
    0xB0B8, 0xEC2E, 0x91B8, 0xE46E, 0x9098, 0xE426, 0xD84C, 0xC9DC, 0xB3B8, 0xC8CC, 0xB198, 0xC844,
    0xB088, 0xEC22, 0x97B8, 0xE5EE, 0x9398, 0xE4E6, 0x9188, 0xE462, 0xD8C4, 0xCBCC, 0xB798, 0xC9C4,
    0xB388, 0xECE2, 0x9788, 0xE5E2, 0xDBC4, 0xC42E, 0x985C, 0xDC2E, 0x88DC, 0x884C, 0xB8DC, 0xCC26,
    0xB84C, 0xC4EE, 0x99DC, 0xC466, 0x98CC, 0xC422, 0x9844, 0xDC22, 0x8BDC, 0x89CC, 0xBBDC, 0x88C4,
    0xB9CC, 0xCC62, 0xB8C4, 0xC5E6, 0x9BCC, 0xC4E2, 0x99C4, 0xDCE2, 0x8BC4, 0xCDE2, 0xBBC4, 0x8C2E,
    0x846E, 0x8426, 0x9C6E, 0x9C26, 0x8CEE, 0x8C66, 0xBCEE, 0x8C22, 0xBC66, 0x85EE, 0x84E6, 0x9DEE,
    0x8462, 0x9CE6, 0x9C62, 0x8DE6, 0x8CE2, 0xBDE6, 0xBCE2, 0x85E2, 0x9DE2, 0xD05C, 0xA1B8, 0xE86E,
    0xA098, 0xE826, 0xD1DC, 0xD0CC, 0xD044, 0xA7B8, 0xE9EE, 0xA398, 0xE8E6, 0xA188, 0xE862, 0xD7DC,
    0xD3CC, 0xD1C4, 0xAF98, 0xEBE6, 0xA788, 0xE9E2, 0xC82E, 0xB05C, 0x90DC, 0x904C, 0xD826, 0xC8EE,
    0xB1DC, 0xC866, 0xB0CC, 0xC822, 0xB044, 0x93DC, 0x91CC, 0x90C4, 0xD862, 0xCBEE, 0xB7DC, 0xC9E6,
    0xB3CC, 0xC8E2, 0xB1C4, 0x97CC, 0x93C4, 0xD9E2, 0x982E, 0x886E, 0x8826, 0xB86E, 0x98EE, 0x9866,
    0x9822, 0x89EE, 0x88E6, 0xB9EE, 0x8862, 0xB8E6, 0xB862, 0x9BEE, 0x99E6, 0x98E2, 0x8BE6, 0x89E2,
    0xBBE6, 0xB9E2, 0xD02E, 0xA0DC, 0xA04C, 0xD0EE, 0xD066, 0xD022, 0xA3DC, 0xA1CC, 0xA0C4, 0xD3EE,
    0xD1E6, 0xD0E2, 0xAFDC, 0xA7CC, 0xA3C4, 0x906E, 0x9026, 0xB0EE, 0xB066, 0x91EE, 0x90E6, 0x9062,
    0xB3EE, 0xB1E6, 0xB0E2, 0x97EE, 0x93E6, 0x91E2, 0xD4E0, 0xF538, 0xFD4E, 0xA8C0, 0xEA30, 0xFA8C,
    0xD420, 0xF508, 0xFD42, 0xDAF0, 0xF6BC, 0xCA70, 0xF29C, 0xB4E0, 0xED38, 0xFB4E, 0x9460, 0xE518,
    0xF946, 0xDA30, 0xF68C, 0xCA10, 0xF284, 0xB420, 0xED08, 0xFB42, 0xCD78, 0xF35E, 0xBAF0, 0xEEBC,
    0xC538, 0xF14E, 0x9A70, 0xE69C, 0xDD38, 0xF74E, 0x8A30, 0xE28C, 0xCD18, 0xF346, 0xBA30, 0xEE8C,
    0xC508, 0xF142, 0x9A10, 0xE684, 0xDD08, 0xF742, 0xC6BC, 0x9D78, 0xE75E, 0xDEBC, 0xC29C, 0x8D38,
    0xE34E, 0xCE9C, 0xBD38, 0xEF4E, 0x8518, 0xE146, 0xC68C, 0x9D18, 0xE746, 0xDE8C, 0xC284, 0x8D08,
    0xE342, 0xCE84, 0xBD08, 0xEF42, 0xC35E, 0x8EBC, 0xCF5E, 0xBEBC, 0xC14E, 0x869C, 0xC74E, 0x9E9C,
    0xDF4E, 0x828C, 0xC346, 0x8E8C, 0xCF46, 0xBE8C, 0xC142, 0x8684, 0xC742, 0x9E84, 0xDF42, 0x875E,
    0x9F5E, 0x834E, 0x8F4E, 0xBF4E, 0x8146, 0x8746, 0x9F46, 0x8342, 0x8F42, 0xBF42, 0xE978, 0xFA5E,
    0xD6F0, 0xF5BC, 0xD270, 0xF49C, 0xACE0, 0xEB38, 0xFACE, 0xA460, 0xE918, 0xFA46, 0xD630, 0xF58C,
    0xD210, 0xF484, 0xAC20, 0xEB08, 0xFAC2, 0x92F0, 0xE4BC, 0xD978, 0xF65E, 0xCB78, 0xF2DE, 0xB6F0,
    0xC938, 0xF24E, 0xB270, 0xEC9C, 0x9670, 0xE59C, 0x9230, 0xE48C, 0xD918, 0xF646, 0xCB18, 0xF2C6,
    0xB630, 0xC908, 0xF242, 0xB210, 0xEC84, 0x9610, 0xE584, 0xDB08, 0xF6C2, 0x8978, 0xE25E, 0xCCBC,
    0xB978, 0xEE5E, 0xC5BC, 0x9B78, 0xC49C, 0x9938, 0xE64E, 0xDC9C, 0x8B38, 0xE2CE, 0x8918, 0xE246,
    0xBB38, 0xCC8C, 0xB918, 0xEE46, 0xC58C, 0x9B18, 0xC484, 0x9908, 0xE642, 0xDC84, 0x8B08, 0xE2C2,
    0xCD84, 0xBB08, 0xEEC2, 0x84BC, 0xC65E, 0x9CBC, 0xDE5E, 0xC2DE, 0x8DBC, 0xC24E, 0x8C9C, 0xBDBC,
    0xCE4E, 0xBC9C, 0x859C, 0x848C, 0x9D9C, 0xC646, 0x9C8C, 0xDE46, 0xC2C6, 0x8D8C, 0xC242, 0x8C84,
    0xBD8C, 0xCE42, 0xBC84, 0x8584, 0xC6C2, 0x9D84, 0xDEC2, 0x825E, 0x8E5E, 0xBE5E, 0x86DE, 0x864E,
    0x9EDE, 0x9E4E, 0x82CE, 0x8246, 0x8ECE, 0x8E46, 0xBECE, 0xBE46, 0x86C6, 0x8642, 0x9EC6, 0x9E42,
    0x82C2, 0x8EC2, 0xBEC2, 0xA2F0, 0xE8BC, 0xD378, 0xF4DE, 0xD138, 0xF44E, 0xAEF0, 0xEBBC, 0xA670,
    0xE99C, 0xA230, 0xE88C, 0xD738, 0xF5CE, 0xD318, 0xF4C6, 0xD108, 0xF442, 0xAE30, 0xEB8C, 0xA610,
    0xE984, 0xD708, 0xF5C2, 0x9178, 0xE45E, 0xD8BC, 0xC9BC, 0xB378, 0xC89C, 0xB138, 0xEC4E, 0x9778,
    0xE5DE, 0x9338, 0xE4CE, 0x9118, 0xE446, 0xD88C, 0xCB9C, 0xB738, 0xC98C, 0xB318, 0xC884, 0xB108,
    0xEC42, 0x9718, 0xE5C6, 0x9308, 0xE4C2, 0xD984, 0xCB84, 0xB708, 0xEDC2, 0x88BC, 0xCC5E, 0xB8BC,
    0xC4DE, 0x99BC, 0xC44E, 0x989C, 0xDC4E, 0x8BBC, 0x899C, 0xBBBC, 0x888C, 0xB99C, 0xCC46, 0xB88C,
    0xC5CE, 0x9B9C, 0xC4C6, 0x998C, 0xC442, 0x9884, 0xDC42, 0x8B8C, 0x8984, 0xBB8C, 0xCCC2, 0xB984,
    0xC5C2, 0x9B84, 0xDDC2, 0x845E, 0x9C5E, 0x8CDE, 0x8C4E, 0xBCDE, 0xBC4E, 0x85DE, 0x84CE, 0x9DDE,
    0x8446, 0x9CCE, 0x9C46, 0x8DCE, 0x8CC6, 0xBDCE, 0x8C42, 0xBCC6, 0xBC42, 0x85C6, 0x84C2, 0x9DC6,
    0x9CC2, 0x8DC2, 0xBDC2, 0xA178, 0xE85E, 0xD1BC, 0xD09C, 0xA778, 0xE9DE, 0xA338, 0xE8CE, 0xA118,
    0xE846, 0xD7BC, 0xD39C, 0xD18C, 0xD084, 0xAF38, 0xEBCE, 0xA718, 0xE9C6, 0xA308, 0xE8C2, 0xD78C,
    0xD384, 0x90BC, 0xD85E, 0xC8DE, 0xB1BC, 0xC84E, 0xB09C, 0x93BC, 0x919C, 0x908C, 0xD846, 0xCBDE,
    0xB7BC, 0xC9CE, 0xB39C, 0xC8C6, 0xB18C, 0xC842, 0xB084, 0x979C, 0x938C, 0x9184, 0xD8C2, 0xCBC6,
    0xB78C, 0xC9C2, 0xB384, 0x885E, 0xB85E, 0x98DE, 0x984E, 0x89DE, 0x88CE, 0xB9DE, 0x8846, 0xB8CE,
    0xB846, 0x9BDE, 0x99CE, 0x98C6, 0x9842, 0x8BCE, 0x89C6, 0xBBCE, 0x88C2, 0xB9C6, 0xB8C2, 0x9BC6,
    0x99C2, 0xA0BC, 0xD0DE, 0xD04E, 0xA3BC, 0xA19C, 0xA08C, 0xD3DE, 0xD1CE, 0xD0C6, 0xD042, 0xAFBC,
    0xA79C, 0xA38C, 0xA184, 0xD7CE, 0xD3C6, 0xD1C2, 0x905E, 0xB0DE, 0xB04E, 0x91DE, 0x90CE, 0x9046,
    0xB3DE, 0xB1CE, 0xB0C6, 0xB042, 0x97DE, 0x93CE, 0x91C6, 0x90C2, 0xB7CE, 0xB3C6, 0xB1C2, 0xA05E,
    0xA1DE, 0xA0CE, 0xA046, 0xA7DE, 0xA3CE, 0xA1C6, 0xA0C2, 0xA9E0, 0xEA78, 0xFA9E, 0xD470, 0xF51C,
    0xA860, 0xEA18, 0xFA86, 0xD410, 0xF504, 0xED7C, 0x94F0, 0xE53C, 0xDA78, 0xF69E, 0xCA38, 0xF28E,
    0xB470, 0xED1C, 0x9430, 0xE50C, 0xDA18, 0xF686, 0xCA08, 0xF282, 0xB410, 0xED04, 0x9AF8, 0xE6BE,
    0xDD7C, 0x8A78, 0xE29E, 0xCD3C, 0xBA78, 0xEE9E, 0xC51C, 0x9A38, 0xE68E, 0xDD1C, 0x8A18, 0xE286,
    0xCD0C, 0xBA18, 0xEE86, 0xC504, 0x9A08, 0xE682, 0xDD04, 0x8D7C, 0xCEBE, 0xBD7C, 0x853C, 0xC69E,
    0x9D3C, 0xDE9E, 0xC28E, 0x8D1C, 0xCE8E, 0xBD1C, 0x850C, 0xC686, 0x9D0C, 0xDE86, 0xC282, 0x8D04,
    0xCE82, 0xBD04, 0x86BE, 0x9EBE, 0x829E, 0x8E9E, 0xBE9E, 0x868E, 0x9E8E, 0x8286, 0x8E86, 0xBE86,
    0x8682, 0x9E82, 0xD2F8, 0xF4BE, 0xADF0, 0xEB7C, 0xA4F0, 0xE93C, 0xD678, 0xF59E, 0xD238, 0xF48E,
    0xAC70, 0xEB1C, 0xA430, 0xE90C, 0xD618, 0xF586, 0xD208, 0xF482, 0xAC10, 0xEB04, 0xC97C, 0xB2F8,
    0xECBE, 0x96F8, 0xE5BE, 0x9278, 0xE49E, 0xD93C, 0xCB3C, 0xB678, 0xC91C, 0xB238, 0xEC8E, 0x9638,
    0xE58E, 0x9218, 0xE486, 0xD90C, 0xCB0C, 0xB618, 0xC904, 0xB208, 0xEC82, 0x9608, 0xE582, 0xDB04,
    0xC4BE, 0x997C, 0xDCBE, 0x8B7C, 0x893C, 0xBB7C, 0xCC9E, 0xB93C, 0xC59E, 0x9B3C, 0xC48E, 0x991C,
    0xDC8E, 0x8B1C, 0x890C, 0xBB1C, 0xCC86, 0xB90C, 0xC586, 0x9B0C, 0xC482, 0x9904, 0xDC82, 0x8B04,
    0xCD82, 0xBB04, 0x8CBE, 0xBCBE, 0x85BE, 0x849E, 0x9DBE, 0x9C9E, 0x8D9E, 0x8C8E, 0xBD9E, 0xBC8E,
    0x858E, 0x8486, 0x9D8E, 0x9C86, 0x8D86, 0x8C82, 0xBD86, 0xBC82, 0x8582, 0x9D82, 0xD17C, 0xA6F8,
    0xE9BE, 0xA278, 0xE89E, 0xD77C, 0xD33C, 0xD11C, 0xAE78, 0xEB9E, 0xA638, 0xE98E, 0xA218, 0xE886,
    0xD71C, 0xD30C, 0xD104, 0xAE18, 0xEB86, 0xA608, 0xE982, 0xC8BE, 0xB17C, 0x937C, 0x913C, 0xD89E,
    0xCBBE, 0xB77C, 0xC99E, 0xB33C, 0xC88E, 0xB11C, 0x973C, 0x931C, 0x910C, 0xD886, 0xCB8E, 0xB71C,
    0xC986, 0xB30C, 0xC882, 0xB104, 0x970C, 0x9304, 0xD982, 0x98BE, 0x89BE, 0x889E, 0xB9BE, 0xB89E,
    0x9BBE, 0x999E, 0x988E, 0x8B9E, 0x898E, 0xBB9E, 0x8886, 0xB98E, 0xB886, 0x9B8E, 0x9986, 0x9882,
    0x8B86, 0x8982, 0xBB86, 0xB982, 0xD0BE, 0xA37C, 0xA13C, 0xD3BE, 0xD19E, 0xD08E, 0xAF7C, 0xA73C,
    0xA31C, 0xA10C, 0xD79E, 0xD38E, 0xD186, 0xD082, 0xAF1C, 0xA70C, 0xA304, 0xB0BE, 0x91BE, 0x909E,
    0xB3BE, 0xB19E, 0xB08E, 0x97BE, 0x939E, 0x918E, 0x9086, 0xB79E, 0xB38E, 0xB186, 0xB082, 0x978E,
    0x9386, 0x9182, 0xA1BE, 0xA09E, 0xA7BE, 0xA39E, 0xA18E, 0xA086, 0xAF9E, 0xA78E, 0xA386, 0xA182,
    0xD4F8, 0xF53E, 0xA8F0, 0xEA3C, 0xD438, 0xF50E, 0xA830, 0xEA0C, 0xD408, 0xF502, 0xDAFC, 0xCA7C,
    0xB4F8, 0xED3E, 0x9478, 0xE51E, 0xDA3C, 0xCA1C, 0xB438, 0xED0E, 0x9418, 0xE506, 0xDA0C, 0xCA04,
    0xB408, 0xED02, 0xCD7E, 0xBAFC, 0xC53E, 0x9A7C, 0xDD3E, 0x8A3C, 0xCD1E, 0xBA3C, 0xC50E, 0x9A1C,
    0xDD0E, 0x8A0C, 0xCD06, 0xBA0C, 0xC502, 0x9A04, 0xDD02, 0x9D7E, 0x8D3E, 0xBD3E, 0x851E, 0x9D1E,
    0x8D0E, 0xBD0E, 0x8506, 0x9D06, 0x8D02, 0xBD02, 0xE97E, 0xD6FC, 0xD27C, 0xACF8, 0xEB3E, 0xA478,
    0xE91E, 0xD63C, 0xD21C, 0xAC38, 0xEB0E, 0xA418, 0xE906, 0xD60C, 0xD204, 0x92FC, 0xD97E, 0xCB7E,
    0xB6FC, 0xC93E, 0xB27C, 0x967C, 0x923C, 0xD91E, 0xCB1E, 0xB63C, 0xC90E, 0xB21C, 0x961C, 0x920C,
    0xD906, 0xCB06, 0xB60C, 0xC902, 0xB204, 0x897E, 0xB97E, 0x9B7E, 0x993E, 0x8B3E, 0x891E, 0xBB3E,
    0xB91E, 0x9B1E, 0x990E, 0x8B0E, 0x8906, 0xBB0E, 0xB906, 0x9B06, 0x9902, 0xA2FC, 0xD37E, 0xD13E,
    0xAEFC,
];

#[cfg(test)]
mod tests {
    use super::*;

    // ----- helpers -----

    /// zint-style input: bytes >= 128 become FNC4 + (b & 127); K1/K2 over the raw bytes.
    fn zint_input(bytes: &[u8]) -> (Vec<u16>, Vec<u32>) {
        let mut d = Vec::new();
        for &b in bytes {
            if b >= 128 {
                d.push(A_FNC4);
            }
            d.push(u16::from(b & 127));
        }
        (d, bytes.iter().map(|&b| u32::from(b)).collect())
    }

    /// zint's automatic (rows, data columns) choice for option_1/option_2.
    fn zint_dims(len: usize, o1: i32, o2: i32) -> (i32, i32) {
        let rows = o1.max(0);
        let mut cols = o2.max(0);
        if rows == 0 && cols == 0 {
            cols = ((len as f64).sqrt().floor() as i32 + 5).clamp(9, 67);
        }
        (rows, if cols > 0 { cols - 5 } else { 0 })
    }

    fn bits(m: &[bool]) -> String {
        m.iter().map(|&b| if b { '1' } else { '0' }).collect()
    }

    fn bar_rows(s: &Stacked) -> Vec<&StackedRow> {
        s.rows
            .iter()
            .filter(|r| matches!(r.height, RowHeight::Bar { .. }))
            .collect()
    }

    /// Structural checks shared by all symbols: full-width binds, separators between rows.
    fn check_layout(s: &Stacked, sep: (usize, usize)) {
        let w = s.width as usize;
        assert!(s.rows.iter().all(|r| r.modules.len() == w));
        let n = s.rows.len();
        assert!(n >= 3 && n % 2 == 1);
        for (i, r) in s.rows.iter().enumerate() {
            if i == 0 || i == n - 1 {
                assert_eq!(r.height, RowHeight::Modules(1));
                assert!(r.modules.iter().all(|&b| b));
            } else if i % 2 == 0 {
                assert_eq!(r.height, RowHeight::Modules(1));
                for (x, &b) in r.modules.iter().enumerate() {
                    assert_eq!(b, x >= sep.0 && x < sep.1, "separator x {x}");
                }
            } else {
                assert_eq!(r.height, RowHeight::Bar { weight: 1 });
            }
        }
    }

    /// Decode a Code 128 row into values (start .. check), verifying the stop and checksum.
    fn c128_decode_row(m: &[bool]) -> Vec<u8> {
        let mut runs = Vec::new();
        let mut i = 0;
        while i < m.len() {
            let mut j = i;
            while j < m.len() && m[j] == m[i] {
                j += 1;
            }
            runs.push(((j - i) as u8, m[i]));
            i = j;
        }
        // Drop trailing light modules (none for Codablock F rows).
        while runs.last().is_some_and(|r| !r.1) {
            runs.pop();
        }
        assert!(runs[0].1, "row must start with a bar");
        let stop: Vec<u8> = runs[runs.len() - 7..].iter().map(|r| r.0).collect();
        assert_eq!(stop, [2, 3, 3, 1, 1, 1, 2], "stop pattern");
        let body = &runs[..runs.len() - 7];
        assert_eq!(body.len() % 6, 0);
        let values: Vec<u8> = body
            .chunks(6)
            .map(|c| {
                let w: Vec<u8> = c.iter().map(|r| r.0).collect();
                C128_WIDTHS
                    .iter()
                    .position(|p| p[..] == w[..])
                    .expect("valid symbol character") as u8
            })
            .collect();
        let check = values[..values.len() - 1]
            .iter()
            .enumerate()
            .map(|(i, &v)| i.max(1) as u32 * u32::from(v))
            .sum::<u32>()
            % 103;
        assert_eq!(
            u32::from(*values.last().unwrap()),
            check,
            "row check character"
        );
        values
    }

    /// Decode Code 128 data values in a given set into bytes (FNC1 -> 0x1D GS unless first,
    /// FNC2/FNC3 dropped, FNC4 -> +128). Returns the final set.
    fn c128_data(values: &[u8], mut set: u8, out: &mut Vec<u32>, fnc4: &mut bool) -> u8 {
        let mut shift = false;
        for &v in values {
            let cur = if shift {
                if set == b'A' { b'B' } else { b'A' }
            } else {
                set
            };
            shift = false;
            let mut emit = |c: u32, out: &mut Vec<u32>| {
                out.push(c + if *fnc4 { 128 } else { 0 });
                *fnc4 = false;
            };
            match (cur, v) {
                (b'C', 0..=99) => {
                    emit(u32::from(v / 10) + 48, out);
                    emit(u32::from(v % 10) + 48, out);
                }
                (b'C', 100) => set = b'B',
                (b'C', 101) => set = b'A',
                (_, 102) => {
                    if !out.is_empty() {
                        out.push(29)
                    }
                }
                (b'A' | b'B', 96 | 97) => {}
                (b'A' | b'B', 98) => shift = true,
                (b'A' | b'B', 99) => set = b'C',
                (b'A', 100) => set = b'B',
                (b'B', 101) => set = b'A',
                (b'A', 101) | (b'B', 100) => *fnc4 = true,
                (b'A', 0..=63) => emit(u32::from(v) + 32, out),
                (b'A', 64..=95) => emit(u32::from(v) - 64, out),
                (b'B', 0..=95) => emit(u32::from(v) + 32, out),
                _ => panic!("bad value {v} in set {}", cur as char),
            }
        }
        set
    }

    fn sum_from_value(v: u8, set: u8) -> u32 {
        let v = u32::from(v);
        if set == b'C' {
            v
        } else if v >= 64 {
            v - 64
        } else if v <= 15 {
            v + 32
        } else {
            v + 22
        }
    }

    /// Independent Codablock F decoder: verifies every row, the row indicators and K1/K2 and
    /// returns the message (FNC4 extended characters as values >= 128).
    fn decode_codablock_f(s: &Stacked) -> Vec<u32> {
        let rows = bar_rows(s);
        let nrows = rows.len();
        let mut msg = Vec::new();
        let mut fnc4 = false;
        let mut k = (0, 0);
        for (r, row) in rows.iter().enumerate() {
            let v = c128_decode_row(&row.modules);
            assert_eq!(v[0], 103, "Start A");
            let set = match v[1] {
                98 => b'A',
                100 => b'B',
                99 => b'C',
                x => panic!("bad selector {x}"),
            };
            let ind_set = if set == b'C' { b'C' } else { b'B' };
            let ri = sum_from_value(v[2], ind_set);
            if r == 0 {
                assert_eq!(ri as usize, nrows - 2, "row count indicator");
            } else {
                assert_eq!(ri as usize, r + 42, "row indicator");
            }
            let end = v.len() - 1 - if r == nrows - 1 { 2 } else { 0 };
            let set_after = c128_data(&v[3..end], set, &mut msg, &mut fnc4);
            if r == nrows - 1 {
                k = (
                    sum_from_value(v[end], set_after),
                    sum_from_value(v[end + 1], set_after),
                );
            }
        }
        let (mut k1, mut k2) = (0, 0);
        for (i, &c) in msg.iter().enumerate() {
            k1 = (k1 + (i as u32 + 1) * c) % 86;
            k2 = (k2 + i as u32 * c) % 86;
        }
        assert_eq!(k, (k1, k2), "K1/K2");
        msg
    }

    fn lcg(seed: &mut u64) -> u32 {
        *seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (*seed >> 33) as u32
    }

    // ----- Codablock F: zint test-suite vectors -----

    /// zint backend/tests/test_codablock.c `test_input`: (data, option_1 rows, option_2 total
    /// columns, expected rows, expected width, codewords (prefix for long symbols)).
    type InputVector = (&'static [u8], i32, i32, usize, usize, &'static str);
    const CBF_INPUT: &[InputVector] = &[
        (b"A", -1, -1, 2, 101, "67 64 40 21 63 64 63 42 6A 67 64 0B 63 64 2B 40 4F 6A"), // 0
        (b"AAA", -1, -1, 2, 101, "67 64 40 21 21 21 63 55 6A 67 64 0B 63 64 0E 57 48 6A"), // 1
        (b"AAAA", -1, -1, 2, 101, "67 64 40 21 21 21 21 65 6A 67 64 0B 63 64 1A 0E 03 6A"), // 2
        (b"AAAAA", -1, -1, 2, 101, "67 64 40 21 21 21 21 65 6A 67 64 0B 21 63 1D 30 14 6A"), // 3
        (b"AAAAAA", -1, -1, 2, 101, "67 64 40 21 21 21 21 65 6A 67 64 0B 21 21 35 5D 2B 6A"), // 4
        (b"1234", -1, -1, 2, 101, "67 63 00 0C 22 64 63 1A 6A 67 64 0B 63 64 3A 1C 29 6A"), // 5
        (b"12345", -1, -1, 2, 101, "67 63 00 0C 22 64 15 49 6A 67 64 0B 63 64 41 44 07 6A"), // 6
        (b"123456", -1, -1, 2, 101, "67 63 00 0C 22 38 64 12 6A 67 64 0B 63 64 2D 50 52 6A"), // 7
        (b"1234567", -1, -1, 2, 101, "67 63 00 0C 22 38 64 12 6A 67 64 0B 17 63 16 02 5B 6A"), // 8
        (b"12345678", -1, -1, 2, 101, "67 63 00 0C 22 38 4E 5C 6A 67 64 0B 63 64 08 1C 64 6A"), // 9
        (b"123456789", -1, -1, 2, 101, "67 63 00 0C 22 38 4E 5C 6A 67 64 0B 19 63 25 4C 65 6A"), // 10
        (b"1234567890", -1, -1, 2, 101, "67 63 00 0C 22 38 4E 5C 6A 67 64 0B 19 10 41 38 62 6A"), // 11
        (b"A123\xf1", -1, -1, 2, 101, "67 64 40 21 11 12 13 54 6A 67 64 0B 64 51 42 28 50 6A"), // 12
        (b"a\xdf", -1, -1, 2, 101, "67 64 40 41 64 3F 63 54 6A 67 64 0B 63 64 5B 1D 06 6A"), // 13
        (b"\x1f\xdf", -1, -1, 2, 101, "67 62 40 5F 65 3F 63 49 6A 67 64 0B 63 64 0F 1D 26 6A"), // 14
        (b"aa\xdf", -1, -1, 2, 101, "67 64 40 41 41 64 3F 10 6A 67 64 0B 63 64 4E 5B 04 6A"), // 15
        (b"\x1f\x1f\xdf", -1, -1, 2, 101, "67 62 40 5F 5F 65 3F 17 6A 67 64 0B 63 64 34 0F 24 6A"), // 16
        (b"aaa\xdf", -1, -1, 2, 101, "67 64 40 41 41 41 63 39 6A 67 64 0B 64 3F 4C 4E 50 6A"), // 17
        (b"\x1f\x1f\x1f\xdf", -1, -1, 2, 101, "67 62 40 5F 5F 5F 63 03 6A 67 64 0B 64 3F 0E 34 1A 6A"), // 18
        (b"a\xe0", -1, -1, 2, 101, "67 64 40 41 64 40 63 59 6A 67 64 0B 63 64 5D 1E 16 6A"), // 19
        (b"\x1f\xe0", -1, -1, 2, 101, "67 62 40 5F 65 62 40 26 6A 67 64 0B 63 64 1B 1E 01 6A"), // 20
        (b"\x1f\xe0a", -1, -1, 2, 101, "67 62 40 5F 64 64 40 2C 6A 67 64 0B 41 63 52 4A 16 6A"), // 21
        (b"aa\xe0", -1, -1, 2, 101, "67 64 40 41 41 64 40 16 6A 67 64 0B 63 64 51 5D 1F 6A"), // 22
        (b"\x1f\x1f\xe0", -1, -1, 2, 101, "67 62 40 5F 5F 63 64 1D 6A 67 64 0B 64 40 37 1B 55 6A"), // 23
        (b"aaa\xe0", -1, -1, 2, 101, "67 64 40 41 41 41 63 39 6A 67 64 0B 64 40 50 51 13 6A"), // 24
        (b"\x1f\x1f\x1f\xe0", -1, -1, 2, 101, "67 62 40 5F 5F 5F 63 03 6A 67 64 0B 64 40 1C 37 0F 6A"), // 25
        (b"\x1f\x80", -1, -1, 2, 101, "67 62 40 5F 65 40 63 4E 6A 67 64 0B 63 64 5D 0A 05 6A"), // 26
        (b"a\x80", -1, -1, 2, 101, "67 64 40 41 64 62 40 31 6A 67 64 0B 63 64 49 0A 08 6A"), // 27
        (b"a\x80\x1f", -1, -1, 2, 101, "67 64 40 41 65 65 40 44 6A 67 62 0B 5F 63 10 12 3E 6A"), // 28
        (b"\x1f\x1f\x80", -1, -1, 2, 101, "67 62 40 5F 5F 65 40 1D 6A 67 64 0B 63 64 0F 5D 0A 6A"), // 29
        (b"aa\x80", -1, -1, 2, 101, "67 64 40 41 41 63 64 1B 6A 67 62 0B 65 40 33 49 21 6A"), // 30
        (b"\x1f\x1f\x1f\x80", -1, -1, 2, 101, "67 62 40 5F 5F 5F 63 03 6A 67 62 0B 65 40 4A 0F 06 6A"), // 31
        (b"aaa\x80", -1, -1, 2, 101, "67 64 40 41 41 41 63 39 6A 67 62 0B 65 40 28 33 34 6A"), // 32
        (b"\x80\xa0\x9f\xe0\xdf\xe1\xff", -1, 10, 4, 112, "67 62 42 65 40 65 00 63 1E 6A 67 62 0B 65 5F 64 64 40 55 6A 67 64 0C 64 3F 64 41 63"), // 33
        (b"\x00a\x1f\x7f}12", -1, -1, 3, 101, "67 62 41 40 62 41 5F 3B 6A 67 64 0B 5F 5D 11 12 2D 6A 67 64 0C 63 64 40 05 26 6A"), // 34
        (b"abcd\xe9f", -1, -1, 3, 101, "67 64 41 41 42 43 44 5D 6A 67 64 0B 64 49 46 63 0A 6A 67 64 0C 63 64 4F 26 02 6A"), // 35
        (b"a12\xe9\x00", -1, -1, 3, 101, "67 64 41 41 11 12 63 2C 6A 67 64 0B 64 49 62 40 2B 6A 67 64 0C 63 64 33 34 31 6A"), // 36
        (b"1234\x01", -1, 11, 2, 123, "67 63 00 0C 22 65 41 63 64 54 6A 67 64 0B 63 64 63 64 3F 20 24 6A"), // 37
        (b"\xc1\xc1\xc1\xc1\xc1\xc199999999999999\xc1\xc1\xc1\xc1\xc1\xc1\xc199999999999999\xc1", 3, -1, 3, 244, "67 64 41 64 21 64 21 64 21 64 21 64 21 64 21 63 63 63 63 63 56 6A 67 63 2B 63 63 63"), // 38
        (b"@g(\xc2\xc2\xc2\xc2\xc25555\xc2\xc2\xc2\xc2\xc2\xc2\xc2\xc2", 2, -1, 2, 255, "67 64 40 20 47 08 64 22 64 22 64 22 64 22 64 22 63 37 37 64 63 4B 6A 67 64 0B 64 22"), // 39
        (b"\xff\x0a\xe0a\x0a1\x0a\xe0A\xe0", -1, -1, 5, 101, "67 64 43 64 5F 65 4A 09 6A 67 64 0B 64 40 41 63 34 6A 67 62 0C 4A 11 4A 63 25 6A 67"), // 40
        (b"\xffy1234\x0a\xe0a\x0a1\x0a7890\xe0A\xe0D\xe0\x0a\xe0", -1, -1, 7, 112, "67 64 45 64 5F 59 11 12 2E 6A 67 62 0B 13 14 4A 63 64 43 6A 67 64 0C 64 40 41 65 4A"), // 41
        (b"\xff12345678\x0a\xe012345678abcdef\x0a1\x0a23456\x0a\x0a7890\xe0A\xe0BCDEF\xe0\x0a\x0a\xe0", -1, -1, 8, 134, "67 64 46 64 5F 63 0C 22 38 4E 5E 6A 67 62 0B 4A 64 64 40 63 0C 22 2B 6A 67 63 2C 38"), // 42
        (b"\xff12345678\x0a\xe012345678abcdef\x0a\x0a123456\x0a\x0a7890\xe0ABCDEF\xe0\x0a\x0a\xe0\xe0GHIJK\xe0LMNOPQabc\x0adef\xe01234567ghijk\x0a\x0a\xe0901234\x0a\x0a\x0a\x0a567890\xe0\xe0\xe0\xe0ABCDEFG\xe0\xe0\xe0\x0a\x0a\x0aHIJK\x0a\x0a\x0a\x0a\xe0\x0a\xe0\x0a\xe0\x0a12345\xe0\x0a\xe0\x0a\xe067890\xe0\xe0\xc49012\xc4\xc456789\xc412\xc43\x0a4\x0a5\x0a6A\x0aa\x0aA\x0aA\x0aa\x0aa\x0aBCD1A2B3C4a5b6c7d8e9\x0a0\x0a1\x0a2\x0a3\xc44\xc45\xc46A7a8A9a0\x0a\xc412345678\xc4\xc4\xc4\xc4\xc4\xc42\xc4\xc4\xc4\xc4\xc4\xc44\xc4\xc4\xc4\xc4\xc4\xc4a\xc4\xc4\xc4\xc4\xe91\xe92\xe934\xe956\xc478\xe99\xe9A\xe9B\xe9C\xe9\xe9\xe9\xe9\xe9a\xe9b\xe9c\xe9123456789012345ABCDEFGHI\x0a123456\xff12345678\x0a\xe012345678abcdef\x0a123456\x0a\x0a7890\xe0ABCDEF\xe0\x0a\x0a\xe0\xe0GHIJK\xe0LMNOPQabc\x0adef\xe01234567ghijk\x0a\x0a\xe0901234\x0a2567890\xe0\xe0\xe0\xe0\xe0ABCDEFG\xe0\xe0\xe0\x0a\x0a\x0aHIJK\x0a\x0a\x0a2\xe0\x0a\xe0\x0a\xe0\x0a12345\xe0\x0a\xe0\x0a\xe067890\xe0\xe0\xc49012\xc4\xc456789\xc412\xc43\x0a4\x0a5\x0a6A\x0aa\x0aA\x0aA\x0aa\x0aa\x0aBCD1A2B3C4a5b6c7d8e9\x0a0\x0a1\x0a2\x0a3\xc44\xc45\xc46A7a8A9a0\x0a\xc412345678\xc4\xc42\xc44\xc4a\xc4\xe91\xe92\xe934\xe956\xc478\xe99\xe9A\xe9B\xe9C\xe9\xe9a\xe9b\xe9c\xe9123456789012345ABCDEF\x0a123456\xff123456\x0a\xe012345678abcdef\x0a\x0a1234\x0a\x0a7890\xe0ABCD\xe0\x0a\x0a\xe0\xe0GHIJK\xe0LMabc\x0adef\xe01234567ghijk\x0a\x0a\xe0901234\x0a\x0a\x0a\x0a567890\xe0\xe0ABCDEFG\xe0\xe0\xe0\x0a\x0a\x0aHIJK\x0a\x0a\x0a\x0a\xe0\x0a\xe0\x0a\xe0\x0a12345\xe0\x0a\xe0\x0a\xe067890\xe0\xe0\xc49012\xc4\xc456789\xc412\xc43\x0a4\x0a5\x0a6A\x0aa\x0aA\x0aA\x0aa\x0aa\x0aBCD1A2", -1, -1, 33, 387, "67 64 5F 64 5F 63 0C 22 38 4E 65 4A 64 64 40 63 0C 22 38 4E 64 41 42 43 44 45 46"), // 44
        (b"\x01\x02\x03\x04\x05\x06\x07\x08\x09", -1, -1, 3, 101, "67 62 41 41 42 43 44 5B 6A 67 62 0B 45 46 47 48 37 6A 67 62 0C 49 63 1B 44 2C 6A"), // 45
        (b"\x80\x81\x82\x83\x9e\x9f\xa0", 4, -1, 4, 101, "67 62 42 65 40 65 41 24 6A 67 62 0B 65 42 65 43 31 6A 67 62 0C 65 5E 65 5F 16 6A 67"), // 46
        (b"\x00z", -1, -1, 2, 101, "67 62 40 40 62 5A 63 00 6A 67 64 0B 63 64 32 04 3F 6A"), // 47
        (b"2610\xf2", -1, -1, 2, 101, "67 63 00 1A 0A 64 63 4B 6A 67 64 0B 64 52 33 26 64 6A"), // 48
        (b"*\x0d\xf2\x82\x82(\x982\x82\x82*\x0d\xf2\x82\xa8\x82\x82\x82\x82", 7, -1, 7, 123, "67 62 45 0A 4D 64 64 52 63 35 6A 67 62 0B 65 42 65 42 08 63 43 6A 67 62 0C 65 58 12"), // 49
        (b"\x18\xf2", -1, -1, 2, 101, "67 62 40 58 65 62 52 16 6A 67 64 0B 63 64 38 30 30 6A"), // 50
        (b"\xae^a\xef\x1b\xb8\x0f,\xbcu$B\xc5\xc9\x06\x09]\xbb\x15u\xcd\xb38\xb3\xdb", -1, -1, 7, 123, "67 64 45 64 0E 3E 41 64 4F 33 6A 67 62 0B 5B 65 18 4F 0C 63 01 6A 67 64 0C 64 1C 55"), // 51
        (b"\xee\x9fV\xc8\xb6767", -1, -1, 5, 101, "67 64 43 64 4E 63 64 57 6A 67 62 0B 65 5F 36 63 13 6A 67 64 0C 64 28 64 16 53 6A 67"), // 52
    ];

    #[test]
    fn codablock_f_zint_codewords() {
        for (i, &(data, o1, o2, er, ew, exp)) in CBF_INPUT.iter().enumerate() {
            let (d, k) = zint_input(data);
            let (rows, cols) = zint_dims(d.len(), o1, o2);
            let cws = cbf_codewords(&d, &k, rows, cols).unwrap();
            let got: Vec<String> = cws.iter().flatten().map(|v| format!("{v:02X}")).collect();
            let exp: Vec<&str> = exp.split(' ').collect();
            assert_eq!(cws.len(), er, "vector {i} rows");
            assert_eq!(cws[0].len() * 11 + 2, ew, "vector {i} width");
            assert_eq!(&got[..exp.len()], &exp[..], "vector {i} {data:?}");
        }
    }

    /// zint `test_encode`: (data, option_1, option_2, module rows). Includes AIM ISS-X-24
    /// Figure 1 and the front-page figure.
    const CBF_ENCODE: &[(&[u8], i32, i32, &[&str])] = &[
        (b"AIM", 1, -1, &["11010010000101000110001100010001010111011000101110110001100011101011"]), // 0
        (b"AAAAAAA", -1, -1, &["11010000100101111011101001011000010100011000101000110001010001100010100011000110110011001100011101011", "11010000100101111011101100010010010100011000101000110001010001100010111011110100100111101100011101011", "11010000100101111011101011001110010111011110101111011101100001010011011101110111100101001100011101011"]), // 1
        (b"AAAAAAAAAA", -1, -1, &["11010000100101111011101001011000010100011000101000110001010001100010100011000110110011001100011101011", "11010000100101111011101100010010010100011000101000110001010001100010100011000111101000101100011101011", "11010000100101111011101011001110010100011000101000110001110110010010010110000111000101101100011101011"]), // 2
        (b"AAAAAAAAAAA", -1, -1, &["11010000100101111011101001000011010100011000101000110001010001100010100011000110011001101100011101011", "11010000100101111011101100010010010100011000101000110001010001100010100011000111101000101100011101011", "11010000100101111011101011001110010100011000101000110001010001100010111011110100111101001100011101011", "11010000100101111011101001101110010111011110101111011101110101100011101100100110010111001100011101011"]), // 3
        (b"AAAAAAAAAAAAAA", -1, -1, &["11010000100101111011101001000011010100011000101000110001010001100010100011000110011001101100011101011", "11010000100101111011101100010010010100011000101000110001010001100010100011000111101000101100011101011", "11010000100101111011101011001110010100011000101000110001010001100010100011000101111011101100011101011", "11010000100101111011101001101110010100011000101000110001011110100010111011000100110000101100011101011"]), // 4
        (b"AAAAAAAAAAAAAAA", -1, -1, &["11010000100101111011101000010110010100011000101000110001010001100010100011000100100011001100011101011", "11010000100101111011101100010010010100011000101000110001010001100010100011000111101000101100011101011", "11010000100101111011101011001110010100011000101000110001010001100010100011000101111011101100011101011", "11010000100101111011101001101110010100011000101000110001010001100010111011110111101001001100011101011", "11010000100101111011101001100111010111011110101111011101000110001010111101000110001010001100011101011"]), // 5
        (b"AAAAAAAAAAAAAAA", -1, 14, &["110100001001011110111010100001100101000110001010001100010100011000101000110001010001100010100011000101000110001010001100010100011000110001000101100011101011", "110100001001011110111011000100100101000110001010001100010100011000101000110001010001100010100011000101110111101110111101011011000110111000101101100011101011"]), // 6
        (b"AAAAAAAAAAAAAAAA", -1, -1, &["11010000100101111011101000010110010100011000101000110001010001100010100011000100100011001100011101011", "11010000100101111011101100010010010100011000101000110001010001100010100011000111101000101100011101011", "11010000100101111011101011001110010100011000101000110001010001100010100011000101111011101100011101011", "11010000100101111011101001101110010100011000101000110001010001100010100011000111101011101100011101011", "11010000100101111011101001100111010111011110101111011101011100011010001100010100011101101100011101011"]), // 7
        (b"AAAAAAAAAAAAAAAAAAAAAAAAA", -1, -1, &["1101000010010111101110100001001101010001100010100011000101000110001010001100010100011000110110001101100011101011", "1101000010010111101110110001001001010001100010100011000101000110001010001100010100011000110010011101100011101011", "1101000010010111101110101100111001010001100010100011000101000110001010001100010100011000110011101001100011101011", "1101000010010111101110100110111001010001100010100011000101000110001010001100010100011000111010011001100011101011", "1101000010010111101110100110011101010001100010100011000101000110001010001100010100011000111001001101100011101011", "1101000010010111101110101110011001011101111010111101110101110111101110100011010100001100110001010001100011101011"]), // 8
        (b"CODABLOCK F 34567890123456789010040digit", 4, -1, &["1101000010010111101110100100001101000100011010001110110101100010001010001100010001011000100011011101000111011010001000110110110011001100011101011", "1101000010010111101110110001001001011000111011011001100100011000101101100110010111011110100010110001110001011011000010100101100111001100011101011", "1101000010010111011110100011011101101111011010110011100100010110001110001011011000010100110111101101100100010010010001100100011000101100011101011", "1101000010010111101110100110111001001110110010000100110100001101001001101000010000110100100111101001101110111010111000110110010000101100011101011"]), // 9
        (b"CODABLOCK F Symbology", 3, -1, &["1101000010010111101110100101100001000100011010001110110101100010001010001100010001011000100011011101000111011010001000110111010111101100011101011", "1101000010010111101110110001001001011000111011011001100100011000101101100110011011101000110110111101111011101010010000110100100111101100011101011", "1101000010010111101110101100111001000111101011001010000100011110101001101000011011011110101110111101000011001011011101110101001111001100011101011"]), // 10
        (b" !\x22#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\x5c]^_`abcdefghijklmnopqrstuvwxyz{|}~", -1, -1, &["110100001001011110111010000110100110110011001100110110011001100110100100110001001000110010001001100100110010001001100010010001100100101100011101100011101011", "110100001001011110111011000100100110010010001100100010011000100100101100111001001101110010011001110101110011001001110110010011100110100001100101100011101011", "110100001001011101111010001101110111011011101011101100010000101100110110111101011110111011100100110111011001001110011010011100110010100001001101100011101011", "110100001001011110111010011011100110110110001101100011011000110110101000110001000101100010001000110101100010001000110100010001100010111100010101100011101011", "110100001001011110111010011001110110100010001100010100011000100010101101110001011000111010001101110101110110001011100011010001110110100110111001100011101011", "110100001001011110111010111001100111011101101101000111011000101110110111010001101110001011011101110111010110001110100011011100010110100001011001100011101011", "110100001001011110111011100100110111011010001110110001011100011010111011110101100100001011110001010101001100001010000110010010110000100011000101100011101011", "110100001001011110111011101100100100100001101000010110010000100110101100100001011000010010011010000100110000101000011010010000110010101011110001100011101011", "110100001001011110111011100110100110000100101100101000011110111010110000101001000111101010100111100100101111001001001111010111100100101100011101100011101011", "110100001001011110111011100110010100111101001001111001011110100100111100101001111001001011011011110110111101101111011011010101111000111101010001100011101011", "110100001001011110111011011011000101000111101000101111010111011110101111011101011101111010111101110101110111101011100011011101101110101001100001100011101011"]), // 11
    ];

    #[test]
    fn codablock_f_zint_modules() {
        for (i, &(data, o1, o2, exp)) in CBF_ENCODE.iter().enumerate() {
            let got: Vec<String> = if o1 == 1 {
                let s = codablock(
                    std::str::from_utf8(data).unwrap(),
                    CodablockMode::F,
                    0,
                    1,
                    true,
                )
                .unwrap();
                bar_rows(&s).iter().map(|r| bits(&r.modules)).collect()
            } else {
                let (d, k) = zint_input(data);
                let (rows, cols) = zint_dims(d.len(), o1, o2);
                let cws = cbf_codewords(&d, &k, rows, cols).unwrap();
                cws.iter().map(|r| bits(&c128_modules(r))).collect()
            };
            assert_eq!(got, exp, "vector {i}");
        }
    }

    #[test]
    fn codablock_f_public_api_layout() {
        let s = codablock("CODABLOCK F Symbology", CodablockMode::F, 0, 3, true).unwrap();
        assert_eq!(s.width, 145);
        assert_eq!(bar_rows(&s).len(), 3);
        check_layout(&s, (11, 145 - 13));
        assert_eq!(
            bits(&bar_rows(&s)[0].modules),
            "1101000010010111101110100101100001000100011010001110110101100010001010001100010001011000100011011101000111011010001000110111010111101100011101011"
        );
        // Columns given, rows automatic.
        let s = codablock("ABCDEFGHIJKLMNOPQRSTUVWXYZ", CodablockMode::F, 6, 0, false).unwrap();
        assert_eq!(s.width, 11 * 11 + 2);
        assert_eq!(
            decode_codablock_f(&s),
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZ".map(u32::from)
        );
    }

    #[test]
    fn codablock_f_round_trip() {
        let mut seed = 7u64;
        let alphabet: Vec<char> = (0u8..128).map(char::from).chain(['\u{84}']).collect();
        for case in 0..400 {
            let len = 1 + lcg(&mut seed) as usize % 120;
            let digits = case % 3 == 0;
            let s: String = (0..len)
                .map(|_| {
                    if digits && !lcg(&mut seed).is_multiple_of(4) {
                        char::from(b'0' + (lcg(&mut seed) % 10) as u8)
                    } else {
                        alphabet[lcg(&mut seed) as usize % alphabet.len()]
                    }
                })
                .collect();
            let cols = [0, 4, 5, 9, 20, 62][case % 6];
            let rows = if cols == 0 {
                [2, 3, 7, 44][case % 4]
            } else {
                0
            };
            let sym = codablock(&s, CodablockMode::F, cols, rows, true).unwrap();
            let n = bar_rows(&sym).len();
            assert!((2..=44).contains(&n));
            check_layout(&sym, (11, sym.width as usize - 13));
            let want: Vec<u32> = s
                .chars()
                .map(|c| if c == '\u{84}' { 0 } else { c as u32 })
                .collect();
            assert_eq!(decode_codablock_f(&sym), want, "case {case} {s:?}");
        }
    }

    #[test]
    fn codablock_e_and_function_characters() {
        // E = F with an automatic leading FNC1 (not part of K1/K2).
        let s = codablock("0112345678901231", CodablockMode::E, 5, 0, true).unwrap();
        let rows = bar_rows(&s);
        let v = c128_decode_row(&rows[0].modules);
        assert_eq!(&v[..2], &[103, 99], "Start A, Code C");
        assert_eq!(v[3], 102, "FNC1 first");
        assert_eq!(decode_codablock_f(&s), b"0112345678901231".map(u32::from));
        // FNC4 + 'A' = extended 0xC1, embedded FNC1 = GS, FNC3 ignored.
        let s = codablock("x\u{83}Ay\u{80}z\u{82}", CodablockMode::F, 4, 0, true).unwrap();
        assert_eq!(decode_codablock_f(&s), vec![120, 0xC1, 121, 29, 122]);
        assert!(matches!(
            codablock("a\u{83}", CodablockMode::F, 4, 0, true),
            Err(SymbolError::InvalidData(_))
        ));
        assert_eq!(
            codablock("é", CodablockMode::F, 4, 0, true),
            Err(SymbolError::InvalidCharacter('é'))
        );
    }

    #[test]
    fn codablock_f_single_row_is_code128() {
        for data in [
            "AIM",
            "1234567890",
            "ab12345cd",
            "\u{1}abc\u{2}",
            "A\u{80}12\u{83}B",
            "7",
        ] {
            let s = codablock(data, CodablockMode::F, 0, 0, true).unwrap();
            assert_eq!(s.rows.len(), 3);
            check_layout(&s, (0, s.width as usize));
            let v = c128_decode_row(&bar_rows(&s)[0].modules);
            let set = match v[0] {
                103 => b'A',
                104 => b'B',
                105 => b'C',
                _ => panic!(),
            };
            let mut out = Vec::new();
            c128_data(&v[1..v.len() - 1], set, &mut out, &mut false);
            let want: Vec<u32> = data
                .chars()
                .scan(false, |f4, c| {
                    Some(match c as u32 {
                        0x80 => Some(29),
                        0x83 => {
                            *f4 = true;
                            None
                        }
                        x => Some(x + if std::mem::take(f4) { 128 } else { 0 }),
                    })
                })
                .flatten()
                .collect();
            assert_eq!(out, want, "{data:?}");
        }
        // Long column count without rows = single row (ZPL rule).
        let s = codablock("HELLO WORLD", CodablockMode::F, 63, 0, true).unwrap();
        assert_eq!(bar_rows(&s).len(), 1);
    }

    // ----- Codablock A -----

    #[test]
    fn codablock_a_layout() {
        let s = codablock("CODABLOCK A 0123456789", CodablockMode::A, 6, 0, true).unwrap();
        // 22 data + 2 checks over 6 columns = 4 rows; width (6 + 3) * 16 - 1.
        let rows = bar_rows(&s);
        assert_eq!(rows.len(), 4);
        assert_eq!(s.width, 143);
        check_layout(&s, (15, 143 - 15));
        // Decode each row as Code 39 and check the row indicators and checks.
        let mut all = Vec::new();
        for (r, row) in rows.iter().enumerate() {
            let m = &row.modules;
            let mut vals = Vec::new();
            let mut x = 0;
            loop {
                let mut p = 0u16;
                let mut pos = x;
                for e in 0..9 {
                    let mut w = 0;
                    while pos < m.len() && m[pos] == (e % 2 == 0) {
                        w += 1;
                        pos += 1;
                    }
                    assert!(w == 1 || w == 3);
                    p = p << 1 | u16::from(w == 3);
                }
                let v = CODE39_PATTERNS.iter().position(|&q| q == p).unwrap();
                vals.push(v);
                x = pos + 1;
                if v == 43 && vals.len() > 1 {
                    break;
                }
            }
            assert!(m[x - 1..].iter().all(|&b| !b), "light after stop");
            assert_eq!(vals[0], 43);
            assert_eq!(vals[1], if r == 0 { 2 } else { r + 21 });
            all.extend_from_slice(&vals[2..vals.len() - 1]);
        }
        let (data, k) = all.split_at(all.len() - 2);
        let text: String = data.iter().map(|&v| C49_SET[v] as char).collect();
        assert_eq!(text, "CODABLOCK A 0123456789");
        assert_eq!(k[0], data.iter().sum::<usize>() % 43);
        // Without security: no check characters; single row when nothing is specified.
        let s = codablock("AB", CodablockMode::A, 1, 0, false).unwrap();
        assert_eq!(bar_rows(&s).len(), 2);
        let s = codablock("ABC", CodablockMode::A, 0, 0, true).unwrap();
        assert_eq!(bar_rows(&s).len(), 1);
        assert_eq!(s.width, 5 * 16 - 1);
        assert_eq!(
            codablock("a_b", CodablockMode::A, 2, 0, true),
            Err(SymbolError::InvalidCharacter('_'))
        );
    }

    // ----- Code 49 -----

    /// zint test_code49.c `test_input` codeword grids (ANSI/AIM BC6-2000 examples).
    const C49_GRIDS: &[(&str, &str)] = &[
        ("EXAMPLE 2", "14 33 10 22 25 21 14 41 38 2 35 14 18 13 0 22"),
        ("12345", "5 17 9 48 48 48 48 27 48 48 13 23 0 13 2 0"),
        ("123456", "5 17 9 6 48 48 48 34 48 48 36 9 23 41 2 11"),
        ("12345678", "5 17 9 14 6 48 48 0 48 48 25 42 2 17 2 37"),
        ("123456789", "5 17 9 46 16 37 48 31 48 48 7 26 9 39 2 32"),
        ("1234567", "43 45 2 11 39 48 48 40 48 48 33 36 38 6 2 15"),
        ("\x1f", "5 48 48 48 48 48 48 48 48 48 4 33 13 15 4 18"),
        ("\x00\x1f", "38 43 5 48 48 48 48 33 48 48 45 7 38 43 4 37"),
        ("a\x00", "10 43 38 48 48 48 48 38 48 48 32 33 14 15 5 48"),
        ("ab", "10 44 11 48 48 48 48 12 48 48 27 39 42 0 5 13"),
        (
            "\x00A\x00a\x00",
            "38 10 43 38 44 10 43 30 38 48 25 23 38 32 4 12",
        ),
        (
            "1234\x1faA12345A",
            "1 2 3 4 43 5 44 4 10 10 48 5 17 9 48 0 10 48 19 2 13 32 7 33",
        ),
        (
            "1234567890123456789012345678901234567890",
            "5 17 9 29 22 18 5 7 17 9 29 22 18 5 17 19 9 29 22 18 5 17 9 11 29 22 18 48 48 48 48 16",
        ),
        (
            "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVW",
            "10 11 12 13 14 15 16 42 17 18 19 20 21 22 23 42 24 25 26 27 28 29 30 42 31 32 33 34 35",
        ),
    ];

    fn c49_auto_grid(data: &str) -> Vec<[u8; 8]> {
        let mut cw = c49_auto(data.as_bytes());
        let m = match cw[0] {
            C49_NS => 2,
            C49_S1 => 4,
            C49_S2 => 5,
            _ => 0,
        };
        if m != 0 {
            cw.remove(0);
        }
        c49_grid(&cw, m)
    }

    #[test]
    fn code49_zint_grids() {
        for &(data, exp) in C49_GRIDS {
            let got: Vec<String> = c49_auto_grid(data)
                .iter()
                .flatten()
                .map(|v| v.to_string())
                .collect();
            let exp: Vec<&str> = exp.split(' ').collect();
            assert_eq!(&got[..exp.len()], &exp[..], "{data:?}");
        }
        assert_eq!(
            c49_auto_grid("1234567890123456789012345678901234567890").len(),
            5
        );
        assert_eq!(
            code49("ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWX", None),
            Err(SymbolError::InvalidLength(50))
        );
    }

    #[test]
    fn code49_zint_modules() {
        // ANSI/AIM BC6-2000 Figures 1 and 3 (via zint test_encode).
        let cases: &[(&str, &[&str])] = &[
            (
                "MULTIPLE ROWS IN CODE 49",
                &[
                    "1011111011001011101011100110000110111101011011111010111101000100001111",
                    "1010100001000010001001111000101110100110001111010010001011100011001111",
                    "1011001100000101101101110111000010110010110000111011101011110001101111",
                    "1010011001100100001111010010001100101011101111110011010001001111101111",
                    "1011001111001011101000000101001110111110111010001011010001101111101111",
                ],
            ),
            (
                "EXAMPLE 2",
                &[
                    "1011000111011100101111001001000110110011110010100010001111000100101111",
                    "1011000100110010001100010110010000100001101001111010000001001011101111",
                ],
            ),
        ];
        for &(data, exp) in cases {
            let (s, text) = code49(data, None).unwrap();
            assert_eq!(text, data);
            check_layout(&s, (0, 70));
            let got: Vec<String> = bar_rows(&s).iter().map(|r| bits(&r.modules)).collect();
            assert_eq!(got, exp, "{data}");
        }
    }

    /// Independent Code 49 decoder: symbol characters -> grid (checking parity per row),
    /// row checks, X/Y/Z checks, row count, then the message.
    fn decode_code49(s: &Stacked) -> (String, u8) {
        let rows = bar_rows(s);
        let n = rows.len();
        let mut grid = Vec::new();
        for (r, row) in rows.iter().enumerate() {
            let m = &row.modules;
            assert_eq!(bits(&m[..2]), "10");
            assert_eq!(bits(&m[66..]), "1111");
            let mut g = [0u8; 8];
            for j in 0..4 {
                let p = m[2 + 16 * j..18 + 16 * j]
                    .iter()
                    .fold(0u16, |a, &b| a << 1 | u16::from(b));
                let odd = r != n - 1 && C49_PARITY[r][j];
                let table = if odd { &C49_ODD } else { &C49_EVEN };
                let w = table
                    .iter()
                    .position(|&q| q == p)
                    .expect("symbol character");
                g[2 * j] = (w / 49) as u8;
                g[2 * j + 1] = (w % 49) as u8;
            }
            let sum: u32 = g[..7].iter().map(|&v| u32::from(v)).sum();
            assert_eq!(u32::from(g[7]), sum % 49, "row {r} check");
            grid.push(g);
        }
        let last = grid[n - 1];
        let ri = u32::from(last[6]);
        let m = ri - 7 * (n as u32 - 2);
        assert!(m <= 6);
        // Symbol checks over all symbol characters in order.
        let mut words: Vec<u32> = grid[..n - 1]
            .iter()
            .flat_map(|g| (0..4).map(move |j| u32::from(g[2 * j]) * 49 + u32::from(g[2 * j + 1])))
            .collect();
        let z = words
            .iter()
            .zip(C49_Z_WEIGHT)
            .map(|(w, k)| w * k)
            .sum::<u32>()
            + ri * 38;
        if n > 6 {
            assert_eq!(
                u32::from(last[0]) * 49 + u32::from(last[1]),
                z % 2401,
                "Z check"
            );
        }
        words.push(u32::from(last[0]) * 49 + u32::from(last[1]));
        let y = words
            .iter()
            .zip(C49_Y_WEIGHT)
            .map(|(w, k)| w * k)
            .sum::<u32>()
            + ri * 16;
        assert_eq!(
            u32::from(last[2]) * 49 + u32::from(last[3]),
            y % 2401,
            "Y check"
        );
        words.push(u32::from(last[2]) * 49 + u32::from(last[3]));
        let x = words
            .iter()
            .zip(C49_X_WEIGHT)
            .map(|(w, k)| w * k)
            .sum::<u32>()
            + ri * 20;
        assert_eq!(
            u32::from(last[4]) * 49 + u32::from(last[5]),
            x % 2401,
            "X check"
        );
        // Message.
        let mut cw: Vec<u8> = grid[..n - 1].iter().flat_map(|g| g[..7].to_vec()).collect();
        if n <= 6 {
            cw.extend_from_slice(&last[..2]);
        }
        while cw.last() == Some(&C49_NS) {
            cw.pop();
        }
        let mut out = String::new();
        let mut numeric = m == 2;
        let mut shift = match m {
            4 => C49_S1,
            5 => C49_S2,
            _ => 0,
        };
        let mut i = 0;
        while i < cw.len() {
            if numeric {
                let end = cw[i..]
                    .iter()
                    .position(|&v| v == C49_NS)
                    .map_or(cw.len(), |p| i + p);
                let mut j = i;
                while end - j >= 3 {
                    let v = cw[j..j + 3]
                        .iter()
                        .fold(0u32, |a, &c| a * 48 + u32::from(c));
                    if v >= 100_000 {
                        out += &format!("{:04}", v - 100_000);
                    } else {
                        out += &format!("{v:05}");
                    }
                    j += 3;
                }
                match end - j {
                    2 => out += &format!("{:03}", u32::from(cw[j]) * 48 + u32::from(cw[j + 1])),
                    1 => out += &cw[j].to_string(),
                    _ => {}
                }
                i = end + 1;
                numeric = false;
                continue;
            }
            match cw[i] {
                C49_S1 | C49_S2 => shift = cw[i],
                C49_NS => numeric = true,
                v @ 0..=42 => {
                    if shift != 0 {
                        out.push(c49_to_ascii(shift, v).expect("valid shifted character"));
                        shift = 0;
                    } else {
                        out.push(C49_SET[v as usize] as char);
                    }
                }
                v => panic!("unexpected codeword {v}"),
            }
            i += 1;
        }
        (out, m as u8)
    }

    #[test]
    fn code49_round_trip() {
        let mut seed = 49u64;
        for case in 0..600 {
            let len = 1 + lcg(&mut seed) as usize % 40;
            let digits = case % 2 == 0;
            let s: String = (0..len)
                .map(|_| {
                    if digits && !lcg(&mut seed).is_multiple_of(5) {
                        char::from(b'0' + (lcg(&mut seed) % 10) as u8)
                    } else {
                        char::from((lcg(&mut seed) % 128) as u8)
                    }
                })
                .collect();
            match code49(&s, None) {
                Ok((sym, text)) => {
                    assert_eq!(text, s);
                    let n = bar_rows(&sym).len();
                    assert!((2..=8).contains(&n));
                    check_layout(&sym, (0, 70));
                    assert_eq!(decode_code49(&sym).0, s, "case {case}");
                }
                Err(SymbolError::InvalidLength(_)) => assert!(len > 20),
                Err(e) => panic!("{e:?}"),
            }
        }
    }

    #[test]
    fn code49_manual_modes() {
        // Mode 0: internal set; '>' = Shift 2 so ">A" is 'a'.
        let (s, text) = code49("AB>C12", Some(0)).unwrap();
        assert_eq!(text, "ABc12");
        assert_eq!(decode_code49(&s), ("ABc12".into(), 0));
        // Mode 2: starts numeric; '=' returns to alphanumeric.
        let (s, text) = code49("1234567=AB", Some(2)).unwrap();
        assert_eq!(text, "1234567AB");
        assert_eq!(decode_code49(&s), ("1234567AB".into(), 2));
        // Mode 4: implied Shift 1 on the first character (Shift 1 'A' = SOH).
        let (s, text) = code49("AB", Some(4)).unwrap();
        assert_eq!(text, "\u{1}B");
        assert_eq!(decode_code49(&s), ("\u{1}B".into(), 4));
        // Mode 5: implied Shift 2.
        let (s, _) = code49("AB", Some(5)).unwrap();
        assert_eq!(decode_code49(&s), ("aB".into(), 5));
        // Modes 1 and 3 encode like mode 0 with their own mode value.
        let (s, _) = code49("AB", Some(3)).unwrap();
        assert_eq!(decode_code49(&s).1, 3);
        // Invalid sequences truncate: lowercase, double shift, N.A. characters.
        assert_eq!(code49("AB<<C", Some(0)).unwrap().1, "AB");
        assert_eq!(code49("AB:C", Some(0)).unwrap().1, "AB");
        assert_eq!(code49("123456A", Some(2)).unwrap().1, "123456");
        assert!(
            code49("12A", Some(2)).is_err(),
            "a lone digit pair is not encodable"
        );
        assert!(code49("a", Some(0)).is_err());
        // Automatic mode equals zint, matching the starting mode selection.
        assert_eq!(decode_code49(&code49("12345", None).unwrap().0).1, 2);
        assert_eq!(decode_code49(&code49("\u{1f}", None).unwrap().0).1, 4);
        assert_eq!(decode_code49(&code49("ab", None).unwrap().0).1, 5);
        assert_eq!(code49("é", None), Err(SymbolError::InvalidCharacter('é')));
    }

    /// Writes symbols for the external zxing-cpp check when `STACKED_DUMP` names a file.
    #[test]
    fn dump_for_external_check() {
        let Ok(path) = std::env::var("STACKED_DUMP") else {
            return;
        };
        let mut out = String::new();
        let samples = [
            "CODABLOCK F Symbology",
            "CODABLOCK F 34567890123456789010040digit",
            "The quick brown fox jumps over the lazy dog 0123456789!",
            "\u{1}\u{2}control\tchars and 12345678 digits",
        ];
        for (i, d) in samples.iter().enumerate() {
            for (cols, rows) in [(4, 0), (8, 0), (0, 3)] {
                let s = codablock(d, CodablockMode::F, cols, rows, true).unwrap();
                out += &format!("F {i} {cols} {rows}\n");
                for r in bar_rows(&s) {
                    out += &bits(&r.modules);
                    out.push('\n');
                }
            }
            let s = codablock(d, CodablockMode::F, 0, 1, true).unwrap();
            out += &format!("S {i}\n{}\n", bits(&bar_rows(&s)[0].modules));
        }
        std::fs::write(path, out).unwrap();
    }
}
