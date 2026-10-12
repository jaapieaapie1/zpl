//! MaxiCode (ISO/IEC 16023, ZPL `^BD`).
//!
//! A MaxiCode symbol is a fixed 33-row x 30-column grid of hexagonal modules around a bullseye
//! finder. Odd rows (1, 3, ...) are shifted right by half a module and only use columns 0..29.
//! [`encode`] produces the module grid; [`MaxiCode::geometry`] lays it out in printer dots.
//!
//! ZPL field data (`^FD`) for modes 2 and 3 carries the structured carrier message:
//!
//! * `<hpm><lpm>`: the high priority message `aaabbbccccccccc` (mode 2: 3-digit class of
//!   service, 3-digit country code, 9-digit postal code) or `aaabbbcccccc` (mode 3: class,
//!   country, 6-character alphanumeric postal code), followed by the low priority message which
//!   is encoded verbatim in the secondary message (it usually starts with the `[)>RS01GS96`
//!   header).
//! * `[)>RS01GSyy<postal>GS<country>GS<class>GS<rest>`: data that starts with the ANSI MH10.8.3
//!   header carries the primary fields as the first three GS-separated fields after the header
//!   (ISO/IEC 16023 Annex B). They go to the primary message and the secondary message is the
//!   header followed by `<rest>`, which is what decoders reassemble.
//!
//! Modes 4, 5 and 6 encode the whole field data. Encoding uses the minimal code set / numeric
//! compression path search (Bue Jensen's algorithm, as in BWIPP and zint); symbols are
//! module-for-module identical to zint's.

use crate::SymbolError;

/// Columns of the module grid.
pub const COLUMNS: u32 = 30;
/// Rows of the module grid.
pub const ROWS: u32 = 33;

/// Nominal symbol width (ISO/IEC 16023), mm.
const WIDTH_MM: f32 = 28.14;
/// Nominal symbol height (ISO/IEC 16023), mm.
const HEIGHT_MM: f32 = 26.91;

/// A MaxiCode symbol: 33 rows x 30 columns of modules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaxiCode {
    /// Bit `col` of `rows[row]` set = dark.
    rows: [u32; ROWS as usize],
}

/// Physical layout of a [`MaxiCode`] in dots, origin top-left, y down.
#[derive(Debug, Clone, PartialEq)]
pub struct Geometry {
    /// Dots.
    pub width: f32,
    /// Dots.
    pub height: f32,
    /// Centres of the dark hexagons, origin top-left, y down.
    pub hexagons: Vec<(f32, f32)>,
    /// Hexagon circumradius (pointy-top hexagons: vertices at top and bottom), dots.
    pub hex_radius: f32,
    /// Centre of the bullseye finder.
    pub center: (f32, f32),
    /// Dark rings of the bullseye as (outer_radius, inner_radius) pairs, outermost first. The
    /// finder has three dark rings around a light centre (ISO/IEC 16023 4.2.1.1), so there are
    /// always three entries and no central dark disc.
    pub rings: Vec<(f32, f32)>,
}

impl MaxiCode {
    /// Module at column `col` (0..30), row `row` (0..33); true = dark. Out of range = light.
    #[inline]
    pub fn get(&self, col: u32, row: u32) -> bool {
        col < COLUMNS && row < ROWS && (self.rows[row as usize] >> col) & 1 != 0
    }

    /// Physical layout in dots for the given resolution (dots per mm; 8.0 for 203 dpi).
    ///
    /// The symbol is scaled to the nominal 28.14 mm x 26.91 mm: the horizontal module pitch is
    /// width / 30 (adjacent hexagons touch), the hexagons are regular with that width, and the
    /// row pitch spreads the 33 rows over the height. The finder is centred on the module at
    /// row 16, column 14 and is 9 module pitches across (zint/ISO proportions).
    pub fn geometry(&self, dots_per_mm: f32) -> Geometry {
        let width = WIDTH_MM * dots_per_mm;
        let height = HEIGHT_MM * dots_per_mm;
        let pitch_x = width / COLUMNS as f32;
        // Regular pointy-top hexagon whose flat-to-flat width is the column pitch.
        let hex_radius = pitch_x / 3f32.sqrt();
        let pitch_y = (height - 2.0 * hex_radius) / (ROWS - 1) as f32;

        let dark = self.rows.iter().map(|r| r.count_ones() as usize).sum();
        let mut hexagons = Vec::with_capacity(dark);
        for (row, &bits) in self.rows.iter().enumerate() {
            let y = hex_radius + row as f32 * pitch_y;
            let x0 = if row & 1 == 1 { pitch_x } else { pitch_x * 0.5 };
            let mut b = bits;
            while b != 0 {
                let col = b.trailing_zeros();
                b &= b - 1;
                hexagons.push((x0 + col as f32 * pitch_x, y));
            }
        }

        let center = (14.5 * pitch_x, hex_radius + 16.0 * pitch_y);
        // Six concentric diameters d1..d6: d1 = hexagon height, d6 = 9 module pitches.
        let d1 = 2.0 * hex_radius;
        let incr = (9.0 * pitch_x - d1) / 5.0;
        let r = |k: f32| (d1 + incr * k) / 2.0;
        let rings = vec![(r(5.0), r(4.0)), (r(3.0), r(2.0)), (r(1.0), r(0.0))];

        Geometry {
            width,
            height,
            hexagons,
            hex_radius,
            center,
            rings,
        }
    }
}

/// Encodes ZPL `^BD` field data. `mode` is 2..=6; `symbol_number` / `total_symbols` (1..=8) are
/// the structured append position (ignored when `total_symbols` is 1).
pub fn encode(
    data: &str,
    mode: u8,
    symbol_number: u8,
    total_symbols: u8,
) -> Result<MaxiCode, SymbolError> {
    if !(2..=6).contains(&mode) {
        return Err(SymbolError::InvalidData(format!(
            "MaxiCode mode {mode} (2 to 6 allowed)"
        )));
    }
    if !(1..=8).contains(&total_symbols) || !(1..=total_symbols).contains(&symbol_number) {
        return Err(SymbolError::InvalidData(format!(
            "MaxiCode structured append {symbol_number} of {total_symbols}"
        )));
    }

    // Field data as Latin-1 bytes.
    let mut buf = [0u8; MAX_SOURCE];
    let mut len = 0;
    for c in data.chars() {
        let v = c as u32;
        if v > 0xFF {
            return Err(SymbolError::InvalidCharacter(c));
        }
        if len == MAX_SOURCE {
            return Err(SymbolError::InvalidLength(data.chars().count()));
        }
        buf[len] = v as u8;
        len += 1;
    }
    let src = &buf[..len];

    let mut cw = [0u8; 144];
    // Secondary message source: the low priority message, or header + rest.
    let mut secondary = [0u8; MAX_SOURCE];
    let sec_len;
    if mode <= 3 {
        let p = parse_primary(src, mode)?;
        if mode == 2 {
            primary_mode2(&mut cw, p.postal, p.country, p.service)?;
        } else {
            primary_mode3(&mut cw, p.postal, p.country, p.service)?;
        }
        secondary[..p.header.len()].copy_from_slice(p.header);
        secondary[p.header.len()..p.header.len() + p.rest.len()].copy_from_slice(p.rest);
        sec_len = p.header.len() + p.rest.len();
    } else {
        cw[0] = mode;
        secondary[..len].copy_from_slice(src);
        sec_len = len;
    }

    let structured_append =
        (total_symbols > 1).then(|| (total_symbols - 1) | ((symbol_number - 1) << 3));
    encode_text(&mut cw, mode, &secondary[..sec_len], structured_append)?;

    // Primary message: 10 data + 10 error correction codewords.
    let (data_cw, ecc_cw) = cw.split_at_mut(10);
    rs_encode(&GEN_10, data_cw.iter().copied(), &mut ecc_cw[..10]);
    // Secondary message: two interleaved (even/odd) blocks.
    let (sec_data, sec_ecc) = if mode == 5 { (68, 28) } else { (84, 20) };
    let poly: &[u8] = if mode == 5 { &GEN_28 } else { &GEN_20 };
    for parity in 0..2 {
        let mut ecc = [0u8; 28];
        rs_encode(
            poly,
            (0..sec_data / 2).map(|j| cw[20 + parity + 2 * j]),
            &mut ecc[..sec_ecc],
        );
        for (j, &e) in ecc[..sec_ecc].iter().enumerate() {
            cw[20 + sec_data + parity + 2 * j] = e;
        }
    }

    Ok(place(&cw))
}

/// Upper bound for the field data length: 93 codewords hold at most 139 digits.
const MAX_SOURCE: usize = 160;

/// Header `[)>RS01GS` (followed by a two-digit format version) of an ANSI MH10.8.3 message.
const SCM_HEADER: &[u8; 7] = b"[)>\x1e01\x1d";

struct Primary<'a> {
    postal: &'a [u8],
    country: &'a [u8],
    service: &'a [u8],
    /// Prefix of the secondary message (`[)>RS01GSyy` in the header form, else empty).
    header: &'a [u8],
    /// Remainder of the secondary message.
    rest: &'a [u8],
}

fn parse_primary(src: &[u8], mode: u8) -> Result<Primary<'_>, SymbolError> {
    let bad = |what: &str| SymbolError::InvalidData(format!("MaxiCode mode {mode}: {what}"));
    if src.len() >= 9
        && src.starts_with(SCM_HEADER)
        && src[7].is_ascii_digit()
        && src[8].is_ascii_digit()
    {
        // [)>RS01GSyy postal GS country GS class GS rest
        let mut fields = [&src[..0]; 3];
        let mut pos = 9;
        for f in &mut fields {
            let end = src[pos..]
                .iter()
                .position(|&b| b == 0x1D)
                .map(|e| pos + e)
                .ok_or_else(|| {
                    bad("expected postal code, country and class fields after header")
                })?;
            *f = &src[pos..end];
            pos = end + 1;
        }
        return Ok(Primary {
            postal: fields[0],
            country: fields[1],
            service: fields[2],
            header: &src[..9],
            rest: &src[pos..],
        });
    }
    // <class:3><country:3><postal:9 or 6><lpm>
    let postal_len = if mode == 2 { 9 } else { 6 };
    if src.len() < 6 + postal_len {
        return Err(bad(&format!(
            "high priority message needs {} characters",
            6 + postal_len
        )));
    }
    Ok(Primary {
        service: &src[..3],
        country: &src[3..6],
        postal: &src[6..6 + postal_len],
        header: &[],
        rest: &src[6 + postal_len..],
    })
}

fn three_digits(field: &[u8], what: &str) -> Result<u32, SymbolError> {
    if field.len() != 3 || !field.iter().all(u8::is_ascii_digit) {
        return Err(SymbolError::InvalidData(format!(
            "MaxiCode {what} must be 3 digits"
        )));
    }
    Ok(field.iter().fold(0, |a, &d| a * 10 + (d - b'0') as u32))
}

/// Mode 2 primary message: numeric postal code (up to 9 digits).
fn primary_mode2(
    cw: &mut [u8; 144],
    postal: &[u8],
    country: &[u8],
    service: &[u8],
) -> Result<(), SymbolError> {
    let country = three_digits(country, "country code")?;
    let service = three_digits(service, "class of service")?;
    if postal.is_empty() || postal.len() > 9 || !postal.iter().all(u8::is_ascii_digit) {
        return Err(SymbolError::InvalidData(
            "MaxiCode mode 2 postal code must be 1 to 9 digits".into(),
        ));
    }
    let mut num = postal.iter().fold(0u32, |a, &d| a * 10 + (d - b'0') as u32);
    let mut plen = postal.len() as u32;
    if country == 840 && plen == 5 {
        // ISO/IEC 16023 B.1: unknown US "+4" is filled with zeros.
        num *= 10_000;
        plen = 9;
    }
    cw[0] = (((num & 0x03) << 4) | 2) as u8;
    cw[1] = ((num & 0xFC) >> 2) as u8;
    cw[2] = ((num & 0x3F00) >> 8) as u8;
    cw[3] = ((num & 0xFC000) >> 14) as u8;
    cw[4] = ((num & 0x3F0_0000) >> 20) as u8;
    cw[5] = (((num & 0x3C00_0000) >> 26) | ((plen & 0x03) << 4)) as u8;
    cw[6] = (((plen & 0x3C) >> 2) | ((country & 0x03) << 4)) as u8;
    primary_tail(cw, country, service);
    Ok(())
}

/// Mode 3 primary message: 6-character Code Set A postal code (space padded / truncated).
fn primary_mode3(
    cw: &mut [u8; 144],
    postal: &[u8],
    country: &[u8],
    service: &[u8],
) -> Result<(), SymbolError> {
    let country = three_digits(country, "country code")?;
    let service = three_digits(service, "class of service")?;
    let mut pc = [0u8; 6];
    for (i, v) in pc.iter_mut().enumerate() {
        let ch = postal.get(i).copied().unwrap_or(b' ').to_ascii_uppercase();
        if ch < b' ' || CODE_SET[ch as usize] & SET_A == 0 {
            return Err(SymbolError::InvalidCharacter(ch as char));
        }
        *v = SYMBOL_CHAR[ch as usize];
    }
    cw[0] = ((pc[5] & 0x03) << 4) | 3;
    for i in 1..6 {
        cw[i] = ((pc[5 - i] & 0x03) << 4) | ((pc[6 - i] & 0x3C) >> 2);
    }
    cw[6] = ((pc[0] & 0x3C) >> 2) | ((country as u8 & 0x03) << 4);
    primary_tail(cw, country, service);
    Ok(())
}

fn primary_tail(cw: &mut [u8; 144], country: u32, service: u32) {
    cw[7] = ((country & 0xFC) >> 2) as u8;
    cw[8] = (((country & 0x300) >> 8) | ((service & 0x0F) << 2)) as u8;
    cw[9] = ((service & 0x3F0) >> 4) as u8;
}

// ---------------------------------------------------------------------------------------------
// Code sets and minimal encodation
// ---------------------------------------------------------------------------------------------

const SET_A: u8 = 0x01;
const SET_B: u8 = 0x02;
const SET_E: u8 = 0x04;
const SET_C: u8 = 0x08;
const SET_D: u8 = 0x10;
const SHIFT: u8 = 0x20;
const SHIFT2_A: u8 = 0x40 | SET_A;
const SHIFT3_A: u8 = 0x80 | SET_A;

/// Code set states, in tie-break preference order (sets with a PAD character first).
const ST_A: usize = 0;
// B = 1
const ST_E: usize = 2;
const ST_C: usize = 3;
const ST_D: usize = 4;
const STATES: usize = 5;

/// Latch sequence `[later][prior]` and its length.
const LATCH_SEQ: [[[u8; 2]; STATES]; STATES] = [
    [[0, 0], [63, 0], [58, 0], [58, 0], [58, 0]],
    [[63, 0], [0, 0], [63, 0], [63, 0], [63, 0]],
    [[62, 62], [62, 62], [0, 0], [62, 62], [62, 62]],
    [[60, 60], [60, 60], [60, 60], [0, 0], [60, 60]],
    [[61, 61], [61, 61], [61, 61], [61, 61], [0, 0]],
];
const LATCH_LEN: [[u8; STATES]; STATES] = [
    [0, 1, 1, 1, 1],
    [1, 0, 1, 1, 1],
    [2, 2, 0, 2, 2],
    [2, 2, 2, 0, 2],
    [2, 2, 2, 2, 0],
];

#[derive(Clone, Copy)]
struct Op {
    /// Code set bits, plus SHIFT / 2- / 3-shift flags; 0 = numeric compression.
    op: u8,
    /// Characters consumed.
    intake: u8,
    /// Codewords produced.
    output: u8,
}

/// Length of a state that cannot encode the character (larger than any real length, small
/// enough not to overflow when latch and op costs are added).
const UNREACHABLE: u16 = 9999;
const OP_DIGITS: u8 = 0;
#[rustfmt::skip]
const OPS: [Op; 13] = [
    Op { op: 0, intake: 9, output: 6 },               // 0 numeric shift (NS)
    Op { op: SET_A, intake: 1, output: 1 },           // 1
    Op { op: SET_B, intake: 1, output: 1 },           // 2
    Op { op: SET_E, intake: 1, output: 1 },           // 3
    Op { op: SET_C, intake: 1, output: 1 },           // 4
    Op { op: SET_D, intake: 1, output: 1 },           // 5
    Op { op: SHIFT | SET_A, intake: 1, output: 2 },   // 6
    Op { op: SHIFT2_A, intake: 2, output: 3 },        // 7
    Op { op: SHIFT3_A, intake: 3, output: 4 },        // 8
    Op { op: SHIFT | SET_B, intake: 1, output: 2 },   // 9
    Op { op: SHIFT | SET_E, intake: 1, output: 2 },   // 10
    Op { op: SHIFT | SET_C, intake: 1, output: 2 },   // 11
    Op { op: SHIFT | SET_D, intake: 1, output: 2 },   // 12
];

/// Ops available in each state (besides numeric compression).
const STATE_OPS: [&[u8]; STATES] = [
    &[1, 9, 10, 11, 12],
    &[2, 6, 7, 8, 10, 11, 12],
    &[3, 11, 12],
    &[4, 10, 12],
    &[5, 10, 11],
];

#[inline]
fn can_encode(op: u8, ch: u8, num_a: usize) -> bool {
    match op {
        SHIFT2_A => num_a >= 2,
        SHIFT3_A => num_a >= 3,
        _ => CODE_SET[ch as usize] & op != 0,
    }
}

/// Symbol value of `ch` in the code set of `op`.
#[inline]
fn symbol_char(op: u8, ch: u8) -> u8 {
    let sets = CODE_SET[ch as usize];
    if sets == op & 0x1F || op & SET_A != 0 {
        return SYMBOL_CHAR[ch as usize];
    }
    if op & SET_B != 0 {
        match ch {
            b' ' => return 47,
            b',' => return 48,
            b'.' => return 49,
            b'/' => return 50,
            b':' => return 51,
            _ => {}
        }
    }
    if op & SET_E != 0 && (28..=30).contains(&ch) {
        return ch + 4; // FS GS RS
    }
    if ch == b' ' { 59 } else { ch } // SP CR FS GS RS in the other sets
}

/// Encodes the secondary message (and, for modes 4-6, the 9 primary data codewords) into `cw`,
/// padded to capacity.
fn encode_text(
    cw: &mut [u8; 144],
    mode: u8,
    src: &[u8],
    structured_append: Option<u8>,
) -> Result<(), SymbolError> {
    let n = src.len();
    // Modes 4-6 start at 11 so that the first 9 codewords can be moved into the primary (1..10).
    let mut cp = if mode > 3 { 11 } else { 20 };
    let max_len = if mode == 5 { 88 } else { 104 };
    if let Some(sa) = structured_append {
        cw[cp] = 33; // PAD
        cw[cp + 1] = sa;
        cp += 2;
    }

    // best_len[p][s]: fewest codewords for the first p characters ending in state s.
    // best_org[p][s]: the state to be in after p characters before latching to s.
    let mut best_len = [[0u16; STATES]; MAX_SOURCE + 1];
    let mut best_org = [[0u8; STATES]; MAX_SOURCE + 1];
    let mut path_op = [[0u8; STATES]; MAX_SOURCE + 1];
    let mut prior = [[0u8; STATES]; MAX_SOURCE + 1];

    let mut digits = 0usize;
    let mut num_a = 0usize;
    for (i, &ch) in src.iter().enumerate() {
        let p = i + 1;
        digits = if ch.is_ascii_digit() { digits + 1 } else { 0 };
        num_a = if CODE_SET[ch as usize] & SET_A != 0 {
            num_a + 1
        } else {
            0
        };
        for state in 0..STATES {
            let latch = &LATCH_LEN[state];
            let mut min = UNREACHABLE;
            let mut consider = |op_idx: u8| {
                let op = OPS[op_idx as usize];
                let from = p - op.intake as usize;
                let org = best_org[from][state] as usize;
                let len = best_len[from][org] + latch[org] as u16 + op.output as u16;
                if len < min {
                    min = len;
                    path_op[p][state] = op_idx;
                    prior[p][state] = org as u8;
                }
            };
            if digits >= 9 {
                consider(OP_DIGITS); // nothing beats numeric compression
            } else {
                for &op_idx in STATE_OPS[state] {
                    if can_encode(OPS[op_idx as usize].op, ch, num_a) {
                        consider(op_idx);
                    }
                }
            }
            best_len[p][state] = min;
        }
        for state in 0..STATES {
            let latch = &LATCH_LEN[state];
            let mut org = 0;
            let mut org_len = best_len[p][0] + latch[0] as u16;
            for o in 1..STATES {
                let l = best_len[p][o] + latch[o] as u16;
                if l < org_len {
                    org = o;
                    org_len = l;
                }
            }
            best_org[p][state] = org as u8;
        }
    }

    let mut min_state = ST_A;
    for state in 1..STATES {
        if best_len[n][state] < best_len[n][min_state] {
            min_state = state;
        }
    }
    let min_len = best_len[n][min_state] as usize;
    if cp + min_len > max_len {
        return Err(SymbolError::InvalidLength(n));
    }

    // Walk the best path backwards, writing codewords from the end.
    let top = cp + min_len;
    let mut ci = top;
    let mut state = min_state;
    let mut p = n;
    while p > 0 {
        let op = OPS[path_op[p][state] as usize];
        let pcs = prior[p][state] as usize;
        p -= op.intake as usize;
        let s = &src[p..p + op.intake as usize];
        ci -= op.output as usize;
        let out = &mut cw[ci..ci + op.output as usize];
        match op.op {
            OP_DIGITS => {
                let v = s.iter().fold(0u32, |a, &d| a * 10 + (d - b'0') as u32);
                out[0] = 31; // NS
                for (k, o) in out[1..].iter_mut().enumerate() {
                    *o = ((v >> (6 * (4 - k))) & 0x3F) as u8;
                }
            }
            SHIFT2_A => {
                out[0] = 56;
                out[1] = symbol_char(op.op, s[0]);
                out[2] = symbol_char(op.op, s[1]);
            }
            SHIFT3_A => {
                out[0] = 57;
                out[1] = symbol_char(op.op, s[0]);
                out[2] = symbol_char(op.op, s[1]);
                out[3] = symbol_char(op.op, s[2]);
            }
            o if o & SHIFT != 0 => {
                out[0] = match o & 0x1F {
                    SET_C => 60,
                    SET_D => 61,
                    SET_E => 62,
                    _ => 59, // Shift A (from B) / Shift B (from A)
                };
                out[1] = symbol_char(o, s[0]);
            }
            o => out[0] = symbol_char(o, s[0]),
        }
        if state != pcs {
            let l = LATCH_LEN[state][pcs] as usize;
            ci -= l;
            cw[ci..ci + l].copy_from_slice(&LATCH_SEQ[state][pcs][..l]);
            state = pcs;
        }
    }
    debug_assert_eq!(ci, cp);

    let mut cp = top;
    if cp < max_len && (min_state == ST_C || min_state == ST_D) {
        cw[cp] = 58; // Latch A for padding
        cp += 1;
    }
    let pad = if min_state == ST_E { 28 } else { 33 };
    cw[cp..max_len].fill(pad);

    if mode > 3 {
        cw.copy_within(11..20, 1);
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------
// Reed-Solomon over GF(64), x^6 + x + 1
// ---------------------------------------------------------------------------------------------

const fn gf_tables() -> ([u8; 126], [u8; 64]) {
    let mut exp = [0u8; 126];
    let mut log = [0u8; 64];
    let mut x: u16 = 1;
    let mut i = 0;
    while i < 63 {
        exp[i] = x as u8;
        exp[i + 63] = x as u8;
        log[x as usize] = i as u8;
        x <<= 1;
        if x & 0x40 != 0 {
            x ^= 0x43;
        }
        i += 1;
    }
    (exp, log)
}

const GF: ([u8; 126], [u8; 64]) = gf_tables();
const GF_EXP: [u8; 126] = GF.0;
const GF_LOG: [u8; 64] = GF.1;

#[inline]
const fn gf_mul(a: u8, b: u8) -> u8 {
    if a == 0 || b == 0 {
        0
    } else {
        GF_EXP[GF_LOG[a as usize] as usize + GF_LOG[b as usize] as usize]
    }
}

/// Generator polynomial prod_{i=1..N} (x - a^i), coefficients from x^N down to x^0.
const fn generator<const L: usize>() -> [u8; L] {
    let n = L - 1;
    // Ascending coefficients.
    let mut asc = [0u8; L];
    asc[0] = 1;
    let mut i = 1;
    while i <= n {
        let a = GF_EXP[i];
        let mut k = i;
        while k > 0 {
            asc[k] = asc[k - 1] ^ gf_mul(asc[k], a);
            k -= 1;
        }
        asc[0] = gf_mul(asc[0], a);
        i += 1;
    }
    let mut desc = [0u8; L];
    let mut j = 0;
    while j < L {
        desc[j] = asc[n - j];
        j += 1;
    }
    desc
}

const GEN_10: [u8; 11] = generator::<11>();
const GEN_20: [u8; 21] = generator::<21>();
const GEN_28: [u8; 29] = generator::<29>();

/// Systematic RS: the remainder of data(x) * x^n / poly(x), highest degree first, into `ecc`.
fn rs_encode(poly: &[u8], data: impl Iterator<Item = u8>, ecc: &mut [u8]) {
    let n = ecc.len();
    debug_assert_eq!(poly.len(), n + 1);
    ecc.fill(0);
    for d in data {
        let fb = d ^ ecc[0];
        if fb == 0 {
            ecc.copy_within(1.., 0);
            ecc[n - 1] = 0;
        } else {
            let lf = GF_LOG[fb as usize] as usize;
            let term = |g: u8| {
                if g == 0 {
                    0
                } else {
                    GF_EXP[lf + GF_LOG[g as usize] as usize]
                }
            };
            for j in 0..n - 1 {
                ecc[j] = ecc[j + 1] ^ term(poly[j + 1]);
            }
            ecc[n - 1] = term(poly[n]);
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Module placement
// ---------------------------------------------------------------------------------------------

/// Orientation modules that are always dark, as (row, column).
const ORIENTATION_DARK: [(usize, u32); 13] = [
    (0, 28),
    (0, 29),
    (9, 10),
    (9, 11),
    (10, 11),
    (15, 7),
    (16, 8),
    (16, 20),
    (17, 20),
    (22, 10),
    (23, 10),
    (22, 17),
    (23, 17),
];

fn place(cw: &[u8; 144]) -> MaxiCode {
    let mut rows = [0u32; ROWS as usize];
    for (row, bits) in rows.iter_mut().enumerate() {
        let line = &GRID[row * COLUMNS as usize..(row + 1) * COLUMNS as usize];
        for (col, &g) in line.iter().enumerate() {
            if g != 0 {
                let b = (g - 1) as usize;
                if (cw[b / 6] >> (5 - b % 6)) & 1 != 0 {
                    *bits |= 1 << col;
                }
            }
        }
    }
    for &(row, col) in &ORIENTATION_DARK {
        rows[row] |= 1 << col;
    }
    MaxiCode { rows }
}

/// ISO/IEC 16023 Figure 5 module sequence: for each module (row-major, 33 rows x 30 columns)
/// the 1-based bit number (codeword * 6 + bit, MSB first), 0 = not a data module.
#[rustfmt::skip]
const GRID: [u16; 990] = [
    122, 121, 128, 127, 134, 133, 140, 139, 146, 145, 152, 151, 158, 157, 164, 163, 170, 169, 176, 175, 182, 181, 188, 187, 194, 193, 200, 199,   0,   0,
    124, 123, 130, 129, 136, 135, 142, 141, 148, 147, 154, 153, 160, 159, 166, 165, 172, 171, 178, 177, 184, 183, 190, 189, 196, 195, 202, 201, 817,   0,
    126, 125, 132, 131, 138, 137, 144, 143, 150, 149, 156, 155, 162, 161, 168, 167, 174, 173, 180, 179, 186, 185, 192, 191, 198, 197, 204, 203, 819, 818,
    284, 283, 278, 277, 272, 271, 266, 265, 260, 259, 254, 253, 248, 247, 242, 241, 236, 235, 230, 229, 224, 223, 218, 217, 212, 211, 206, 205, 820,   0,
    286, 285, 280, 279, 274, 273, 268, 267, 262, 261, 256, 255, 250, 249, 244, 243, 238, 237, 232, 231, 226, 225, 220, 219, 214, 213, 208, 207, 822, 821,
    288, 287, 282, 281, 276, 275, 270, 269, 264, 263, 258, 257, 252, 251, 246, 245, 240, 239, 234, 233, 228, 227, 222, 221, 216, 215, 210, 209, 823,   0,
    290, 289, 296, 295, 302, 301, 308, 307, 314, 313, 320, 319, 326, 325, 332, 331, 338, 337, 344, 343, 350, 349, 356, 355, 362, 361, 368, 367, 825, 824,
    292, 291, 298, 297, 304, 303, 310, 309, 316, 315, 322, 321, 328, 327, 334, 333, 340, 339, 346, 345, 352, 351, 358, 357, 364, 363, 370, 369, 826,   0,
    294, 293, 300, 299, 306, 305, 312, 311, 318, 317, 324, 323, 330, 329, 336, 335, 342, 341, 348, 347, 354, 353, 360, 359, 366, 365, 372, 371, 828, 827,
    410, 409, 404, 403, 398, 397, 392, 391,  80,  79,   0,   0,  14,  13,  38,  37,   3,   0,  45,  44, 110, 109, 386, 385, 380, 379, 374, 373, 829,   0,
    412, 411, 406, 405, 400, 399, 394, 393,  82,  81,  41,   0,  16,  15,  40,  39,   4,   0,   0,  46, 112, 111, 388, 387, 382, 381, 376, 375, 831, 830,
    414, 413, 408, 407, 402, 401, 396, 395,  84,  83,  42,   0,   0,   0,   0,   0,   6,   5,  48,  47, 114, 113, 390, 389, 384, 383, 378, 377, 832,   0,
    416, 415, 422, 421, 428, 427, 104, 103,  56,  55,  17,   0,   0,   0,   0,   0,   0,   0,  21,  20,  86,  85, 434, 433, 440, 439, 446, 445, 834, 833,
    418, 417, 424, 423, 430, 429, 106, 105,  58,  57,   0,   0,   0,   0,   0,   0,   0,   0,  23,  22,  88,  87, 436, 435, 442, 441, 448, 447, 835,   0,
    420, 419, 426, 425, 432, 431, 108, 107,  60,  59,   0,   0,   0,   0,   0,   0,   0,   0,   0,  24,  90,  89, 438, 437, 444, 443, 450, 449, 837, 836,
    482, 481, 476, 475, 470, 469,  49,   0,  31,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   1,  54,  53, 464, 463, 458, 457, 452, 451, 838,   0,
    484, 483, 478, 477, 472, 471,  50,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0, 466, 465, 460, 459, 454, 453, 840, 839,
    486, 485, 480, 479, 474, 473,  52,  51,  32,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   2,   0,  43, 468, 467, 462, 461, 456, 455, 841,   0,
    488, 487, 494, 493, 500, 499,  98,  97,  62,  61,   0,   0,   0,   0,   0,   0,   0,   0,   0,  27,  92,  91, 506, 505, 512, 511, 518, 517, 843, 842,
    490, 489, 496, 495, 502, 501, 100,  99,  64,  63,   0,   0,   0,   0,   0,   0,   0,   0,  29,  28,  94,  93, 508, 507, 514, 513, 520, 519, 844,   0,
    492, 491, 498, 497, 504, 503, 102, 101,  66,  65,  18,   0,   0,   0,   0,   0,   0,   0,  19,  30,  96,  95, 510, 509, 516, 515, 522, 521, 846, 845,
    560, 559, 554, 553, 548, 547, 542, 541,  74,  73,  33,   0,   0,   0,   0,   0,   0,  11,  68,  67, 116, 115, 536, 535, 530, 529, 524, 523, 847,   0,
    562, 561, 556, 555, 550, 549, 544, 543,  76,  75,   0,   0,   8,   7,  36,  35,  12,   0,  70,  69, 118, 117, 538, 537, 532, 531, 526, 525, 849, 848,
    564, 563, 558, 557, 552, 551, 546, 545,  78,  77,   0,  34,  10,   9,  26,  25,   0,   0,  72,  71, 120, 119, 540, 539, 534, 533, 528, 527, 850,   0,
    566, 565, 572, 571, 578, 577, 584, 583, 590, 589, 596, 595, 602, 601, 608, 607, 614, 613, 620, 619, 626, 625, 632, 631, 638, 637, 644, 643, 852, 851,
    568, 567, 574, 573, 580, 579, 586, 585, 592, 591, 598, 597, 604, 603, 610, 609, 616, 615, 622, 621, 628, 627, 634, 633, 640, 639, 646, 645, 853,   0,
    570, 569, 576, 575, 582, 581, 588, 587, 594, 593, 600, 599, 606, 605, 612, 611, 618, 617, 624, 623, 630, 629, 636, 635, 642, 641, 648, 647, 855, 854,
    728, 727, 722, 721, 716, 715, 710, 709, 704, 703, 698, 697, 692, 691, 686, 685, 680, 679, 674, 673, 668, 667, 662, 661, 656, 655, 650, 649, 856,   0,
    730, 729, 724, 723, 718, 717, 712, 711, 706, 705, 700, 699, 694, 693, 688, 687, 682, 681, 676, 675, 670, 669, 664, 663, 658, 657, 652, 651, 858, 857,
    732, 731, 726, 725, 720, 719, 714, 713, 708, 707, 702, 701, 696, 695, 690, 689, 684, 683, 678, 677, 672, 671, 666, 665, 660, 659, 654, 653, 859,   0,
    734, 733, 740, 739, 746, 745, 752, 751, 758, 757, 764, 763, 770, 769, 776, 775, 782, 781, 788, 787, 794, 793, 800, 799, 806, 805, 812, 811, 861, 860,
    736, 735, 742, 741, 748, 747, 754, 753, 760, 759, 766, 765, 772, 771, 778, 777, 784, 783, 790, 789, 796, 795, 802, 801, 808, 807, 814, 813, 862,   0,
    738, 737, 744, 743, 750, 749, 756, 755, 762, 761, 768, 767, 774, 773, 780, 779, 786, 785, 792, 791, 798, 797, 804, 803, 810, 809, 816, 815, 864, 863,
];

/// Code Sets a byte belongs to (bit flags A 0x01, B 0x02, E 0x04, C 0x08, D 0x10).
#[rustfmt::skip]
const CODE_SET: [u8; 256] = [
    0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x05, 0x04, 0x04,
    0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x1F, 0x1F, 0x1F, 0x04,
    0x1F, 0x02, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x03, 0x01, 0x03, 0x03,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x03, 0x02, 0x02, 0x02, 0x02, 0x02,
    0x02, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x02, 0x02, 0x02, 0x02, 0x02,
    0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
    0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
    0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10,
    0x10, 0x10, 0x10, 0x10, 0x10, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
    0x04, 0x10, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x10, 0x04, 0x08, 0x10, 0x08, 0x04, 0x04, 0x10,
    0x10, 0x08, 0x08, 0x08, 0x10, 0x08, 0x04, 0x10, 0x10, 0x08, 0x08, 0x10, 0x08, 0x08, 0x08, 0x10,
    0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08,
    0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08,
    0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10,
    0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10,
];

/// Symbol value of a byte (Code Set A value for characters in several sets).
#[rustfmt::skip]
const SYMBOL_CHAR: [u8; 256] = [
     0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12,  0, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 30, 28, 29, 30, 35,
    32, 53, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 37, 38, 39, 40, 41,
    52,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 42, 43, 44, 45, 46,
     0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 32, 54, 34, 35, 36,
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 47, 48, 49, 50, 51, 52,
    53, 54, 55, 56, 57, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 36,
    37, 37, 38, 39, 40, 41, 42, 43, 38, 44, 37, 39, 38, 45, 46, 40,
    41, 39, 40, 41, 42, 42, 47, 43, 44, 43, 44, 45, 45, 46, 47, 46,
     0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 32, 33, 34, 35, 36,
     0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 32, 33, 34, 35, 36,
];

#[cfg(test)]
mod tests {
    use super::*;

    // Reference symbols generated with zint (backend/maxicode.c, the reference encoder bundled with
    // zxing-cpp) for the equivalent primary message / data; rows as bit masks (bit = column).
    // zint: mode 4, primary ''
    const MODE4_HELLO: (&str, u8, u8, u8, [u32; 33]) = (
        "Hello World",
        4,
        1,
        1,
        [
            0x3aaaaa04, 0x10000073, 0x3555550b, 0x1aaaaaaa, 0x20000000, 0x05555555, 0x3aaaaaaa,
            0x10000000, 0x05555555, 0x0a843caa, 0x2009fd00, 0x056c0455, 0x2aa005ea, 0x001801c0,
            0x05680355, 0x1ab000ea, 0x00100140, 0x05700095, 0x3a9801aa, 0x10080040, 0x05600595,
            0x1a9005aa, 0x10160600, 0x05522d55, 0x1daaaaaa, 0x08d00000, 0x22055555, 0x148a632b,
            0x24e552cc, 0x0d70b60e, 0x071d1001, 0x07e1fce4, 0x05f800b9,
        ],
    );
    // zint: mode 2, primary '152382802840001'
    const MODE2_UPS: (&str, u8, u8, u8, [u32; 33]) = (
        "001840152382802[)>\u{1e}01\u{1d}961Z00004951\u{1d}UPSN\u{1d}06X610\u{1d}159\u{1d}1234567\u{1d}1/1\u{1d}\u{1d}Y\u{1d}634 ALPHA DR\u{1d}PITTSBURGH\u{1d}PA\u{1e}\u{4}",
        2,
        1,
        1,
        [
            0x3dfdf6eb, 0x121b0eaa, 0x226548db, 0x1fffd547, 0x3019343c, 0x000554e4, 0x1ff7f7f7,
            0x140e4c19, 0x23955462, 0x0fd82c7b, 0x25703ccc, 0x16fa015d, 0x3f680015, 0x11d8006f,
            0x2e600095, 0x1e380084, 0x14100132, 0x10500140, 0x157003c8, 0x03100350, 0x01b805c1,
            0x016c0045, 0x29669704, 0x040654e6, 0x17aa3450, 0x15b07c39, 0x2c052913, 0x0f5cb620,
            0x16f094ba, 0x0bb5573b, 0x32e8a8f6, 0x1a1e1978, 0x33e03148,
        ],
    );
    // zint: mode 2, primary '152382802840001'
    const MODE2_HEADER: (&str, u8, u8, u8, [u32; 33]) = (
        "[)>\u{1e}01\u{1d}96152382802\u{1d}840\u{1d}001\u{1d}1Z00004951\u{1d}UPSN\u{1d}\u{1e}\u{4}",
        2,
        1,
        1,
        [
            0x3dfdf6eb, 0x121b0eaa, 0x126548db, 0x0fffd545, 0x0019343f, 0x100554e6, 0x0aaaaaa3,
            0x10000007, 0x35555552, 0x1a982caa, 0x20303c00, 0x057a0155, 0x3aa8002a, 0x10180040,
            0x25600095, 0x1ab800aa, 0x20100100, 0x15500155, 0x3ab003ea, 0x10100340, 0x257805d5,
            0x1aac00aa, 0x20269700, 0x05465455, 0x132aaaaa, 0x11700000, 0x10b55555, 0x1899dff7,
            0x3ecbe351, 0x117d0067, 0x0d3d7f22, 0x0e14292e, 0x198d7070,
        ],
    );
    // zint: mode 3, primary 'EC1A1A826066'
    const MODE3_UK: (&str, u8, u8, u8, [u32; 33]) = (
        "066826EC1A1A1Z12345678\u{1d}UPSN\u{1d}SHIP TO LONDON",
        3,
        1,
        1,
        [
            0x355ffff7, 0x10795408, 0x2c539399, 0x11419200, 0x3f281cfd, 0x09c40c38, 0x2aaaaaa0,
            0x1000000f, 0x1555555b, 0x0aa49caa, 0x30083a00, 0x056b0655, 0x2aa8002a, 0x10200100,
            0x25600055, 0x1ab000aa, 0x20100100, 0x15580195, 0x3aa803ea, 0x100801c0, 0x054003d5,
            0x0a8801aa, 0x101e1700, 0x155ec555, 0x371aaaaa, 0x1f400000, 0x21955555, 0x0c5f4bef,
            0x0485e7b3, 0x1b0cb829, 0x2f436165, 0x1926098e, 0x33ab1f52,
        ],
    );
    // zint: mode 3, primary 'B1050 056012'
    const MODE3_HEADER: (&str, u8, u8, u8, [u32; 33]) = (
        "[)>\u{1e}01\u{1d}96B1050\u{1d}056\u{1d}012\u{1d}abc 123",
        3,
        1,
        1,
        [
            0x30fdf6eb, 0x10db0eaa, 0x39e548db, 0x03bfaaaa, 0x13000000, 0x1f1b5555, 0x1aaaaaaa,
            0x00000000, 0x35555555, 0x0aa41daa, 0x20183b00, 0x055b0255, 0x3a8400aa, 0x10180000,
            0x05580015, 0x1a8001ea, 0x30100140, 0x05500015, 0x2a9801ea, 0x10080380, 0x154002d5,
            0x0a9407aa, 0x202e0600, 0x05566e55, 0x326aaaaa, 0x10100000, 0x2aa55555, 0x1b6c1e4f,
            0x095ad873, 0x1c41655b, 0x1bd3939b, 0x05ec3786, 0x1f8c3e5e,
        ],
    );
    // zint: mode 5, primary ''
    const MODE5_MIXED: (&str, u8, u8, u8, [u32; 33]) = (
        "1234567890123 Mixed lower UPPER \u{e9}\u{e8}\u{c0}\u{c9} \u{1}\u{2} ;<=>",
        5,
        1,
        1,
        [
            0x344204c8, 0x117f5af2, 0x19f311d3, 0x1e546ccc, 0x1c410eec, 0x0c418540, 0x1aab8ce3,
            0x12570ccb, 0x34e72984, 0x0ab8beaa, 0x00190800, 0x05490655, 0x3a9804ea, 0x10140080,
            0x25780355, 0x1a9000aa, 0x00100140, 0x05700155, 0x2aa000aa, 0x00280180, 0x25440255,
            0x0a6007e3, 0x21e7f4ee, 0x05facdca, 0x2d812514, 0x1f368757, 0x23a85bad, 0x112a9d9e,
            0x2f2e2c8f, 0x07b31d93, 0x1996f83c, 0x13bdd07e, 0x3aaa10f7,
        ],
    );
    // zint: mode 6, primary ''
    const MODE6_PROG: (&str, u8, u8, u8, [u32; 33]) = (
        "READER PROGRAM 42",
        6,
        1,
        1,
        [
            0x3aaaf810, 0x00001307, 0x3555816f, 0x1aaaaaaa, 0x20000000, 0x05555555, 0x1aaaaaaa,
            0x00000000, 0x35555555, 0x1a804faa, 0x20311e00, 0x15620355, 0x1ab0016a, 0x10100080,
            0x25480215, 0x1a8000aa, 0x30100140, 0x05700015, 0x3a90012a, 0x00380280, 0x055006d5,
            0x1aba02aa, 0x30125600, 0x15620d55, 0x0ffaaaaa, 0x10000000, 0x16055555, 0x1d515d39,
            0x35afef66, 0x11110a2e, 0x11737374, 0x10d26dfe, 0x004c32bd,
        ],
    );
    // zint: mode 4, primary '', structapp 2/3
    const MODE4_SA: (&str, u8, u8, u8, [u32; 33]) = (
        "PART TWO OF THREE",
        4,
        2,
        3,
        [
            0x3aa04608, 0x00052473, 0x055580b3, 0x1aaaaaaa, 0x00000000, 0x15555555, 0x1aaaaaaa,
            0x00000000, 0x15555555, 0x1ab04caa, 0x20216a00, 0x15600055, 0x1ab8052a, 0x10300140,
            0x35500315, 0x1a8000aa, 0x00100140, 0x05700155, 0x0a8002aa, 0x00300240, 0x157802d5,
            0x0aa801aa, 0x200fa500, 0x15460555, 0x1e2aaaaa, 0x10800000, 0x22c55555, 0x0233d7ca,
            0x338b6c93, 0x19893878, 0x144abfa6, 0x164548d9, 0x03c56c71,
        ],
    );
    // zint: mode 4, primary ''
    const MODE4_DIGITS: (&str, u8, u8, u8, [u32; 33]) = (
        "123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678",
        4,
        1,
        1,
        [
            0x316f1da5, 0x1703f497, 0x1fbe74a3, 0x17d4807b, 0x175cd7de, 0x181ce6d7, 0x2fc918d3,
            0x0b4f0077, 0x055f3f7e, 0x16403cdd, 0x0ef1cc13, 0x1c6002fb, 0x0408051e, 0x0e340114,
            0x2fe80059, 0x048000f5, 0x1b100147, 0x0e300103, 0x27580169, 0x1d1c0165, 0x0d2c02a8,
            0x13f80794, 0x2f0ff70d, 0x06fe45ef, 0x2a40217d, 0x08bd735d, 0x11c9b342, 0x1ab802c4,
            0x078fcec6, 0x1425cf0a, 0x23ec59ca, 0x12dbca74, 0x2737f0dc,
        ],
    );
    // zint: mode 4, primary ''
    const MODE4_SHIFTS: (&str, u8, u8, u8, [u32; 33]) = (
        "aBc\u{e0}\u{a1}\u{ff}\u{1b}AAA bb,./:CcCC\u{c1}\u{c2}\u{c3}\u{c4}\u{e1}\u{e2}\u{e3}\u{e4}\u{7f}\u{80}",
        4,
        1,
        1,
        [
            0x3a33807b, 0x0f2200f7, 0x08bb15a1, 0x1b303c03, 0x0e203c07, 0x1effc1b1, 0x0aabec03,
            0x1000dd03, 0x05540ce5, 0x1ab0cfaa, 0x1021cb00, 0x05400555, 0x0aa002aa, 0x10340140,
            0x15700115, 0x0a9000ea, 0x20100140, 0x055000d5, 0x0a98016a, 0x001401c0, 0x35780695,
            0x0aba02aa, 0x201ff600, 0x157ae655, 0x2b7aaaaa, 0x17600000, 0x04d55555, 0x029188cc,
            0x36a0ad73, 0x00dbfa08, 0x3b8dab03, 0x1508530d, 0x2187b348,
        ],
    );

    const REFERENCES: [(&str, u8, u8, u8, [u32; 33]); 10] = [
        MODE4_HELLO,
        MODE2_UPS,
        MODE2_HEADER,
        MODE3_UK,
        MODE3_HEADER,
        MODE5_MIXED,
        MODE6_PROG,
        MODE4_SA,
        MODE4_DIGITS,
        MODE4_SHIFTS,
    ];

    fn render(m: &MaxiCode) -> String {
        let mut s = String::new();
        for row in 0..ROWS {
            if row & 1 == 1 {
                s.push(' ');
            }
            for col in 0..COLUMNS {
                s.push(if m.get(col, row) { 'X' } else { '.' });
                s.push(' ');
            }
            s.push('\n');
        }
        s
    }

    #[test]
    fn matches_zint_reference_symbols() {
        for (i, (data, mode, n, t, expected)) in REFERENCES.iter().enumerate() {
            let m = encode(data, *mode, *n, *t).unwrap();
            let want = MaxiCode { rows: *expected };
            assert_eq!(
                m,
                want,
                "reference {i} ({data:?})\ngot:\n{}\nwant:\n{}",
                render(&m),
                render(&want)
            );
        }
    }

    #[test]
    fn primary_message_fields() {
        // Mode 2 primary: postal 152382802 (9 digits), country 840, class 001.
        let mut cw = [0u8; 144];
        primary_mode2(&mut cw, b"152382802", b"840", b"001").unwrap();
        let num = 152_382_802u32;
        assert_eq!(cw[0] & 0x0F, 2);
        let decoded = ((cw[0] as u32 >> 4) & 3)
            | (cw[1] as u32) << 2
            | (cw[2] as u32) << 8
            | (cw[3] as u32) << 14
            | (cw[4] as u32) << 20
            | ((cw[5] as u32) & 0x0F) << 26;
        assert_eq!(decoded, num);
        let len = ((cw[5] as u32) >> 4) | ((cw[6] as u32) & 0x0F) << 2;
        assert_eq!(len, 9);
        let country = ((cw[6] as u32) >> 4) | (cw[7] as u32) << 2 | ((cw[8] as u32) & 3) << 8;
        assert_eq!(country, 840);
        let service = ((cw[8] as u32) >> 2) | (cw[9] as u32) << 4;
        assert_eq!(service, 1);
        // US 5-digit ZIP is extended with "0000".
        let mut cw5 = [0u8; 144];
        primary_mode2(&mut cw5, b"15238", b"840", b"001").unwrap();
        let mut cw9 = [0u8; 144];
        primary_mode2(&mut cw9, b"152380000", b"840", b"001").unwrap();
        assert_eq!(cw5[..10], cw9[..10]);
    }

    #[test]
    fn reed_solomon_codewords_are_valid() {
        // Evaluate the codeword polynomial at the generator roots a^1..a^n: all zero.
        fn syndromes_zero(cws: &[u8], n: usize) -> bool {
            (1..=n).all(|i| cws.iter().fold(0u8, |acc, &c| gf_mul(acc, GF_EXP[i]) ^ c) == 0)
        }
        let mut cw = [0u8; 144];
        for (i, c) in cw.iter_mut().enumerate().take(10) {
            *c = (i * 7 % 64) as u8;
        }
        let (d, e) = cw.split_at_mut(10);
        rs_encode(&GEN_10, d.iter().copied(), &mut e[..10]);
        assert!(syndromes_zero(&cw[..20], 10));
        let data: Vec<u8> = (0..42).map(|i| (i * 13 % 64) as u8).collect();
        let mut ecc = [0u8; 20];
        rs_encode(&GEN_20, data.iter().copied(), &mut ecc);
        let all: Vec<u8> = data.iter().chain(ecc.iter()).copied().collect();
        assert!(syndromes_zero(&all, 20));
        let data: Vec<u8> = (0..34).map(|i| (i * 29 % 64) as u8).collect();
        let mut ecc = [0u8; 28];
        rs_encode(&GEN_28, data.iter().copied(), &mut ecc);
        let all: Vec<u8> = data.iter().chain(ecc.iter()).copied().collect();
        assert!(syndromes_zero(&all, 28));
    }

    #[test]
    fn orientation_and_finder_area() {
        let m = encode("", 4, 1, 1).unwrap();
        for &(row, col) in &ORIENTATION_DARK {
            assert!(m.get(col, row as u32));
        }
        // Modules covered by the bullseye are never dark (except orientation modules).
        for row in 0..ROWS {
            for col in 0..COLUMNS {
                let g = GRID[(row * COLUMNS + col) as usize];
                let orient = ORIENTATION_DARK.contains(&(row as usize, col));
                if g == 0 && !orient {
                    assert!(!m.get(col, row), "({col},{row})");
                }
            }
        }
        // Every data bit appears exactly once in the placement table.
        let mut seen = [false; 864];
        for &g in GRID.iter().filter(|&&g| g != 0) {
            assert!(!seen[g as usize - 1]);
            seen[g as usize - 1] = true;
        }
        assert!(seen.iter().all(|&s| s));
        assert!(!m.get(30, 0) && !m.get(0, 33));
    }

    #[test]
    fn errors() {
        assert!(matches!(
            encode("x", 1, 1, 1),
            Err(SymbolError::InvalidData(_))
        ));
        assert!(matches!(
            encode("x", 7, 1, 1),
            Err(SymbolError::InvalidData(_))
        ));
        assert!(matches!(
            encode("x", 4, 3, 2),
            Err(SymbolError::InvalidData(_))
        ));
        assert!(matches!(
            encode("x", 4, 1, 9),
            Err(SymbolError::InvalidData(_))
        ));
        assert!(matches!(
            encode("x", 4, 0, 1),
            Err(SymbolError::InvalidData(_))
        ));
        assert_eq!(
            encode("\u{20ac}", 4, 1, 1),
            Err(SymbolError::InvalidCharacter('\u{20ac}'))
        );
        // 93 codewords: 138 digits fit in mode 4, 139 don't; mode 5 holds 77.
        assert!(encode(&"1".repeat(138), 4, 1, 1).is_ok());
        assert!(matches!(
            encode(&"1".repeat(139), 4, 1, 1),
            Err(SymbolError::InvalidLength(_))
        ));
        assert!(encode(&"A".repeat(77), 5, 1, 1).is_ok());
        assert!(matches!(
            encode(&"A".repeat(78), 5, 1, 1),
            Err(SymbolError::InvalidLength(_))
        ));
        assert!(encode(&"A".repeat(93), 4, 1, 1).is_ok());
        assert!(encode(&"A".repeat(93), 4, 1, 2).is_err()); // structured append takes 2
        assert!(encode(&"A".repeat(91), 4, 1, 2).is_ok());
        assert!(encode(&"a".repeat(200), 4, 1, 1).is_err());
        // Malformed high priority messages.
        assert!(matches!(
            encode("00184015238280", 2, 1, 1),
            Err(SymbolError::InvalidData(_))
        ));
        assert!(matches!(
            encode("0018401523828A2", 2, 1, 1),
            Err(SymbolError::InvalidData(_))
        ));
        assert!(matches!(
            encode("00A840152382802", 2, 1, 1),
            Err(SymbolError::InvalidData(_))
        ));
        assert!(matches!(
            encode("001826EC1A1", 3, 1, 1),
            Err(SymbolError::InvalidData(_))
        ));
        assert_eq!(
            encode("001826EC1A1;", 3, 1, 1),
            Err(SymbolError::InvalidCharacter(';'))
        );
        assert!(matches!(
            encode("[)>\u{1e}01\u{1d}96152382802\u{1d}840", 2, 1, 1),
            Err(SymbolError::InvalidData(_))
        ));
        // Mode 3 postal codes are upper-cased like zint does.
        assert_eq!(
            encode("066826ec1a1a", 3, 1, 1),
            encode("066826EC1A1A", 3, 1, 1)
        );
    }

    #[test]
    fn geometry_scales_from_nominal_size() {
        let m = encode("Hello World", 4, 1, 1).unwrap();
        let g = m.geometry(8.0);
        assert!((g.width - 225.12).abs() < 1e-3);
        assert!((g.height - 215.28).abs() < 1e-3);
        let dark = (0..ROWS)
            .flat_map(|r| (0..COLUMNS).map(move |c| (c, r)))
            .filter(|&(c, r)| m.get(c, r))
            .count();
        assert_eq!(g.hexagons.len(), dark);
        let pitch = g.width / 30.0;
        assert!((g.hex_radius * 3f32.sqrt() - pitch).abs() < 1e-4);
        // Hexagons stay inside the symbol box.
        for &(x, y) in &g.hexagons {
            assert!(x - pitch / 2.0 >= -1e-3 && x + pitch / 2.0 <= g.width + 1e-3);
            assert!(y - g.hex_radius >= -1e-3 && y + g.hex_radius <= g.height + 1e-3);
        }
        // Top-right filler modules (row 0, columns 28/29) are the last two of row 0.
        let row0: Vec<_> = g
            .hexagons
            .iter()
            .filter(|h| (h.1 - g.hex_radius).abs() < 1e-3)
            .collect();
        assert!((row0.last().unwrap().0 - 29.5 * pitch).abs() < 1e-3);
        // Odd rows are shifted right by half a module.
        let odd = g
            .hexagons
            .iter()
            .find(|h| h.1 > g.hex_radius + 1.0)
            .unwrap();
        assert!(((odd.0 / pitch) - 1.0).fract().abs() < 1e-3);
        assert!((g.center.0 - 14.5 * pitch).abs() < 1e-3);
        assert!((g.center.1 - g.height / 2.0).abs() < 1e-3);
        assert_eq!(g.rings.len(), 3);
        assert!((g.rings[0].0 - 4.5 * pitch).abs() < 1e-3);
        assert!((g.rings[2].1 - g.hex_radius).abs() < 1e-3);
        for w in g.rings.windows(2) {
            assert!(w[0].1 > w[1].0);
        }
        // Scales linearly.
        let g2 = m.geometry(12.0);
        assert!((g2.hexagons[5].0 / g.hexagons[5].0 - 1.5).abs() < 1e-4);
    }

    /// Writes the geometry of a set of symbols for the out-of-tree zxing-cpp round-trip check:
    /// `MAXICODE_DUMP_DIR=/some/dir cargo test -p zpl-symbols maxicode::tests::dump_geometry`.
    #[test]
    fn dump_geometry() {
        let Ok(dir) = std::env::var("MAXICODE_DUMP_DIR") else {
            return;
        };
        let mut cases: Vec<(String, u8, u8, u8)> = REFERENCES
            .iter()
            .map(|(d, m, n, t, _)| (d.to_string(), *m, *n, *t))
            .collect();
        cases.push(("Mode 5 full EEC 0123456789".into(), 5, 1, 1));
        cases.push(("001840152382802".into(), 2, 1, 1));
        cases.push(("999999ABCDEF".into(), 3, 1, 1));
        cases.push(("LAST OF EIGHT".into(), 4, 8, 8));
        for (i, (data, mode, n, t)) in cases.iter().enumerate() {
            let g = encode(data, *mode, *n, *t).unwrap().geometry(8.0);
            let mut s = format!(
                "mode {mode}\nn {n}\nt {t}\ndata {}\nsize {} {}\nradius {}\ncenter {} {}\n",
                data.chars()
                    .map(|c| format!("{:02x}", c as u32))
                    .collect::<String>(),
                g.width,
                g.height,
                g.hex_radius,
                g.center.0,
                g.center.1
            );
            for (o, i) in &g.rings {
                s += &format!("ring {o} {i}\n");
            }
            for (x, y) in &g.hexagons {
                s += &format!("hex {x} {y}\n");
            }
            std::fs::write(format!("{dir}/case{i:02}.txt"), s).unwrap();
        }
    }

    #[test]
    #[ignore]
    fn bench_encode() {
        let data = "001840152382802[)>\u{1e}01\u{1d}961Z00004951\u{1d}UPSN\u{1d}06X610\u{1d}159\u{1d}1234567\u{1d}1/1\u{1d}\u{1d}Y\u{1d}634 ALPHA DR\u{1d}PITTSBURGH\u{1d}PA\u{1e}\u{4}";
        let start = std::time::Instant::now();
        let iters = 20_000;
        for _ in 0..iters {
            std::hint::black_box(encode(std::hint::black_box(data), 2, 1, 1).unwrap());
        }
        eprintln!("encode: {:?}/iter", start.elapsed() / iters);
    }
}
