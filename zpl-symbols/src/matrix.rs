//! 2D (matrix and stacked) symbologies: DataMatrix (^BX), PDF417 (^B7) and Aztec (^BO/^B0).
//!
//! Every encoder returns a [`Matrix`] of modules without quiet zone. All three encoders are
//! implemented here without dependencies (an encode takes a few microseconds); they are verified
//! against reference decoders in the tests.

use std::sync::OnceLock;

/// A module grid, row-major, `true` = dark. Excludes the quiet zone.
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub width: u32,
    pub height: u32,
    pub modules: Vec<bool>,
}

impl Matrix {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            modules: vec![false; (width * height) as usize],
        }
    }

    /// Module at column `x`, row `y` (`true` = dark). Out-of-range coordinates are light.
    #[inline]
    pub fn get(&self, x: u32, y: u32) -> bool {
        x < self.width && y < self.height && self.modules[(y * self.width + x) as usize]
    }

    #[inline]
    fn set(&mut self, x: u32, y: u32) {
        let w = self.width;
        self.modules[(y * w + x) as usize] = true;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EncodeError {
    /// The data does not fit in the largest symbol (or the requested configuration).
    TooLong,
    /// The data cannot be encoded (e.g. empty Aztec data).
    InvalidData,
    /// The requested variant is not implemented (e.g. Aztec runes).
    Unsupported,
}

// ---------------------------------------------------------------------------------------------
// DataMatrix (ECC 200)
// ---------------------------------------------------------------------------------------------

/// An ECC 200 symbol size.
struct DmSize {
    rows: u32,
    cols: u32,
    data: u16,
    /// Error-correction codewords per interleaved block.
    ecc_per_block: u16,
    blocks: u16,
    /// Data region size (without finder/alignment patterns).
    region_w: u32,
    region_h: u32,
}

const fn dm(rows: u32, cols: u32, data: u16, ecc: u16, blocks: u16, rw: u32, rh: u32) -> DmSize {
    DmSize {
        rows,
        cols,
        data,
        ecc_per_block: ecc,
        blocks,
        region_w: rw,
        region_h: rh,
    }
}

/// ISO/IEC 16022 table 7, squares then rectangles (each ascending).
const DM_SIZES: [DmSize; 30] = [
    dm(10, 10, 3, 5, 1, 8, 8),
    dm(12, 12, 5, 7, 1, 10, 10),
    dm(14, 14, 8, 10, 1, 12, 12),
    dm(16, 16, 12, 12, 1, 14, 14),
    dm(18, 18, 18, 14, 1, 16, 16),
    dm(20, 20, 22, 18, 1, 18, 18),
    dm(22, 22, 30, 20, 1, 20, 20),
    dm(24, 24, 36, 24, 1, 22, 22),
    dm(26, 26, 44, 28, 1, 24, 24),
    dm(32, 32, 62, 36, 1, 14, 14),
    dm(36, 36, 86, 42, 1, 16, 16),
    dm(40, 40, 114, 48, 1, 18, 18),
    dm(44, 44, 144, 56, 1, 20, 20),
    dm(48, 48, 174, 68, 1, 22, 22),
    dm(52, 52, 204, 42, 2, 24, 24),
    dm(64, 64, 280, 56, 2, 14, 14),
    dm(72, 72, 368, 36, 4, 16, 16),
    dm(80, 80, 456, 48, 4, 18, 18),
    dm(88, 88, 576, 56, 4, 20, 20),
    dm(96, 96, 696, 68, 4, 22, 22),
    dm(104, 104, 816, 56, 6, 24, 24),
    dm(120, 120, 1050, 68, 6, 18, 18),
    dm(132, 132, 1304, 62, 8, 20, 20),
    dm(144, 144, 1558, 62, 10, 22, 22),
    dm(8, 18, 5, 7, 1, 16, 6),
    dm(8, 32, 10, 11, 1, 14, 6),
    dm(12, 26, 16, 14, 1, 24, 10),
    dm(12, 36, 22, 18, 1, 16, 10),
    dm(16, 36, 32, 24, 1, 16, 14),
    dm(16, 48, 49, 28, 1, 22, 14),
];

/// FNC1 marker in the unescaped DataMatrix input.
const DM_FNC1: u16 = 256;

/// ^BX DataMatrix (ECC 200 only; quality param 0-140 other than 200 -> treat as 200).
///
/// `rows`/`cols`: requested symbol size from ^BX params (0 = automatic smallest fit). As on
/// Zebra printers (checked on Labelary) the request is exact: a size that does not exist for
/// the aspect ratio (e.g. 8 x 8, or 16 x 20 without `rectangular`) or that the data does not
/// fit prints no symbol. One of the two alone picks the symbol with that many rows (columns).
/// `rectangular`: ^BX aspect ratio 2 — rectangular symbols (falls back to squares when the
/// data does not fit the largest rectangle and no size was requested).
/// `escape`: ^BX escape character. Supported sequences: `<e>1` FNC1 (codeword 232; a leading
/// FNC1 marks GS1 DataMatrix), `<e><e>` literal escape char, `<e>dNNN` byte with decimal value NNN, `<e>@`..`<e>_` control
/// characters 0-31. Other sequences are kept literally.
///
/// Encodation follows Labelary (ISO/IEC 16022 annex P look-ahead, see [`dm_encode`]), so the
/// symbols match it codeword for codeword.
pub fn datamatrix(
    data: &[u8],
    rows: u32,
    cols: u32,
    rectangular: bool,
    escape: Option<char>,
) -> Result<Matrix, EncodeError> {
    let input: Vec<u16> = match escape {
        Some(e) if e.is_ascii() => dm_unescape(data, e as u8),
        _ => data.iter().map(|&b| u16::from(b)).collect(),
    };
    let (size, mut cws) = dm_codewords(&input, rows, cols, rectangular)?;
    dm_ecc(&mut cws, size);
    Ok(dm_place(&cws, size))
}

/// The symbol size and its padded data codewords (see [`datamatrix`] for the size rules).
fn dm_codewords(
    input: &[u16],
    rows: u32,
    cols: u32,
    rectangular: bool,
) -> Result<(&'static DmSize, Vec<u8>), EncodeError> {
    let enc = dm_encode(input);
    // The size must hold the codewords plus the pending values (as Labelary sizes it), and
    // the finished data.
    let finished = |s: &DmSize| {
        let cap = usize::from(s.data);
        let base = enc.cws.len() + enc.pending.len();
        (base <= cap).then(|| dm_finish(&enc, input, cap - enc.cws.len())).filter(|c| c.len() <= cap)
    };
    let pick = |sizes: &'static [DmSize]| sizes.iter().find_map(|s| finished(s).map(|c| (s, c)));
    let (size, mut cws) = if rows > 0 || cols > 0 {
        DM_SIZES
            .iter()
            .filter(|s| (s.rows != s.cols) == rectangular)
            .filter(|s| (rows == 0 || s.rows == rows) && (cols == 0 || s.cols == cols))
            .find_map(|s| finished(s).map(|c| (s, c)))
    } else {
        rectangular.then(|| pick(&DM_SIZES[24..])).flatten().or_else(|| pick(&DM_SIZES[..24]))
    }
    .ok_or(EncodeError::TooLong)?;

    let cap = usize::from(size.data);
    if cws.len() < cap {
        cws.push(129);
    }
    while cws.len() < cap {
        let p = cws.len() as u32 + 1;
        let v = 129 + (149 * p) % 253 + 1;
        cws.push(if v > 254 { v - 254 } else { v } as u8);
    }
    Ok((size, cws))
}

/// Resolve ^BX escape sequences into bytes and [`DM_FNC1`] markers.
fn dm_unescape(data: &[u8], esc: u8) -> Vec<u16> {
    let mut out = Vec::with_capacity(data.len());
    let mut i = 0;
    while i < data.len() {
        let c = data[i];
        if c != esc || i + 1 >= data.len() {
            out.push(u16::from(c));
            i += 1;
            continue;
        }
        let n = data[i + 1];
        match n {
            b'1' => {
                out.push(DM_FNC1);
                i += 2;
            }
            _ if n == esc => {
                out.push(u16::from(esc));
                i += 2;
            }
            b'd' if data.len() >= i + 5 && data[i + 2..i + 5].iter().all(u8::is_ascii_digit) => {
                let v = data[i + 2..i + 5]
                    .iter()
                    .fold(0u16, |a, d| a * 10 + u16::from(d - b'0'));
                if v <= 255 {
                    out.push(v);
                    i += 5;
                } else {
                    out.push(u16::from(c));
                    i += 1;
                }
            }
            b'@'..=b'_' => {
                out.push(u16::from(n - b'@'));
                i += 2;
            }
            _ => {
                out.push(u16::from(c));
                i += 1;
            }
        }
    }
    out
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DmMode {
    Ascii,
    C40,
    Text,
    X12,
    Edifact,
    Base256,
}

/// Result of [`dm_encode`]: the codewords so far and the open C40/Text/X12/EDIFACT values,
/// finished by [`dm_finish`] once the symbol size is known.
struct DmEncoded {
    cws: Vec<u8>,
    mode: DmMode,
    /// Values of the incomplete C40/Text/X12 triple or EDIFACT group.
    pending: Vec<u16>,
}

/// C40 (`text == false`) or Text values for one character; returns the value count.
fn dm_c40_values(c: u16, text: bool, out: &mut [u16; 4]) -> usize {
    if c == DM_FNC1 {
        out[..2].copy_from_slice(&[1, 27]); // Shift 2, FNC1
        return 2;
    }
    let mut n = 0;
    let mut c = c;
    if c >= 128 {
        out[0] = 1; // Shift 2
        out[1] = 30; // Upper Shift
        n = 2;
        c -= 128;
    }
    let (shift, v) = match c {
        0..=31 => (Some(0), c),
        32 => (None, 3),
        0x30..=0x39 => (None, c - 0x30 + 4),
        0x41..=0x5a if !text => (None, c - 0x41 + 14),
        0x61..=0x7a if text => (None, c - 0x61 + 14),
        33..=47 => (Some(1), c - 33),
        58..=64 => (Some(1), c - 58 + 15),
        91..=95 => (Some(1), c - 91 + 22),
        0x41..=0x5a => (Some(2), c - 0x40), // Text: shift 3, 'A' = 1
        _ => (Some(2), c - 96),             // C40: '`', a-z, {|}~ DEL = 0..31
    };
    if let Some(s) = shift {
        out[n] = s;
        n += 1;
    }
    out[n] = v;
    n + 1
}

fn dm_is_x12(c: u16) -> bool {
    matches!(c, 13 | 42 | 62 | 32 | 0x30..=0x39 | 0x41..=0x5a)
}

/// ISO/IEC 16022 annex P look-ahead test, as Labelary (Okapi Barcode, after zint) runs it:
/// the encodation to continue with at `pos` when currently in `mode`.
fn dm_look_ahead(input: &[u16], pos: usize, mode: DmMode) -> DmMode {
    use DmMode::*;
    // Smallest difference acted on (float noise).
    const ST: f32 = 1.0 / 24.0;
    let (mut a, mut c, mut t, mut x, mut e, mut b) =
        if mode == Ascii { (0.0, 1.0, 1.0, 1.0, 1.0, 1.25) } else { (1.0, 2.0, 2.0, 2.0, 2.0, 2.25) };
    match mode {
        C40 => c = 0.0,
        Text => t = 0.0,
        X12 => x = 0.0,
        Edifact => e = 0.0,
        Base256 => b = 0.0,
        Ascii => {}
    }
    let mut sp = pos;
    loop {
        let mut best = None;
        if let Some(&ch) = input.get(sp) {
            let ext = ch > 127 && ch != DM_FNC1;
            let digit = (0x30..=0x39).contains(&ch);
            a = if digit { a + 0.5 } else { f32::ceil(a) + if ext { 2.0 } else { 1.0 } };
            let third = |basic: bool| if basic { 2.0 / 3.0 } else if ext { 8.0 / 3.0 } else { 4.0 / 3.0 };
            c += third(ch == 32 || digit || (0x41..=0x5a).contains(&ch));
            t += third(ch == 32 || digit || (0x61..=0x7a).contains(&ch));
            x += if dm_is_x12(ch) { 2.0 / 3.0 } else if ext { 13.0 / 3.0 } else { 10.0 / 3.0 };
            e += if (32..=94).contains(&ch) { 0.75 } else if ext { 17.0 / 4.0 } else { 13.0 / 4.0 };
            if ch == DM_FNC1 {
                e += 6.0;
            }
            b += if ch == DM_FNC1 { 4.0 } else { 1.0 };
        } else {
            // End of data: step (k).
            [a, c, t, x, e, b] = [a, c, t, x, e, b].map(f32::ceil);
            let mut count = c;
            best = Some(C40);
            for (n, m) in [(x, X12), (t, Text), (e, Edifact), (b, Base256)] {
                if n < count - ST {
                    (count, best) = (n, Some(m));
                }
            }
            if a <= count + ST {
                best = Some(Ascii);
            }
        }
        if sp > pos + 3 {
            // Step (r): the first scheme at least a codeword ahead of the others.
            let below = |v: f32, others: &[f32]| others.iter().all(|&o| v + 1.0 < o - ST);
            if below(c, &[a, b, e, t]) {
                if c < x - ST {
                    best = Some(C40);
                } else if c <= x + ST {
                    best = Some(if dm_x12_terminator_first(input, sp) { X12 } else { C40 });
                }
            }
            if below(x, &[a, b, e, t, c]) {
                best = Some(X12);
            }
            if below(t, &[a, b, e, x, c]) {
                best = Some(Text);
            }
            if below(e, &[a, b, t, x, c]) {
                best = Some(Edifact);
            }
            if b + 1.0 <= a + ST || below(b, &[e, t, x, c]) {
                best = Some(Base256);
            }
            if [b, e, t, x, c].iter().all(|&o| a + 1.0 <= o + ST) {
                best = Some(Ascii);
            }
        }
        if let Some(m) = best {
            return m;
        }
        sp += 1;
    }
}

/// Annex P (r)(6)(ii)(I): an X12 terminator (CR, `*`, `>`) comes before the first non-X12
/// character from `pos` on.
fn dm_x12_terminator_first(input: &[u16], pos: usize) -> bool {
    let rest = &input[pos.min(input.len())..];
    let non_x12 = rest.iter().position(|&c| !dm_is_x12(c));
    let terminator = rest.iter().position(|&c| matches!(c, 13 | 42 | 62));
    matches!((terminator, non_x12), (Some(t), Some(n)) if t < n)
}

/// Data codewords by Labelary's encodation choices: ASCII with digit pairs, switching modes
/// where the annex P look-ahead says so (checked from ASCII before every non-pair character,
/// from C40/Text/X12 at triple boundaries, from EDIFACT before a group's last character and
/// from Base 256 before every byte).
fn dm_encode(input: &[u16]) -> DmEncoded {
    use DmMode::*;
    let n = input.len();
    let mut out: Vec<u8> = Vec::with_capacity(n + 8);
    let mut pending: Vec<u16> = Vec::with_capacity(6);
    let mut vals = [0u16; 4];
    let mut b256_start = 0; // index of the Base 256 segment's first data byte in `out`
    let (mut mode, mut next) = (Ascii, Ascii);
    let is_digit = |c: u16| (0x30..=0x39).contains(&c);
    let triples = |out: &mut Vec<u8>, pending: &mut Vec<u16>| {
        while pending.len() >= 3 {
            let packed = 1600 * pending[0] + 40 * pending[1] + pending[2] + 1;
            out.extend_from_slice(&packed.to_be_bytes());
            pending.drain(..3);
        }
    };
    let mut sp = 0;
    while sp < n {
        if mode == Base256 && next != Base256 {
            dm_close_b256(&mut out, b256_start);
        }
        mode = next;
        let c = input[sp];
        match mode {
            Ascii => {
                if sp + 1 < n && is_digit(c) && is_digit(input[sp + 1]) {
                    out.push(130 + ((c - 0x30) * 10 + (input[sp + 1] - 0x30)) as u8);
                    sp += 2;
                    continue;
                }
                next = dm_look_ahead(input, sp, Ascii);
                match next {
                    C40 => out.push(230),
                    Text => out.push(239),
                    X12 => out.push(238),
                    Edifact => out.push(240),
                    Base256 => {
                        out.push(231);
                        b256_start = out.len();
                    }
                    Ascii => {
                        if c == DM_FNC1 {
                            out.push(232);
                        } else if c < 128 {
                            out.push(c as u8 + 1);
                        } else {
                            out.push(235);
                            out.push((c - 127) as u8);
                        }
                        sp += 1;
                    }
                }
            }
            C40 | Text | X12 => {
                if pending.is_empty() {
                    next = dm_look_ahead(input, sp, mode);
                    // Never start an X12 triple that cannot hold the next characters.
                    if next == X12 && input[sp..n.min(sp + 3)].iter().any(|&c| !dm_is_x12(c)) {
                        next = Ascii;
                    }
                }
                if next != mode {
                    out.push(254);
                    next = Ascii;
                    continue;
                }
                if mode == X12 {
                    pending.push(match c {
                        13 => 0,
                        42 => 1,
                        62 => 2,
                        32 => 3,
                        0x30..=0x39 => c - 0x30 + 4,
                        _ => c - 0x41 + 14,
                    });
                } else {
                    let nv = dm_c40_values(c, mode == Text, &mut vals);
                    pending.extend_from_slice(&vals[..nv]);
                }
                triples(&mut out, &mut pending);
                sp += 1;
            }
            Edifact => {
                if pending.len() == 3 {
                    next = dm_look_ahead(input, sp, Edifact);
                }
                if next == Edifact {
                    pending.push(c & 0x3f);
                    sp += 1;
                } else {
                    pending.push(31); // unlatch
                    next = Ascii;
                }
                if pending.len() == 4 {
                    dm_flush_edifact(&mut out, &pending);
                    pending.clear();
                }
            }
            Base256 => {
                next = dm_look_ahead(input, sp, Base256);
                if next == Base256 {
                    out.push(c as u8);
                    sp += 1;
                } else {
                    next = Ascii;
                }
            }
        }
    }
    if mode == Base256 {
        dm_close_b256(&mut out, b256_start);
    }
    DmEncoded { cws: out, mode, pending }
}

/// EDIFACT group of 1-4 six-bit values -> 1-3 bytes.
fn dm_flush_edifact(out: &mut Vec<u8>, values: &[u16]) {
    let v = values.iter().enumerate().fold(0u32, |a, (k, &e)| a | u32::from(e) << (18 - 6 * k));
    out.extend_from_slice(&v.to_be_bytes()[1..1 + values.len().min(3)]);
}

/// Insert a Base 256 segment's length before `start` and randomise the segment.
fn dm_close_b256(out: &mut Vec<u8>, start: usize) {
    let len = out.len() - start;
    if len <= 249 {
        out.insert(start, len as u8);
    } else {
        out.splice(start..start, [(len / 250 + 249) as u8, (len % 250) as u8]);
    }
    for (p, cw) in out.iter_mut().enumerate().skip(start) {
        let v = u32::from(*cw) + (149 * (p as u32 + 1)) % 255 + 1;
        *cw = if v > 255 { v - 256 } else { v } as u8;
    }
}

/// Closes the encodation once `left` codewords remain in the symbol (annex P / zint): the
/// pending C40/Text/X12/EDIFACT values, the last characters in ASCII where they need it, and
/// an unlatch only when there is room for more than the finished data.
fn dm_finish(enc: &DmEncoded, input: &[u16], left: usize) -> Vec<u8> {
    let mut out = enc.cws.clone();
    let p = enc.pending.len();
    // The input characters behind `k` pending single-value X12/EDIFACT values (or the last
    // C40/Text character), in ASCII.
    let ascii_tail = |out: &mut Vec<u8>, k: usize| {
        for &c in &input[input.len() - k..] {
            match c {
                DM_FNC1 => out.push(232),
                0..=127 => out.push(c as u8 + 1),
                _ => out.extend_from_slice(&[235, (c - 127) as u8]),
            }
        }
    };
    match enc.mode {
        DmMode::C40 | DmMode::Text => match p {
            1 => {
                if left > 1 {
                    out.push(254);
                }
                ascii_tail(&mut out, 1);
            }
            2 => {
                let packed = 1600 * enc.pending[0] + 40 * enc.pending[1] + 1;
                out.extend_from_slice(&packed.to_be_bytes());
                if left > 2 {
                    out.push(254);
                }
            }
            _ if left > 0 => out.push(254),
            _ => {}
        },
        DmMode::X12 => {
            if !(left == 1 && p == 1) {
                out.push(254);
            }
            ascii_tail(&mut out, p);
        }
        DmMode::Edifact if left <= 2 => ascii_tail(&mut out, p),
        DmMode::Edifact => {
            let mut group = enc.pending.clone();
            group.push(31);
            dm_flush_edifact(&mut out, &group);
        }
        DmMode::Ascii | DmMode::Base256 => {}
    }
    out
}

/// Append interleaved Reed-Solomon codewords (GF(256), poly 0x12d) to the padded data.
fn dm_ecc(cws: &mut Vec<u8>, size: &DmSize) {
    let blocks = usize::from(size.blocks);
    let ecc = usize::from(size.ecc_per_block);
    let data_len = cws.len();
    cws.resize(data_len + blocks * ecc, 0);
    // Generators cached by ECC length (max 68 per block).
    static GEN: [OnceLock<Vec<u16>>; 69] = [const { OnceLock::new() }; 69];
    let generator = GEN[ecc].get_or_init(|| rs_generator_log(ecc, 8));
    let mut block: Vec<u16> = Vec::with_capacity(data_len / blocks + 1 + ecc);
    for b in 0..blocks {
        block.clear();
        block.extend((b..data_len).step_by(blocks).map(|i| u16::from(cws[i])));
        let n = block.len();
        block.resize(n + ecc, 0);
        rs_encode(&mut block, generator, 8);
        for (j, &e) in block[n..].iter().enumerate() {
            cws[data_len + b + j * blocks] = e as u8;
        }
    }
}

/// ECC 200 codeword placement state (ISO/IEC 16022 annex F).
struct DmPlacer<'a> {
    cws: &'a [u8],
    nrow: i32,
    ncol: i32,
    /// 0 = unset, 1 = light, 2 = dark
    grid: Vec<u8>,
}

impl DmPlacer<'_> {
    fn module(&mut self, mut r: i32, mut c: i32, pos: usize, bit: u32) {
        if r < 0 {
            r += self.nrow;
            c += 4 - ((self.nrow + 4) % 8);
        }
        if c < 0 {
            c += self.ncol;
            r += 4 - ((self.ncol + 4) % 8);
        }
        let dark = self
            .cws
            .get(pos)
            .is_some_and(|&v| v & (1 << (8 - bit)) != 0);
        self.grid[(r * self.ncol + c) as usize] = if dark { 2 } else { 1 };
    }

    fn utah(&mut self, r: i32, c: i32, pos: usize) {
        let pts = [
            (r - 2, c - 2),
            (r - 2, c - 1),
            (r - 1, c - 2),
            (r - 1, c - 1),
            (r - 1, c),
            (r, c - 2),
            (r, c - 1),
            (r, c),
        ];
        self.place(pos, pts);
    }

    fn place(&mut self, pos: usize, pts: [(i32, i32); 8]) {
        for (k, (r, c)) in pts.into_iter().enumerate() {
            self.module(r, c, pos, k as u32 + 1);
        }
    }

    fn is_set(&self, r: i32, c: i32) -> bool {
        self.grid[(r * self.ncol + c) as usize] != 0
    }

    fn run(&mut self) {
        let (nr, nc) = (self.nrow, self.ncol);
        let mut pos = 0;
        let (mut row, mut col) = (4i32, 0i32);
        loop {
            if row == nr && col == 0 {
                self.place(
                    pos,
                    [
                        (nr - 1, 0),
                        (nr - 1, 1),
                        (nr - 1, 2),
                        (0, nc - 2),
                        (0, nc - 1),
                        (1, nc - 1),
                        (2, nc - 1),
                        (3, nc - 1),
                    ],
                );
                pos += 1;
            }
            if row == nr - 2 && col == 0 && nc % 4 != 0 {
                self.place(
                    pos,
                    [
                        (nr - 3, 0),
                        (nr - 2, 0),
                        (nr - 1, 0),
                        (0, nc - 4),
                        (0, nc - 3),
                        (0, nc - 2),
                        (0, nc - 1),
                        (1, nc - 1),
                    ],
                );
                pos += 1;
            }
            if row == nr - 2 && col == 0 && nc % 8 == 4 {
                self.place(
                    pos,
                    [
                        (nr - 3, 0),
                        (nr - 2, 0),
                        (nr - 1, 0),
                        (0, nc - 2),
                        (0, nc - 1),
                        (1, nc - 1),
                        (2, nc - 1),
                        (3, nc - 1),
                    ],
                );
                pos += 1;
            }
            if row == nr + 4 && col == 2 && nc % 8 == 0 {
                self.place(
                    pos,
                    [
                        (nr - 1, 0),
                        (nr - 1, nc - 1),
                        (0, nc - 3),
                        (0, nc - 2),
                        (0, nc - 1),
                        (1, nc - 3),
                        (1, nc - 2),
                        (1, nc - 1),
                    ],
                );
                pos += 1;
            }
            // Sweep upward-right.
            loop {
                if row < nr && col >= 0 && !self.is_set(row, col) {
                    self.utah(row, col, pos);
                    pos += 1;
                }
                row -= 2;
                col += 2;
                if row < 0 || col >= nc {
                    break;
                }
            }
            row += 1;
            col += 3;
            // Sweep downward-left.
            loop {
                if row >= 0 && col < nc && !self.is_set(row, col) {
                    self.utah(row, col, pos);
                    pos += 1;
                }
                row += 2;
                col -= 2;
                if row >= nr || col < 0 {
                    break;
                }
            }
            row += 3;
            col += 1;
            if row >= nr && col >= nc {
                break;
            }
        }
        if !self.is_set(nr - 1, nc - 1) {
            self.grid[((nr - 1) * nc + nc - 1) as usize] = 2;
            self.grid[((nr - 2) * nc + nc - 2) as usize] = 2;
        }
    }
}

/// Place the codewords and add finder and clock-track patterns.
fn dm_place(cws: &[u8], size: &DmSize) -> Matrix {
    let (rw, rh) = (size.region_w, size.region_h);
    let ncol = rw * (size.cols / (rw + 2));
    let nrow = rh * (size.rows / (rh + 2));
    let mut placer = DmPlacer {
        cws,
        nrow: nrow as i32,
        ncol: ncol as i32,
        grid: vec![0; (nrow * ncol) as usize],
    };
    placer.run();
    let grid = placer.grid;

    // Map data regions into the symbol, adding finder (L) and clock-track patterns.
    let mut m = Matrix::new(size.cols, size.rows);
    for my in 0..size.rows {
        let ry = my % (rh + 2);
        for mx in 0..size.cols {
            let rx = mx % (rw + 2);
            let dark = if ry == 0 {
                mx % 2 == 0 // top clock track
            } else if ry == rh + 1 || rx == 0 {
                true // bottom / left solid finder
            } else if rx == rw + 1 {
                (my - 1) % 2 == 0 // right clock track
            } else {
                let dy = (my / (rh + 2)) * rh + ry - 1;
                let dx = (mx / (rw + 2)) * rw + rx - 1;
                grid[(dy * ncol + dx) as usize] == 2
            };
            if dark {
                m.set(mx, my);
            }
        }
    }
    m
}

// ---------------------------------------------------------------------------------------------
// PDF417
// ---------------------------------------------------------------------------------------------

const PDF_START: u32 = 0b1_1111_1110_1010_1000; // 81111113, 17 modules
const PDF_STOP: u32 = 0b11_1111_1010_0010_1001; // 711311121, 18 modules
const PDF_MAX_CODEWORDS: u32 = 928;

/// ^B7 PDF417.
///
/// `security`: 0-8 error correction level; `None` = 0, Zebra's default (error detection only).
/// `columns`: data columns 1-30 (0 = auto: height:width of about 1:2 at 3-module rows, Zebra's
/// documented default).
/// `rows`: 3-90 (0 = auto). A too small explicit grid grows (rows first, then columns).
/// `truncated`: compact PDF417 (right row indicator omitted, stop pattern is a single 1-module bar).
///
/// Each PDF417 row is ONE matrix row (the renderer stretches rows to the row height).
pub fn pdf417(
    data: &[u8],
    security: Option<u8>,
    columns: u32,
    rows: u32,
    truncated: bool,
) -> Result<Matrix, EncodeError> {
    let mut cws: Vec<u16> = Vec::with_capacity(data.len() + 8);
    cws.push(0); // symbol length descriptor, filled in below
    pdf_high_level(data, &mut cws);
    let n_data = cws.len() as u32;

    // ^B7 security level, default 0 (error detection only).
    let level = u32::from(security.unwrap_or(0).min(8));
    let n_ecc = 2u32 << level;
    let needed = n_data + n_ecc;
    if needed > PDF_MAX_CODEWORDS {
        return Err(EncodeError::TooLong);
    }
    let (rows, cols) = pdf_dimensions(needed, columns, rows).ok_or(EncodeError::TooLong)?;

    // Pad, set the length descriptor and append error correction.
    let total = rows * cols;
    cws.resize((total - n_ecc) as usize, 900);
    cws[0] = (total - n_ecc) as u16;
    pdf_ecc(&mut cws, level as usize);

    let width = if truncated {
        17 + 17 + 17 * cols + 1
    } else {
        17 + 17 + 17 * cols + 17 + 18
    };
    let mut m = Matrix::new(width, rows);
    let rows_val = (rows - 1) / 3;
    let cols_val = cols - 1;
    let level_val = level * 3 + (rows - 1) % 3;
    for r in 0..rows {
        let cluster = (r % 3) as usize;
        let base = (r / 3) * 30;
        let (left, right) = match cluster {
            0 => (rows_val, cols_val),
            1 => (level_val, rows_val),
            _ => (cols_val, level_val),
        };
        let mut x = 0;
        let mut put = |m: &mut Matrix, bits: u32, len: u32| {
            for b in (0..len).rev() {
                if bits >> b & 1 != 0 {
                    m.set(x + len - 1 - b, r);
                }
            }
            x += len;
        };
        let cw = |v: u32| 0x1_0000 | u32::from(CODEWORDS[cluster][v as usize]);
        put(&mut m, PDF_START, 17);
        put(&mut m, cw(base + left), 17);
        for c in 0..cols {
            put(&mut m, cw(u32::from(cws[(r * cols + c) as usize])), 17);
        }
        if truncated {
            put(&mut m, 1, 1);
        } else {
            put(&mut m, cw(base + right), 17);
            put(&mut m, PDF_STOP, 18);
        }
    }
    Ok(m)
}

/// Pick (rows, columns) holding `needed` codewords, honouring the requested values if possible.
fn pdf_dimensions(needed: u32, columns: u32, rows: u32) -> Option<(u32, u32)> {
    // Smallest valid row count >= min_rows for a column count (capped to the codeword limit).
    let fit = |cols: u32, min_rows: u32| -> Option<(u32, u32)> {
        if !(1..=30).contains(&cols) {
            return None;
        }
        let need = needed.div_ceil(cols).max(3);
        let r = min_rows.max(need).min(90).min(PDF_MAX_CODEWORDS / cols);
        (r >= need).then_some((r, cols))
    };
    let search = |pref: u32, min_rows: u32| {
        (0..30)
            .find_map(|d| fit(pref + d, min_rows).or_else(|| fit(pref.wrapping_sub(d), min_rows)))
    };
    let req_rows = if rows > 0 { rows.clamp(3, 90) } else { 3 };
    let pref_cols = if columns > 0 {
        columns.clamp(1, 30)
    } else if rows > 0 {
        needed.div_ceil(req_rows).clamp(1, 30)
    } else {
        // Zebra's 1:2 aspect ratio is height:width with rows at the nominal 3 modules: the
        // column count whose 3 * rows / (69 + 17 * columns) modules comes closest to 1/2
        // (fewest columns on a tie). Matches Labelary for 11 to 258 codewords.
        let aspect = |cols: u32| (3 * needed.div_ceil(cols).max(3)) as f32 / (69 + 17 * cols) as f32;
        (1..=30)
            .filter(|&cols| needed.div_ceil(cols) <= 90)
            .min_by(|&a, &b| (aspect(a) - 0.5).abs().total_cmp(&(aspect(b) - 0.5).abs()))
            .unwrap_or(30)
    };
    search(pref_cols, req_rows).or_else(|| search(pref_cols, 3))
}

/// PDF417 high-level encoding (text / byte / numeric compaction), after zxing's
/// `PDF417HighLevelEncoder` in auto mode, with Zebra's choice between text and numeric runs.
pub(crate) fn pdf_high_level(msg: &[u8], out: &mut Vec<u16>) {
    const TEXT: u8 = 0;
    const BYTE: u8 = 1;
    const NUMERIC: u8 = 2;
    let len = msg.len();
    let mut mode = TEXT;
    let mut sub = SUB_ALPHA;
    let mut p = 0;
    while p < len {
        let n = consecutive_digits(msg, p);
        if numeric_run(msg, p, n) {
            out.push(902);
            mode = NUMERIC;
            sub = SUB_ALPHA;
            pdf_numeric(&msg[p..p + n], out);
            p += n;
            continue;
        }
        let t = consecutive_text(msg, p);
        // Any text run stays text (zxing switches to bytes for runs under 5; Zebra and
        // Labelary encode e.g. "AB" as one text codeword).
        if t > 0 {
            if mode != TEXT {
                out.push(900);
                mode = TEXT;
                sub = SUB_ALPHA;
            }
            sub = pdf_text(&msg[p..p + t], sub, out);
            p += t;
            continue;
        }
        let b = consecutive_binary(msg, p).max(1);
        if b == 1 && mode == TEXT {
            out.push(913);
            out.push(u16::from(msg[p]));
        } else {
            pdf_bytes(&msg[p..p + b], out);
            mode = BYTE;
        }
        p += b;
    }
}

/// Whether the `digits` digits at `start` go into numeric compaction. zxing waits for 13;
/// Zebra (measured on Labelary) switches as soon as it is no longer than staying in text,
/// counting half a codeword per text value (the digits plus the latches into and out of
/// mixed) against 902, the numeric codewords and the 900 back to text. A message of digits
/// only is always numeric.
fn numeric_run(msg: &[u8], start: usize, digits: usize) -> bool {
    if digits == 0 {
        return false;
    }
    if digits >= 13 || digits == msg.len() {
        return true;
    }
    let more = usize::from(start + digits < msg.len());
    // Base-900 codewords for "1" followed by the digits: about 2.95 digits per codeword.
    let numeric = 1 + (digits as f32 / 900f32.log10()) as usize + 1 + more;
    2 * numeric <= 1 + digits + more
}

fn consecutive_digits(msg: &[u8], start: usize) -> usize {
    msg[start..]
        .iter()
        .take_while(|c| c.is_ascii_digit())
        .count()
}

pub(crate) fn is_pdf_text(c: u8) -> bool {
    c == b'\t' || c == b'\n' || c == b'\r' || (32..=126).contains(&c)
}

fn consecutive_text(msg: &[u8], start: usize) -> usize {
    let mut idx = start;
    while idx < msg.len() {
        let digits = consecutive_digits(msg, idx);
        if idx > start && numeric_run(msg, idx, digits) {
            break;
        }
        if digits > 0 {
            idx += digits;
            continue;
        }
        if !is_pdf_text(msg[idx]) {
            break;
        }
        idx += 1;
    }
    idx - start
}

fn consecutive_binary(msg: &[u8], start: usize) -> usize {
    let mut idx = start;
    while idx < msg.len() {
        if numeric_run(msg, idx, consecutive_digits(msg, idx)) {
            break;
        }
        if idx > start && consecutive_text(msg, idx) >= 5 {
            break;
        }
        idx += 1;
    }
    idx - start
}

pub(crate) fn pdf_bytes(bytes: &[u8], out: &mut Vec<u16>) {
    out.push(if bytes.len().is_multiple_of(6) {
        924
    } else {
        901
    });
    let mut chunks = bytes.chunks_exact(6);
    for chunk in &mut chunks {
        let mut v = chunk.iter().fold(0u64, |a, &b| (a << 8) | u64::from(b));
        let mut five = [0u16; 5];
        for slot in five.iter_mut().rev() {
            *slot = (v % 900) as u16;
            v /= 900;
        }
        out.extend_from_slice(&five);
    }
    out.extend(chunks.remainder().iter().map(|&b| u16::from(b)));
}

pub(crate) fn pdf_numeric(digits: &[u8], out: &mut Vec<u16>) {
    for group in digits.chunks(44) {
        // Big number "1" + digits, as base-10 digits, converted to base 900 by long division.
        let mut num: Vec<u32> = Vec::with_capacity(45);
        num.push(1);
        num.extend(group.iter().map(|&d| u32::from(d - b'0')));
        let mut res: Vec<u16> = Vec::with_capacity(16);
        while !num.is_empty() {
            let mut rem = 0u32;
            let mut q = Vec::with_capacity(num.len());
            for &d in &num {
                let cur = rem * 10 + d;
                let qd = cur / 900;
                rem = cur % 900;
                if !(q.is_empty() && qd == 0) {
                    q.push(qd);
                }
            }
            res.push(rem as u16);
            num = q;
        }
        out.extend(res.iter().rev());
    }
}

pub(crate) const SUB_ALPHA: u8 = 0;
const SUB_LOWER: u8 = 1;
const SUB_MIXED: u8 = 2;
const SUB_PUNCT: u8 = 3;

const PDF_MIXED_RAW: [u8; 25] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'&', b'\r', b'\t', b',', b':',
    b'#', b'-', b'.', b'$', b'/', b'+', b'%', b'*', b'=', b'^',
];
const PDF_PUNCT_RAW: [u8; 29] = [
    b';', b'<', b'>', b'@', b'[', b'\\', b']', b'_', b'`', b'~', b'!', b'\r', b'\t', b',', b':',
    b'\n', b'-', b'.', b'$', b'/', b'"', b'|', b'*', b'(', b')', b'?', b'{', b'}', b'\'',
];

fn pdf_mixed(c: u8) -> Option<u16> {
    if c == b' ' {
        return Some(26);
    }
    PDF_MIXED_RAW.iter().position(|&m| m == c).map(|p| p as u16)
}

fn pdf_punct(c: u8) -> Option<u16> {
    PDF_PUNCT_RAW.iter().position(|&m| m == c).map(|p| p as u16)
}

pub(crate) fn pdf_text(msg: &[u8], mut sub: u8, out: &mut Vec<u16>) -> u8 {
    let upper = |c: u8| c == b' ' || c.is_ascii_uppercase();
    let lower = |c: u8| c == b' ' || c.is_ascii_lowercase();
    let mut tmp: Vec<u16> = Vec::with_capacity(msg.len() * 2);
    let mut idx = 0;
    while idx < msg.len() {
        let ch = msg[idx];
        match sub {
            SUB_ALPHA => {
                if upper(ch) {
                    tmp.push(if ch == b' ' { 26 } else { u16::from(ch - b'A') });
                } else if lower(ch) {
                    sub = SUB_LOWER;
                    tmp.push(27);
                    continue;
                } else if pdf_mixed(ch).is_some() {
                    sub = SUB_MIXED;
                    tmp.push(28);
                    continue;
                } else {
                    tmp.push(29);
                    tmp.push(pdf_punct(ch).unwrap_or(0));
                }
            }
            SUB_LOWER => {
                if lower(ch) {
                    tmp.push(if ch == b' ' { 26 } else { u16::from(ch - b'a') });
                } else if upper(ch) {
                    tmp.push(27);
                    tmp.push(u16::from(ch - b'A'));
                } else if pdf_mixed(ch).is_some() {
                    sub = SUB_MIXED;
                    tmp.push(28);
                    continue;
                } else {
                    tmp.push(29);
                    tmp.push(pdf_punct(ch).unwrap_or(0));
                }
            }
            SUB_MIXED => {
                if let Some(v) = pdf_mixed(ch) {
                    tmp.push(v);
                } else if upper(ch) {
                    sub = SUB_ALPHA;
                    tmp.push(28);
                    continue;
                } else if lower(ch) {
                    sub = SUB_LOWER;
                    tmp.push(27);
                    continue;
                } else if msg.get(idx + 1).is_some_and(|&n| pdf_punct(n).is_some()) {
                    sub = SUB_PUNCT;
                    tmp.push(25);
                    continue;
                } else {
                    tmp.push(29);
                    tmp.push(pdf_punct(ch).unwrap_or(0));
                }
            }
            _ => {
                if let Some(v) = pdf_punct(ch) {
                    tmp.push(v);
                } else {
                    sub = SUB_ALPHA;
                    tmp.push(29);
                    continue;
                }
            }
        }
        idx += 1;
    }
    for pair in tmp.chunks(2) {
        out.push(pair[0] * 30 + pair.get(1).copied().unwrap_or(29));
    }
    // The odd-length pad value 29 is PS in alpha/lower/mixed but AL (latch) in punctuation.
    if tmp.len() % 2 == 1 && sub == SUB_PUNCT {
        sub = SUB_ALPHA;
    }
    sub
}

/// PDF417 error-correction generator coefficients (low degree first) for a level.
fn pdf_generator(level: usize) -> &'static [u16] {
    static CACHE: [OnceLock<Vec<u16>>; 9] = [const { OnceLock::new() }; 9];
    CACHE[level].get_or_init(|| {
        let k = 2usize << level;
        let mut poly: Vec<u32> = vec![1];
        let mut a = 1u32;
        for _ in 0..k {
            a = a * 3 % 929;
            // poly *= (x - a)
            let mut next = vec![0u32; poly.len() + 1];
            for (j, &p) in poly.iter().enumerate() {
                next[j + 1] = (next[j + 1] + p) % 929;
                next[j] = (next[j] + 929 - p * a % 929) % 929;
            }
            poly = next;
        }
        poly[..k].iter().map(|&v| v as u16).collect()
    })
}

/// Append the error-correction codewords for `cws` (all data codewords).
fn pdf_ecc(cws: &mut Vec<u16>, level: usize) {
    let coef = pdf_generator(level);
    let k = coef.len();
    let mut e = vec![0u32; k];
    for &d in cws.iter() {
        let t1 = (u32::from(d) + e[k - 1]) % 929;
        for j in (1..k).rev() {
            let t2 = t1 * u32::from(coef[j]) % 929;
            e[j] = (e[j - 1] + 929 - t2) % 929;
        }
        e[0] = (929 - t1 * u32::from(coef[0]) % 929) % 929;
    }
    cws.extend(
        e.iter()
            .rev()
            .map(|&v| if v == 0 { 0 } else { (929 - v) as u16 }),
    );
}

// ---------------------------------------------------------------------------------------------
// Aztec
// ---------------------------------------------------------------------------------------------

/// ^BO / ^B0 Aztec. `ecc_percent`: 0 = default (Zint's 23% + 3 codewords sizing), 1-99 =
/// minimum error correction percent, 101-104 = compact 1-4 layers, 201-232 = full-range 1-32
/// layers, 300 = rune. Other values are treated as the default. If the data does not fit a fixed layer count the
/// symbol is sized automatically instead.
pub fn aztec(data: &[u8], ecc_percent: u32) -> Result<Matrix, EncodeError> {
    if ecc_percent == 300 {
        // Like Labelary, data that is not a number 0-255 is rune 0.
        let value = std::str::from_utf8(data).ok().and_then(|d| d.trim().parse::<u8>().ok());
        return Ok(aztec_rune(value.unwrap_or(0)));
    }
    if data.is_empty() {
        return Err(EncodeError::InvalidData);
    }
    let bits = aztec_high_level(data);
    let fixed = match ecc_percent {
        101..=104 => Some((true, ecc_percent - 100)),
        201..=232 => Some((false, ecc_percent - 200)),
        _ => None,
    };
    let percent = if (1..=99).contains(&ecc_percent) {
        ecc_percent
    } else {
        23
    };
    let ecc_bits = bits.len() as u32 * percent / 100 + 11;

    let layout = fixed
        .and_then(|(compact, layers)| aztec_fixed_layout(&bits, compact, layers, ecc_bits))
        .or_else(|| match ecc_percent {
            1..=99 => aztec_auto_layout(&bits, ecc_bits),
            _ => aztec_default_layout(&bits),
        })
        .ok_or(EncodeError::TooLong)?;
    Ok(aztec_draw(layout))
}

struct AztecLayout {
    compact: bool,
    layers: u32,
    word_size: u32,
    stuffed: Vec<bool>,
}

const AZTEC_WORD_SIZE: [u32; 33] = [
    4, 6, 6, 8, 8, 8, 8, 8, 8, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 12, 12, 12,
    12, 12, 12, 12, 12, 12, 12,
];

fn aztec_total_bits(layers: u32, compact: bool) -> u32 {
    ((if compact { 88 } else { 112 }) + 16 * layers) * layers
}

fn aztec_fixed_layout(
    bits: &[bool],
    compact: bool,
    layers: u32,
    ecc_bits: u32,
) -> Option<AztecLayout> {
    let total = aztec_total_bits(layers, compact);
    let word_size = AZTEC_WORD_SIZE[layers as usize];
    let usable = total - total % word_size;
    let stuffed = aztec_stuff_bits(bits, word_size);
    let len = stuffed.len() as u32;
    if len + ecc_bits > usable || (compact && len > word_size * 64) {
        return None;
    }
    Some(AztecLayout {
        compact,
        layers,
        word_size,
        stuffed,
    })
}

fn aztec_auto_layout(bits: &[bool], ecc_bits: u32) -> Option<AztecLayout> {
    let total_needed = bits.len() as u32 + ecc_bits;
    let mut word_size = 0;
    let mut stuffed = Vec::new();
    for i in 0..=32u32 {
        let compact = i <= 3;
        let layers = if compact { i + 1 } else { i };
        let total = aztec_total_bits(layers, compact);
        if total_needed > total {
            continue;
        }
        if word_size != AZTEC_WORD_SIZE[layers as usize] {
            word_size = AZTEC_WORD_SIZE[layers as usize];
            stuffed = aztec_stuff_bits(bits, word_size);
        }
        let usable = total - total % word_size;
        let len = stuffed.len() as u32;
        if compact && len > word_size * 64 {
            continue;
        }
        if len + ecc_bits <= usable {
            return Some(AztecLayout {
                compact,
                layers,
                word_size,
                stuffed,
            });
        }
    }
    None
}

/// Most data bits per layer count at the default error correction (23% + 3 codewords), from
/// Zint, whose sizing Labelary follows: compact symbols, then full-range ones.
const AZTEC_COMPACT_23: [u32; 4] = [66, 168, 288, 440];
const AZTEC_FULL_23: [u32; 32] = [
    84, 204, 352, 520, 720, 944, 1184, 1456, 1750, 2070, 2410, 2780, 3180, 3590, 4040, 4500, 5000,
    5520, 6060, 6630, 7210, 7830, 8472, 9132, 9816, 10536, 11280, 12036, 12828, 13644, 14472, 15348,
];

/// The smallest symbol whose default-ECC capacity exceeds the stuffed data, compact first.
fn aztec_default_layout(bits: &[bool]) -> Option<AztecLayout> {
    let compact = AZTEC_COMPACT_23.iter().enumerate().map(|(i, &max)| (true, i as u32 + 1, max));
    let full = AZTEC_FULL_23.iter().enumerate().map(|(i, &max)| (false, i as u32 + 1, max));
    compact.chain(full).find_map(|(compact, layers, max)| {
        let word_size = AZTEC_WORD_SIZE[layers as usize];
        let stuffed = aztec_stuff_bits(bits, word_size);
        (stuffed.len() < max as usize).then_some(AztecLayout { compact, layers, word_size, stuffed })
    })
}

fn push_bits(out: &mut Vec<bool>, value: u32, n: u32) {
    for b in (0..n).rev() {
        out.push(value >> b & 1 != 0);
    }
}

fn aztec_stuff_bits(bits: &[bool], word_size: u32) -> Vec<bool> {
    let mut out = Vec::with_capacity(bits.len() + bits.len() / 4 + 16);
    let n = bits.len() as i64;
    let ws = i64::from(word_size);
    let mask = (1u32 << word_size) - 2;
    let mut i: i64 = 0;
    while i < n {
        let mut word = 0u32;
        for j in 0..ws {
            if i + j >= n || bits[(i + j) as usize] {
                word |= 1 << (ws - 1 - j);
            }
        }
        if word & mask == mask {
            push_bits(&mut out, word & mask, word_size);
            i -= 1;
        } else if word & mask == 0 {
            push_bits(&mut out, word | 1, word_size);
            i -= 1;
        } else {
            push_bits(&mut out, word, word_size);
        }
        i += ws;
    }
    out
}

/// Galois field tables (exp doubled to avoid a modulo) for the Aztec word sizes.
struct Gf {
    exp: Vec<u16>,
    log: Vec<u16>,
    size: usize,
}

fn aztec_gf(word_size: u32) -> &'static Gf {
    static CACHE: [OnceLock<Gf>; 5] = [const { OnceLock::new() }; 5];
    let (idx, poly) = match word_size {
        4 => (0, 0x13),
        6 => (1, 0x43),
        8 => (2, 0x12d),
        10 => (3, 0x409),
        _ => (4, 0x1069),
    };
    CACHE[idx].get_or_init(|| {
        let size = 1usize << word_size;
        let mut exp = vec![0u16; size * 2];
        let mut log = vec![0u16; size];
        let mut x = 1usize;
        for e in exp.iter_mut().take(size) {
            *e = x as u16;
            x <<= 1;
            if x >= size {
                x ^= poly;
            }
        }
        for i in 0..size - 1 {
            log[exp[i] as usize] = i as u16;
        }
        for i in size..size * 2 {
            exp[i] = exp[i - (size - 1)];
        }
        Gf { exp, log, size }
    })
}

impl Gf {
    #[inline]
    fn mul(&self, a: u16, b: u16) -> u16 {
        if a == 0 || b == 0 {
            0
        } else {
            self.exp[self.log[a as usize] as usize + self.log[b as usize] as usize]
        }
    }
}

/// Reed-Solomon generator prod_{i=1..=ec} (x + a^i) over the Aztec/DataMatrix field for
/// `word_size`, as logarithms of the non-leading coefficients (highest degree first; `u16::MAX`
/// marks a zero coefficient).
fn rs_generator_log(ec: usize, word_size: u32) -> Vec<u16> {
    let gf = aztec_gf(word_size);
    let mut g: Vec<u16> = Vec::with_capacity(ec + 1);
    g.push(1);
    for i in 1..=ec {
        let a = gf.exp[i % (gf.size - 1)];
        g.push(0);
        for j in (1..g.len()).rev() {
            g[j] ^= gf.mul(g[j - 1], a);
        }
    }
    g[1..]
        .iter()
        .map(|&c| if c == 0 { u16::MAX } else { gf.log[c as usize] })
        .collect()
}

/// Reed-Solomon encode `words` in place: the last `gen_log.len()` entries become check words.
fn rs_encode(words: &mut [u16], gen_log: &[u16], word_size: u32) {
    let gf = aztec_gf(word_size);
    let ec = gen_log.len();
    let data_len = words.len() - ec;
    let (data, rem) = words.split_at_mut(data_len);
    rem.fill(0);
    for &d in data.iter() {
        let f = d ^ rem[0];
        rem.copy_within(1.., 0);
        rem[ec - 1] = 0;
        if f != 0 {
            let lf = gf.log[f as usize] as usize;
            for (r, &lg) in rem.iter_mut().zip(gen_log) {
                if lg != u16::MAX {
                    *r ^= gf.exp[lg as usize + lf];
                }
            }
        }
    }
}

/// Reed-Solomon encode with a freshly computed generator (Aztec: one symbol, many sizes).
fn aztec_rs(words: &mut [u16], ec: usize, word_size: u32) {
    rs_encode(words, &rs_generator_log(ec, word_size), word_size);
}

/// Message words + check words as a bit stream of `total_bits` (left padded with zeros).
fn aztec_check_words(bits: &[bool], total_bits: u32, word_size: u32) -> Vec<bool> {
    let ws = word_size as usize;
    let n_msg = bits.len() / ws;
    let n_total = (total_bits / word_size) as usize;
    let mut words = vec![0u16; n_total];
    for (w, chunk) in words.iter_mut().zip(bits.chunks_exact(ws)) {
        *w = chunk.iter().fold(0u16, |a, &b| (a << 1) | u16::from(b));
    }
    aztec_rs(&mut words, n_total - n_msg, word_size);
    let mut out = Vec::with_capacity(total_bits as usize);
    out.resize((total_bits % word_size) as usize, false);
    for &w in &words {
        push_bits(&mut out, u32::from(w), word_size);
    }
    out
}

fn aztec_draw(l: AztecLayout) -> Matrix {
    let AztecLayout {
        compact,
        layers,
        word_size,
        stuffed,
    } = l;
    let total_bits = aztec_total_bits(layers, compact);
    let message = aztec_check_words(&stuffed, total_bits, word_size);
    let msg_words = stuffed.len() as u32 / word_size;

    let mut mode = Vec::with_capacity(40);
    let mode_bits = if compact {
        push_bits(&mut mode, layers - 1, 2);
        push_bits(&mut mode, msg_words - 1, 6);
        aztec_check_words(&mode, 28, 4)
    } else {
        push_bits(&mut mode, layers - 1, 5);
        push_bits(&mut mode, msg_words - 1, 11);
        aztec_check_words(&mode, 40, 4)
    };

    let base = (if compact { 11 } else { 14 }) + layers * 4;
    let mut align = vec![0u32; base as usize];
    let size;
    if compact {
        size = base;
        for (i, a) in align.iter_mut().enumerate() {
            *a = i as u32;
        }
    } else {
        size = base + 1 + 2 * ((base / 2 - 1) / 15);
        let orig_center = base / 2;
        let center = size / 2;
        for i in 0..orig_center {
            let off = i + i / 15;
            align[(orig_center - i - 1) as usize] = center - off - 1;
            align[(orig_center + i) as usize] = center + off + 1;
        }
    }
    let mut m = Matrix::new(size, size);
    let a = |i: u32| align[i as usize];
    let bit = |i: u32| message[i as usize];

    let mut row_offset = 0;
    for i in 0..layers {
        let row_size = (layers - i) * 4 + if compact { 9 } else { 12 };
        for j in 0..row_size {
            let col = j * 2;
            for k in 0..2 {
                if bit(row_offset + col + k) {
                    m.set(a(i * 2 + k), a(i * 2 + j));
                }
                if bit(row_offset + row_size * 2 + col + k) {
                    m.set(a(i * 2 + j), a(base - 1 - i * 2 - k));
                }
                if bit(row_offset + row_size * 4 + col + k) {
                    m.set(a(base - 1 - i * 2 - k), a(base - 1 - i * 2 - j));
                }
                if bit(row_offset + row_size * 6 + col + k) {
                    m.set(a(base - 1 - i * 2 - j), a(i * 2 + k));
                }
            }
        }
        row_offset += row_size * 8;
    }

    aztec_core(&mut m, compact, &mode_bits);

    // Reference grid (full-range only).
    let center = size / 2;
    if !compact {
        let mut i = 0;
        let mut j = 0;
        while i < base / 2 - 1 {
            let mut k = center & 1;
            while k < size {
                m.set(center - j, k);
                m.set(center + j, k);
                m.set(k, center - j);
                m.set(k, center + j);
                k += 2;
            }
            i += 15;
            j += 16;
        }
    }
    m
}

/// Draws the mode message and the bull's eye with its orientation marks around the centre.
fn aztec_core(m: &mut Matrix, compact: bool, mode_bits: &[bool]) {
    let center = m.width / 2;
    if compact {
        for i in 0..7 {
            let off = center - 3 + i;
            let i = i as usize;
            if mode_bits[i] {
                m.set(off, center - 5);
            }
            if mode_bits[i + 7] {
                m.set(center + 5, off);
            }
            if mode_bits[20 - i] {
                m.set(off, center + 5);
            }
            if mode_bits[27 - i] {
                m.set(center - 5, off);
            }
        }
    } else {
        for i in 0..10 {
            let off = center - 5 + i + i / 5;
            let i = i as usize;
            if mode_bits[i] {
                m.set(off, center - 7);
            }
            if mode_bits[i + 10] {
                m.set(center + 7, off);
            }
            if mode_bits[29 - i] {
                m.set(off, center + 7);
            }
            if mode_bits[39 - i] {
                m.set(center - 7, off);
            }
        }
    }

    // Bull's eye with orientation marks.
    let eye = if compact { 5 } else { 7 };
    for i in (0..eye).step_by(2) {
        for j in center - i..=center + i {
            m.set(j, center - i);
            m.set(j, center + i);
            m.set(center - i, j);
            m.set(center + i, j);
        }
    }
    m.set(center - eye, center - eye);
    m.set(center - eye + 1, center - eye);
    m.set(center - eye, center - eye + 1);
    m.set(center + eye, center - eye);
    m.set(center + eye, center - eye + 1);
    m.set(center + eye, center + eye - 1);

}

/// Aztec rune `value` (^B0 ECC 300): the 11 x 11 compact core alone, its mode message the
/// value's two 4-bit words plus five check words, every other bit inverted.
fn aztec_rune(value: u8) -> Matrix {
    let mut word = Vec::with_capacity(8);
    push_bits(&mut word, u32::from(value), 8);
    let mut mode_bits = aztec_check_words(&word, 28, 4);
    for bit in mode_bits.iter_mut().step_by(2) {
        *bit = !*bit;
    }
    let mut m = Matrix::new(11, 11);
    aztec_core(&mut m, true, &mode_bits);
    m
}

// Aztec character modes.
const AZ_UPPER: usize = 0;
const AZ_LOWER: usize = 1;
const AZ_MIXED: usize = 2;
const AZ_DIGIT: usize = 3;

/// Code of `c` in a mode (0 = not encodable in that mode).
fn az_code(mode: usize, c: u8) -> u32 {
    match mode {
        AZ_UPPER => match c {
            b' ' => 1,
            b'A'..=b'Z' => u32::from(c - b'A') + 2,
            _ => 0,
        },
        AZ_LOWER => match c {
            b' ' => 1,
            b'a'..=b'z' => u32::from(c - b'a') + 2,
            _ => 0,
        },
        AZ_MIXED => match c {
            b' ' => 1,
            1..=13 => u32::from(c) + 1,
            27..=31 => u32::from(c - 27) + 15,
            b'@' => 20,
            b'\\' => 21,
            b'^' => 22,
            b'_' => 23,
            b'`' => 24,
            b'|' => 25,
            b'~' => 26,
            127 => 27,
            _ => 0,
        },
        _ => match c {
            b' ' => 1,
            b'0'..=b'9' => u32::from(c - b'0') + 2,
            b',' => 12,
            b'.' => 13,
            _ => 0,
        },
    }
}

fn az_punct(c: u8) -> u32 {
    match c {
        b'\r' => 1,
        b'!' => 6,
        b'"' => 7,
        b'#' => 8,
        b'$' => 9,
        b'%' => 10,
        b'&' => 11,
        b'\'' => 12,
        b'(' => 13,
        b')' => 14,
        b'*' => 15,
        b'+' => 16,
        b',' => 17,
        b'-' => 18,
        b'.' => 19,
        b'/' => 20,
        b':' => 21,
        b';' => 22,
        b'<' => 23,
        b'=' => 24,
        b'>' => 25,
        b'?' => 26,
        b'[' => 27,
        b']' => 28,
        b'{' => 29,
        b'}' => 30,
        _ => 0,
    }
}

const AZ_PUNCT: usize = 4;
/// DP states 5..=7: inside a binary-shift run started from upper / lower / mixed.
const AZ_BIN: usize = 5;
const AZ_STATES: usize = 8;
/// Longest binary-shift run (5-bit 0 + 11-bit length + 31).
const AZ_MAX_BIN: u32 = 2047 + 31;

fn az_bits(mode: usize) -> u32 {
    if mode == AZ_DIGIT { 4 } else { 5 }
}

/// Latch sequences between the character modes as (code, bit count) pairs.
fn az_latch_seq(from: usize, to: usize) -> &'static [(u32, u32)] {
    match (from, to) {
        (AZ_UPPER, AZ_LOWER) => &[(28, 5)],
        (AZ_UPPER, AZ_MIXED) => &[(29, 5)],
        (AZ_UPPER, AZ_DIGIT) => &[(30, 5)],
        (AZ_UPPER, AZ_PUNCT) => &[(29, 5), (30, 5)],
        // Zint (and so Labelary) goes through mixed, not the shorter digit latch.
        (AZ_LOWER, AZ_UPPER) => &[(29, 5), (29, 5)],
        (AZ_LOWER, AZ_MIXED) => &[(29, 5)],
        (AZ_LOWER, AZ_DIGIT) => &[(30, 5)],
        (AZ_LOWER, AZ_PUNCT) => &[(29, 5), (30, 5)],
        (AZ_MIXED, AZ_UPPER) => &[(29, 5)],
        (AZ_MIXED, AZ_LOWER) => &[(28, 5)],
        (AZ_MIXED, AZ_DIGIT) => &[(29, 5), (30, 5)],
        (AZ_MIXED, AZ_PUNCT) => &[(30, 5)],
        (AZ_DIGIT, AZ_UPPER) => &[(14, 4)],
        (AZ_DIGIT, AZ_LOWER) => &[(14, 4), (28, 5)],
        (AZ_DIGIT, AZ_MIXED) => &[(14, 4), (29, 5)],
        (AZ_DIGIT, AZ_PUNCT) => &[(14, 4), (29, 5), (30, 5)],
        (AZ_PUNCT, AZ_UPPER) => &[(31, 5)],
        (AZ_PUNCT, AZ_LOWER) => &[(31, 5), (28, 5)],
        (AZ_PUNCT, AZ_MIXED) => &[(31, 5), (29, 5)],
        (AZ_PUNCT, AZ_DIGIT) => &[(31, 5), (30, 5)],
        _ => &[],
    }
}

/// Latch cost in bits (precomputed from [`az_latch_seq`]).
const AZ_LATCH_BITS: [[u32; 5]; 5] = [
    [0, 5, 5, 5, 10],
    [10, 0, 5, 5, 10],
    [5, 5, 0, 10, 5],
    [4, 9, 9, 0, 14],
    [5, 10, 10, 10, 0],
];

/// Code of `c` (or the two-character pair `c`,`next` when `pair`) in a mode incl. punctuation.
fn az_mode_code(mode: usize, c: u8) -> u32 {
    if mode == AZ_PUNCT {
        az_punct(c)
    } else {
        az_code(mode, c)
    }
}

/// Punctuation pair code for (CR LF), ". ", ", ", ": ".
fn az_pair(c: u8, next: u8) -> u32 {
    match (c, next) {
        (b'\r', b'\n') => 2,
        (b'.', b' ') => 3,
        (b',', b' ') => 4,
        (b':', b' ') => 5,
        _ => 0,
    }
}

/// How a DP state was reached.
#[derive(Clone, Copy)]
enum AzOp {
    /// Latch (possibly none) to the state's mode and encode one character or a pair.
    Char,
    /// Punctuation shift (one char or pair) staying in the mode.
    PunctShift,
    /// Upper shift from lower or digit.
    UpperShift,
    /// Start a binary-shift run (latching to upper first from digit/punct).
    BinStart,
    /// Extend a binary-shift run by one byte.
    BinExtend,
}

/// Aztec high-level encoding with a shortest-path search over (position, mode) in bits:
/// character modes with latches, punctuation/upper shifts, punctuation pairs and binary shift
/// runs.
fn aztec_high_level(data: &[u8]) -> Vec<bool> {
    const INF: u32 = u32::MAX;
    let n = data.len();
    let mut cost = vec![INF; (n + 1) * AZ_STATES];
    // (previous position, previous state, op, binary run length at this state)
    let mut back = vec![(0u32, 0u8, AzOp::Char, 0u32); (n + 1) * AZ_STATES];
    cost[AZ_UPPER] = 0;
    for i in 0..=n {
        let row = i * AZ_STATES;
        // Ending a binary run is free: continue in the mode it was started from.
        for r in 0..3 {
            let c = cost[row + AZ_BIN + r];
            if c != INF && c < cost[row + r] {
                cost[row + r] = c;
                back[row + r] = back[row + AZ_BIN + r];
                back[row + r].3 = u32::MAX; // marker: inherits the binary state's step
            }
        }
        if i == n {
            break;
        }
        let ch = data[i];
        let pair = data.get(i + 1).map_or(0, |&nx| az_pair(ch, nx));
        // Like Zint, only bytes no character mode holds go in binary shift runs.
        let binary = (0..4).all(|m| az_code(m, ch) == 0) && az_punct(ch) == 0;
        let mut cur = [INF; AZ_STATES];
        cur.copy_from_slice(&cost[row..row + AZ_STATES]);
        let runs = [
            back[row + AZ_BIN].3,
            back[row + AZ_BIN + 1].3,
            back[row + AZ_BIN + 2].3,
        ];
        let mut relax = |j: usize, t: usize, from: usize, op: AzOp, c: u32, run: u32| {
            let k = j * AZ_STATES + t;
            if c < cost[k] {
                cost[k] = c;
                back[k] = (i as u32, from as u8, op, run);
            }
        };
        for m in 0..5 {
            let c = cur[m];
            if c == INF {
                continue;
            }
            // Like Zint (and so Labelary), a character the current mode holds (a space, or
            // `,` `.` in digit mode) never latches away, even when that would be shorter.
            let held = az_mode_code(m, ch) != 0;
            for (t, &latch) in AZ_LATCH_BITS[m].iter().enumerate() {
                if held && t != m {
                    continue;
                }
                if az_mode_code(t, ch) != 0 {
                    relax(i + 1, t, m, AzOp::Char, c + latch + az_bits(t), 0);
                }
                if t == AZ_PUNCT && pair != 0 {
                    relax(i + 2, t, m, AzOp::Char, c + latch + 5, 0);
                }
            }
            if m != AZ_PUNCT {
                if az_punct(ch) != 0 {
                    relax(i + 1, m, m, AzOp::PunctShift, c + az_bits(m) + 5, 0);
                }
                if pair != 0 {
                    relax(i + 2, m, m, AzOp::PunctShift, c + az_bits(m) + 5, 0);
                }
            }
            // Upper shift for a lone capital only; a run of them latches.
            let lone = !data.get(i + 1).is_some_and(u8::is_ascii_uppercase);
            if (m == AZ_LOWER || m == AZ_DIGIT) && lone && az_code(AZ_UPPER, ch) != 0 {
                relax(i + 1, m, m, AzOp::UpperShift, c + az_bits(m) + 5, 0);
            }
            let (ret, pre) = match m {
                AZ_UPPER | AZ_LOWER | AZ_MIXED => (m, 0),
                _ => (AZ_UPPER, AZ_LATCH_BITS[m][AZ_UPPER]),
            };
            if binary {
                relax(i + 1, AZ_BIN + ret, m, AzOp::BinStart, c + pre + 10 + 8, 1);
            }
        }
        for r in 0..3 {
            let c = cur[AZ_BIN + r];
            let run = runs[r];
            if c != INF && binary && run < AZ_MAX_BIN {
                let extra = if run + 1 == 32 { 11 } else { 0 };
                relax(
                    i + 1,
                    AZ_BIN + r,
                    AZ_BIN + r,
                    AzOp::BinExtend,
                    c + 8 + extra,
                    run + 1,
                );
            }
        }
    }
    let end = (0..AZ_STATES)
        .min_by_key(|&s| cost[n * AZ_STATES + s])
        .unwrap_or(AZ_UPPER);

    // Reconstruct steps (from, to, from_state, to_state, op).
    let mut steps: Vec<(usize, usize, usize, usize, AzOp)> = Vec::with_capacity(n);
    let (mut j, mut s) = (n, end);
    while j > 0 {
        let (pi, ps, op, run) = back[j * AZ_STATES + s];
        // A char state reached by ending a binary run carries that run's step.
        let to_state = if run == u32::MAX { AZ_BIN + s } else { s };
        steps.push((pi as usize, j, ps as usize, to_state, op));
        j = pi as usize;
        s = ps as usize;
    }
    steps.reverse();

    let mut out = Vec::with_capacity(n * 6 + 16);
    let mut k = 0;
    while k < steps.len() {
        let (from, to, fs, ts, op) = steps[k];
        let c = data[from];
        match op {
            AzOp::Char => {
                for &(v, b) in az_latch_seq(fs, ts) {
                    push_bits(&mut out, v, b);
                }
                let code = if to - from == 2 {
                    az_pair(c, data[from + 1])
                } else {
                    az_mode_code(ts, c)
                };
                push_bits(&mut out, code, az_bits(ts));
            }
            AzOp::PunctShift => {
                push_bits(&mut out, 0, az_bits(fs));
                let code = if to - from == 2 {
                    az_pair(c, data[from + 1])
                } else {
                    az_punct(c)
                };
                push_bits(&mut out, code, 5);
            }
            AzOp::UpperShift => {
                push_bits(&mut out, if fs == AZ_DIGIT { 15 } else { 28 }, az_bits(fs));
                push_bits(&mut out, az_code(AZ_UPPER, c), 5);
            }
            AzOp::BinStart | AzOp::BinExtend => {
                // Collect the whole run: this start plus the following extends.
                if fs == AZ_DIGIT || fs == AZ_PUNCT {
                    for &(v, b) in az_latch_seq(fs, AZ_UPPER) {
                        push_bits(&mut out, v, b);
                    }
                }
                let mut end = k + 1;
                while end < steps.len() && matches!(steps[end].4, AzOp::BinExtend) {
                    end += 1;
                }
                let len = end - k;
                push_bits(&mut out, 31, 5);
                if len <= 31 {
                    push_bits(&mut out, len as u32, 5);
                } else {
                    push_bits(&mut out, 0, 5);
                    push_bits(&mut out, len as u32 - 31, 11);
                }
                for &b in &data[from..from + len] {
                    push_bits(&mut out, u32::from(b), 8);
                }
                k = end;
                continue;
            }
        }
        k += 1;
    }
    out
}

/// PDF417 bar/space patterns per cluster (0, 3, 6), 17 modules each with the leading bar bit
/// implied. Taken from the MIT-licensed `pdf417` crate (Copyright (c) 2023 Sofiane).
#[rustfmt::skip]
pub(crate) static CODEWORDS: [[u16; 929]; 3] = [
    [
        0xd5c0, 0xeaf0, 0xf57c, 0xd4e0, 0xea78, 0xf53e, 0xa8c0, 0xd470, 0xa860, 0x5040, 0xa830, 0x5020,
        0xadc0, 0xd6f0, 0xeb7c, 0xace0, 0xd678, 0xeb3e, 0x58c0, 0xac70, 0x5860, 0x5dc0, 0xaef0, 0xd77c,
        0x5ce0, 0xae78, 0xd73e, 0x5c70, 0xae3c, 0x5ef0, 0xaf7c, 0x5e78, 0xaf3e, 0x5f7c, 0xf5fa, 0xd2e0,
        0xe978, 0xf4be, 0xa4c0, 0xd270, 0xe93c, 0xa460, 0xd238, 0x4840, 0xa430, 0xd21c, 0x4820, 0xa418,
        0x4810, 0xa6e0, 0xd378, 0xe9be, 0x4cc0, 0xa670, 0xd33c, 0x4c60, 0xa638, 0xd31e, 0x4c30, 0xa61c,
        0x4ee0, 0xa778, 0xd3be, 0x4e70, 0xa73c, 0x4e38, 0xa71e, 0x4f78, 0xa7be, 0x4f3c, 0x4f1e, 0xa2c0,
        0xd170, 0xe8bc, 0xa260, 0xd138, 0xe89e, 0x4440, 0xa230, 0xd11c, 0x4420, 0xa218, 0x4410, 0x4408,
        0x46c0, 0xa370, 0xd1bc, 0x4660, 0xa338, 0xd19e, 0x4630, 0xa31c, 0x4618, 0x460c, 0x4770, 0xa3bc,
        0x4738, 0xa39e, 0x471c, 0x47bc, 0xa160, 0xd0b8, 0xe85e, 0x4240, 0xa130, 0xd09c, 0x4220, 0xa118,
        0xd08e, 0x4210, 0xa10c, 0x4208, 0xa106, 0x4360, 0xa1b8, 0xd0de, 0x4330, 0xa19c, 0x4318, 0xa18e,
        0x430c, 0x4306, 0xa1de, 0x438e, 0x4140, 0xa0b0, 0xd05c, 0x4120, 0xa098, 0xd04e, 0x4110, 0xa08c,
        0x4108, 0xa086, 0x4104, 0x41b0, 0x4198, 0x418c, 0x40a0, 0xd02e, 0xa04c, 0xa046, 0x4082, 0xcae0,
        0xe578, 0xf2be, 0x94c0, 0xca70, 0xe53c, 0x9460, 0xca38, 0xe51e, 0x2840, 0x9430, 0x2820, 0x96e0,
        0xcb78, 0xe5be, 0x2cc0, 0x9670, 0xcb3c, 0x2c60, 0x9638, 0x2c30, 0x2c18, 0x2ee0, 0x9778, 0xcbbe,
        0x2e70, 0x973c, 0x2e38, 0x2e1c, 0x2f78, 0x97be, 0x2f3c, 0x2fbe, 0xdac0, 0xed70, 0xf6bc, 0xda60,
        0xed38, 0xf69e, 0xb440, 0xda30, 0xed1c, 0xb420, 0xda18, 0xed0e, 0xb410, 0xda0c, 0x92c0, 0xc970,
        0xe4bc, 0xb6c0, 0x9260, 0xc938, 0xe49e, 0xb660, 0xdb38, 0xed9e, 0x6c40, 0x2420, 0x9218, 0xc90e,
        0x6c20, 0xb618, 0x6c10, 0x26c0, 0x9370, 0xc9bc, 0x6ec0, 0x2660, 0x9338, 0xc99e, 0x6e60, 0xb738,
        0xdb9e, 0x6e30, 0x2618, 0x6e18, 0x2770, 0x93bc, 0x6f70, 0x2738, 0x939e, 0x6f38, 0xb79e, 0x6f1c,
        0x27bc, 0x6fbc, 0x279e, 0x6f9e, 0xd960, 0xecb8, 0xf65e, 0xb240, 0xd930, 0xec9c, 0xb220, 0xd918,
        0xec8e, 0xb210, 0xd90c, 0xb208, 0xb204, 0x9160, 0xc8b8, 0xe45e, 0xb360, 0x9130, 0xc89c, 0x6640,
        0x2220, 0xd99c, 0xc88e, 0x6620, 0x2210, 0x910c, 0x6610, 0xb30c, 0x9106, 0x2204, 0x2360, 0x91b8,
        0xc8de, 0x6760, 0x2330, 0x919c, 0x6730, 0xb39c, 0x918e, 0x6718, 0x230c, 0x2306, 0x23b8, 0x91de,
        0x67b8, 0x239c, 0x679c, 0x238e, 0x678e, 0x67de, 0xb140, 0xd8b0, 0xec5c, 0xb120, 0xd898, 0xec4e,
        0xb110, 0xd88c, 0xb108, 0xd886, 0xb104, 0xb102, 0x2140, 0x90b0, 0xc85c, 0x6340, 0x2120, 0x9098,
        0xc84e, 0x6320, 0xb198, 0xd8ce, 0x6310, 0x2108, 0x9086, 0x6308, 0xb186, 0x6304, 0x21b0, 0x90dc,
        0x63b0, 0x2198, 0x90ce, 0x6398, 0xb1ce, 0x638c, 0x2186, 0x6386, 0x63dc, 0x63ce, 0xb0a0, 0xd858,
        0xec2e, 0xb090, 0xd84c, 0xb088, 0xd846, 0xb084, 0xb082, 0x20a0, 0x9058, 0xc82e, 0x61a0, 0x2090,
        0x904c, 0x6190, 0xb0cc, 0x9046, 0x6188, 0x2084, 0x6184, 0x2082, 0x20d8, 0x61d8, 0x61cc, 0x61c6,
        0xd82c, 0xd826, 0xb042, 0x902c, 0x2048, 0x60c8, 0x60c4, 0x60c2, 0x8ac0, 0xc570, 0xe2bc, 0x8a60,
        0xc538, 0x1440, 0x8a30, 0xc51c, 0x1420, 0x8a18, 0x1410, 0x1408, 0x16c0, 0x8b70, 0xc5bc, 0x1660,
        0x8b38, 0xc59e, 0x1630, 0x8b1c, 0x1618, 0x160c, 0x1770, 0x8bbc, 0x1738, 0x8b9e, 0x171c, 0x17bc,
        0x179e, 0xcd60, 0xe6b8, 0xf35e, 0x9a40, 0xcd30, 0xe69c, 0x9a20, 0xcd18, 0xe68e, 0x9a10, 0xcd0c,
        0x9a08, 0xcd06, 0x8960, 0xc4b8, 0xe25e, 0x9b60, 0x8930, 0xc49c, 0x3640, 0x1220, 0xcd9c, 0xc48e,
        0x3620, 0x9b18, 0x890c, 0x3610, 0x1208, 0x3608, 0x1360, 0x89b8, 0xc4de, 0x3760, 0x1330, 0xcdde,
        0x3730, 0x9b9c, 0x898e, 0x3718, 0x130c, 0x370c, 0x13b8, 0x89de, 0x37b8, 0x139c, 0x379c, 0x138e,
        0x13de, 0x37de, 0xdd40, 0xeeb0, 0xf75c, 0xdd20, 0xee98, 0xf74e, 0xdd10, 0xee8c, 0xdd08, 0xee86,
        0xdd04, 0x9940, 0xccb0, 0xe65c, 0xbb40, 0x9920, 0xeedc, 0xe64e, 0xbb20, 0xdd98, 0xeece, 0xbb10,
        0x9908, 0xcc86, 0xbb08, 0xdd86, 0x9902, 0x1140, 0x88b0, 0xc45c, 0x3340, 0x1120, 0x8898, 0xc44e,
        0x7740, 0x3320, 0x9998, 0xccce, 0x7720, 0xbb98, 0xddce, 0x8886, 0x7710, 0x3308, 0x9986, 0x7708,
        0x1102, 0x11b0, 0x88dc, 0x33b0, 0x1198, 0x88ce, 0x77b0, 0x3398, 0x99ce, 0x7798, 0xbbce, 0x1186,
        0x3386, 0x11dc, 0x33dc, 0x11ce, 0x77dc, 0x33ce, 0xdca0, 0xee58, 0xf72e, 0xdc90, 0xee4c, 0xdc88,
        0xee46, 0xdc84, 0xdc82, 0x98a0, 0xcc58, 0xe62e, 0xb9a0, 0x9890, 0xee6e, 0xb990, 0xdccc, 0xcc46,
        0xb988, 0x9884, 0xb984, 0x9882, 0xb982, 0x10a0, 0x8858, 0xc42e, 0x31a0, 0x1090, 0x884c, 0x73a0,
        0x3190, 0x98cc, 0x8846, 0x7390, 0xb9cc, 0x1084, 0x7388, 0x3184, 0x1082, 0x3182, 0x10d8, 0x886e,
        0x31d8, 0x10cc, 0x73d8, 0x31cc, 0x10c6, 0x73cc, 0x31c6, 0x10ee, 0x73ee, 0xdc50, 0xee2c, 0xdc48,
        0xee26, 0xdc44, 0xdc42, 0x9850, 0xcc2c, 0xb8d0, 0x9848, 0xcc26, 0xb8c8, 0xdc66, 0xb8c4, 0x9842,
        0xb8c2, 0x1050, 0x882c, 0x30d0, 0x1048, 0x8826, 0x71d0, 0x30c8, 0x9866, 0x71c8, 0xb8e6, 0x1042,
        0x71c4, 0x30c2, 0x71c2, 0x30ec, 0x71ec, 0x71e6, 0xee16, 0xdc22, 0xcc16, 0x9824, 0x9822, 0x1028,
        0x3068, 0x70e8, 0x1022, 0x3062, 0x8560, 0x0a40, 0x8530, 0x0a20, 0x8518, 0xc28e, 0x0a10, 0x850c,
        0x0a08, 0x8506, 0x0b60, 0x85b8, 0xc2de, 0x0b30, 0x859c, 0x0b18, 0x858e, 0x0b0c, 0x0b06, 0x0bb8,
        0x85de, 0x0b9c, 0x0b8e, 0x0bde, 0x8d40, 0xc6b0, 0xe35c, 0x8d20, 0xc698, 0x8d10, 0xc68c, 0x8d08,
        0xc686, 0x8d04, 0x0940, 0x84b0, 0xc25c, 0x1b40, 0x0920, 0xc6dc, 0xc24e, 0x1b20, 0x8d98, 0xc6ce,
        0x1b10, 0x0908, 0x8486, 0x1b08, 0x8d86, 0x0902, 0x09b0, 0x84dc, 0x1bb0, 0x0998, 0x84ce, 0x1b98,
        0x8dce, 0x1b8c, 0x0986, 0x09dc, 0x1bdc, 0x09ce, 0x1bce, 0xcea0, 0xe758, 0xf3ae, 0xce90, 0xe74c,
        0xce88, 0xe746, 0xce84, 0xce82, 0x8ca0, 0xc658, 0x9da0, 0x8c90, 0xc64c, 0x9d90, 0xcecc, 0xc646,
        0x9d88, 0x8c84, 0x9d84, 0x8c82, 0x9d82, 0x08a0, 0x8458, 0x19a0, 0x0890, 0xc66e, 0x3ba0, 0x1990,
        0x8ccc, 0x8446, 0x3b90, 0x9dcc, 0x0884, 0x3b88, 0x1984, 0x0882, 0x1982, 0x08d8, 0x846e, 0x19d8,
        0x08cc, 0x3bd8, 0x19cc, 0x08c6, 0x3bcc, 0x19c6, 0x08ee, 0x19ee, 0x3bee, 0xef50, 0xf7ac, 0xef48,
        0xf7a6, 0xef44, 0xef42, 0xce50, 0xe72c, 0xded0, 0xef6c, 0xe726, 0xdec8, 0xef66, 0xdec4, 0xce42,
        0xdec2, 0x8c50, 0xc62c, 0x9cd0, 0x8c48, 0xc626, 0xbdd0, 0x9cc8, 0xce66, 0xbdc8, 0xdee6, 0x8c42,
        0xbdc4, 0x9cc2, 0xbdc2, 0x0850, 0x842c, 0x18d0, 0x0848, 0x8426, 0x39d0, 0x18c8, 0x8c66, 0x7bd0,
        0x39c8, 0x9ce6, 0x0842, 0x7bc8, 0xbde6, 0x18c2, 0x7bc4, 0x086c, 0x18ec, 0x0866, 0x39ec, 0x18e6,
        0x7bec, 0x39e6, 0x7be6, 0xef28, 0xf796, 0xef24, 0xef22, 0xce28, 0xe716, 0xde68, 0xef36, 0xde64,
        0xce22, 0xde62, 0x8c28, 0xc616, 0x9c68, 0x8c24, 0xbce8, 0x9c64, 0x8c22, 0xbce4, 0x9c62, 0xbce2,
        0x0828, 0x8416, 0x1868, 0x8c36, 0x38e8, 0x1864, 0x0822, 0x79e8, 0x38e4, 0x1862, 0x79e4, 0x38e2,
        0x79e2, 0x1876, 0x79f6, 0xef12, 0xde34, 0xde32, 0x9c34, 0xbc74, 0xbc72, 0x1834, 0x3874, 0x78f4,
        0x78f2, 0x0540, 0x0520, 0x8298, 0x0510, 0x0508, 0x0504, 0x05b0, 0x0598, 0x058c, 0x0586, 0x05dc,
        0x05ce, 0x86a0, 0x8690, 0xc34c, 0x8688, 0xc346, 0x8684, 0x8682, 0x04a0, 0x8258, 0x0da0, 0x86d8,
        0x824c, 0x0d90, 0x86cc, 0x0d88, 0x86c6, 0x0d84, 0x0482, 0x0d82, 0x04d8, 0x826e, 0x0dd8, 0x86ee,
        0x0dcc, 0x04c6, 0x0dc6, 0x04ee, 0x0dee, 0xc750, 0xc748, 0xc744, 0xc742, 0x8650, 0x8ed0, 0xc76c,
        0xc326, 0x8ec8, 0xc766, 0x8ec4, 0x8642, 0x8ec2, 0x0450, 0x0cd0, 0x0448, 0x8226, 0x1dd0, 0x0cc8,
        0x0444, 0x1dc8, 0x0cc4, 0x0442, 0x1dc4, 0x0cc2, 0x046c, 0x0cec, 0x0466, 0x1dec, 0x0ce6, 0x1de6,
        0xe7a8, 0xe7a4, 0xe7a2, 0xc728, 0xcf68, 0xe7b6, 0xcf64, 0xc722, 0xcf62, 0x8628, 0xc316, 0x8e68,
        0xc736, 0x9ee8, 0x8e64, 0x8622, 0x9ee4, 0x8e62, 0x9ee2, 0x0428, 0x8216, 0x0c68, 0x8636, 0x1ce8,
        0x0c64, 0x0422, 0x3de8, 0x1ce4, 0x0c62, 0x3de4, 0x1ce2, 0x0436, 0x0c76, 0x1cf6, 0x3df6, 0xf7d4,
        0xf7d2, 0xe794, 0xefb4, 0xe792, 0xefb2, 0xc714, 0xcf34, 0xc712, 0xdf74, 0xcf32, 0xdf72, 0x8614,
        0x8e34, 0x8612, 0x9e74, 0x8e32, 0xbef4,
    ],
    [
        0xf560, 0xfab8, 0xea40, 0xf530, 0xfa9c, 0xea20, 0xf518, 0xfa8e, 0xea10, 0xf50c, 0xea08, 0xf506,
        0xea04, 0xeb60, 0xf5b8, 0xfade, 0xd640, 0xeb30, 0xf59c, 0xd620, 0xeb18, 0xf58e, 0xd610, 0xeb0c,
        0xd608, 0xeb06, 0xd604, 0xd760, 0xebb8, 0xf5de, 0xae40, 0xd730, 0xeb9c, 0xae20, 0xd718, 0xeb8e,
        0xae10, 0xd70c, 0xae08, 0xd706, 0xae04, 0xaf60, 0xd7b8, 0xebde, 0x5e40, 0xaf30, 0xd79c, 0x5e20,
        0xaf18, 0xd78e, 0x5e10, 0xaf0c, 0x5e08, 0xaf06, 0x5f60, 0xafb8, 0xd7de, 0x5f30, 0xaf9c, 0x5f18,
        0xaf8e, 0x5f0c, 0x5fb8, 0xafde, 0x5f9c, 0x5f8e, 0xe940, 0xf4b0, 0xfa5c, 0xe920, 0xf498, 0xfa4e,
        0xe910, 0xf48c, 0xe908, 0xf486, 0xe904, 0xe902, 0xd340, 0xe9b0, 0xf4dc, 0xd320, 0xe998, 0xf4ce,
        0xd310, 0xe98c, 0xd308, 0xe986, 0xd304, 0xd302, 0xa740, 0xd3b0, 0xe9dc, 0xa720, 0xd398, 0xe9ce,
        0xa710, 0xd38c, 0xa708, 0xd386, 0xa704, 0xa702, 0x4f40, 0xa7b0, 0xd3dc, 0x4f20, 0xa798, 0xd3ce,
        0x4f10, 0xa78c, 0x4f08, 0xa786, 0x4f04, 0x4fb0, 0xa7dc, 0x4f98, 0xa7ce, 0x4f8c, 0x4f86, 0x4fdc,
        0x4fce, 0xe8a0, 0xf458, 0xfa2e, 0xe890, 0xf44c, 0xe888, 0xf446, 0xe884, 0xe882, 0xd1a0, 0xe8d8,
        0xf46e, 0xd190, 0xe8cc, 0xd188, 0xe8c6, 0xd184, 0xd182, 0xa3a0, 0xd1d8, 0xe8ee, 0xa390, 0xd1cc,
        0xa388, 0xd1c6, 0xa384, 0xa382, 0x47a0, 0xa3d8, 0xd1ee, 0x4790, 0xa3cc, 0x4788, 0xa3c6, 0x4784,
        0x4782, 0x47d8, 0xa3ee, 0x47cc, 0x47c6, 0x47ee, 0xe850, 0xf42c, 0xe848, 0xf426, 0xe844, 0xe842,
        0xd0d0, 0xe86c, 0xd0c8, 0xe866, 0xd0c4, 0xd0c2, 0xa1d0, 0xd0ec, 0xa1c8, 0xd0e6, 0xa1c4, 0xa1c2,
        0x43d0, 0xa1ec, 0x43c8, 0xa1e6, 0x43c4, 0x43c2, 0x43ec, 0x43e6, 0xe828, 0xf416, 0xe824, 0xe822,
        0xd068, 0xe836, 0xd064, 0xd062, 0xa0e8, 0xd076, 0xa0e4, 0xa0e2, 0x41e8, 0xa0f6, 0x41e4, 0x41e2,
        0xe814, 0xe812, 0xd034, 0xd032, 0xa074, 0xa072, 0xe540, 0xf2b0, 0xf95c, 0xe520, 0xf298, 0xf94e,
        0xe510, 0xf28c, 0xe508, 0xf286, 0xe504, 0xe502, 0xcb40, 0xe5b0, 0xf2dc, 0xcb20, 0xe598, 0xf2ce,
        0xcb10, 0xe58c, 0xcb08, 0xe586, 0xcb04, 0xcb02, 0x9740, 0xcbb0, 0xe5dc, 0x9720, 0xcb98, 0xe5ce,
        0x9710, 0xcb8c, 0x9708, 0xcb86, 0x9704, 0x9702, 0x2f40, 0x97b0, 0xcbdc, 0x2f20, 0x9798, 0xcbce,
        0x2f10, 0x978c, 0x2f08, 0x9786, 0x2f04, 0x2fb0, 0x97dc, 0x2f98, 0x97ce, 0x2f8c, 0x2f86, 0x2fdc,
        0x2fce, 0xf6a0, 0xfb58, 0x6bf0, 0xf690, 0xfb4c, 0x69f8, 0xf688, 0xfb46, 0x68fc, 0xf684, 0xf682,
        0xe4a0, 0xf258, 0xf92e, 0xeda0, 0xe490, 0xfb6e, 0xed90, 0xf6cc, 0xf246, 0xed88, 0xe484, 0xed84,
        0xe482, 0xed82, 0xc9a0, 0xe4d8, 0xf26e, 0xdba0, 0xc990, 0xe4cc, 0xdb90, 0xedcc, 0xe4c6, 0xdb88,
        0xc984, 0xdb84, 0xc982, 0xdb82, 0x93a0, 0xc9d8, 0xe4ee, 0xb7a0, 0x9390, 0xc9cc, 0xb790, 0xdbcc,
        0xc9c6, 0xb788, 0x9384, 0xb784, 0x9382, 0xb782, 0x27a0, 0x93d8, 0xc9ee, 0x6fa0, 0x2790, 0x93cc,
        0x6f90, 0xb7cc, 0x93c6, 0x6f88, 0x2784, 0x6f84, 0x2782, 0x27d8, 0x93ee, 0x6fd8, 0x27cc, 0x6fcc,
        0x27c6, 0x6fc6, 0x27ee, 0xf650, 0xfb2c, 0x65f8, 0xf648, 0xfb26, 0x64fc, 0xf644, 0x647e, 0xf642,
        0xe450, 0xf22c, 0xecd0, 0xe448, 0xf226, 0xecc8, 0xf666, 0xecc4, 0xe442, 0xecc2, 0xc8d0, 0xe46c,
        0xd9d0, 0xc8c8, 0xe466, 0xd9c8, 0xece6, 0xd9c4, 0xc8c2, 0xd9c2, 0x91d0, 0xc8ec, 0xb3d0, 0x91c8,
        0xc8e6, 0xb3c8, 0xd9e6, 0xb3c4, 0x91c2, 0xb3c2, 0x23d0, 0x91ec, 0x67d0, 0x23c8, 0x91e6, 0x67c8,
        0xb3e6, 0x67c4, 0x23c2, 0x67c2, 0x23ec, 0x67ec, 0x23e6, 0x67e6, 0xf628, 0xfb16, 0x62fc, 0xf624,
        0x627e, 0xf622, 0xe428, 0xf216, 0xec68, 0xf636, 0xec64, 0xe422, 0xec62, 0xc868, 0xe436, 0xd8e8,
        0xc864, 0xd8e4, 0xc862, 0xd8e2, 0x90e8, 0xc876, 0xb1e8, 0xd8f6, 0xb1e4, 0x90e2, 0xb1e2, 0x21e8,
        0x90f6, 0x63e8, 0x21e4, 0x63e4, 0x21e2, 0x63e2, 0x21f6, 0x63f6, 0xf614, 0x617e, 0xf612, 0xe414,
        0xec34, 0xe412, 0xec32, 0xc834, 0xd874, 0xc832, 0xd872, 0x9074, 0xb0f4, 0x9072, 0xb0f2, 0x20f4,
        0x61f4, 0x20f2, 0x61f2, 0xf60a, 0xe40a, 0xec1a, 0xc81a, 0xd83a, 0x903a, 0xb07a, 0xe2a0, 0xf158,
        0xf8ae, 0xe290, 0xf14c, 0xe288, 0xf146, 0xe284, 0xe282, 0xc5a0, 0xe2d8, 0xf16e, 0xc590, 0xe2cc,
        0xc588, 0xe2c6, 0xc584, 0xc582, 0x8ba0, 0xc5d8, 0xe2ee, 0x8b90, 0xc5cc, 0x8b88, 0xc5c6, 0x8b84,
        0x8b82, 0x17a0, 0x8bd8, 0xc5ee, 0x1790, 0x8bcc, 0x1788, 0x8bc6, 0x1784, 0x1782, 0x17d8, 0x8bee,
        0x17cc, 0x17c6, 0x17ee, 0xf350, 0xf9ac, 0x35f8, 0xf348, 0xf9a6, 0x34fc, 0xf344, 0x347e, 0xf342,
        0xe250, 0xf12c, 0xe6d0, 0xe248, 0xf126, 0xe6c8, 0xf366, 0xe6c4, 0xe242, 0xe6c2, 0xc4d0, 0xe26c,
        0xcdd0, 0xc4c8, 0xe266, 0xcdc8, 0xe6e6, 0xcdc4, 0xc4c2, 0xcdc2, 0x89d0, 0xc4ec, 0x9bd0, 0x89c8,
        0xc4e6, 0x9bc8, 0xcde6, 0x9bc4, 0x89c2, 0x9bc2, 0x13d0, 0x89ec, 0x37d0, 0x13c8, 0x89e6, 0x37c8,
        0x9be6, 0x37c4, 0x13c2, 0x37c2, 0x13ec, 0x37ec, 0x13e6, 0x37e6, 0xfba8, 0x75f0, 0xbafc, 0xfba4,
        0x74f8, 0xba7e, 0xfba2, 0x747c, 0x743e, 0xf328, 0xf996, 0x32fc, 0xf768, 0xfbb6, 0x76fc, 0x327e,
        0xf764, 0xf322, 0x767e, 0xf762, 0xe228, 0xf116, 0xe668, 0xe224, 0xeee8, 0xf776, 0xe222, 0xeee4,
        0xe662, 0xeee2, 0xc468, 0xe236, 0xcce8, 0xc464, 0xdde8, 0xcce4, 0xc462, 0xdde4, 0xcce2, 0xdde2,
        0x88e8, 0xc476, 0x99e8, 0x88e4, 0xbbe8, 0x99e4, 0x88e2, 0xbbe4, 0x99e2, 0xbbe2, 0x11e8, 0x88f6,
        0x33e8, 0x11e4, 0x77e8, 0x33e4, 0x11e2, 0x77e4, 0x33e2, 0x77e2, 0x11f6, 0x33f6, 0xfb94, 0x72f8,
        0xb97e, 0xfb92, 0x727c, 0x723e, 0xf314, 0x317e, 0xf734, 0xf312, 0x737e, 0xf732, 0xe214, 0xe634,
        0xe212, 0xee74, 0xe632, 0xee72, 0xc434, 0xcc74, 0xc432, 0xdcf4, 0xcc72, 0xdcf2, 0x8874, 0x98f4,
        0x8872, 0xb9f4, 0x98f2, 0xb9f2, 0x10f4, 0x31f4, 0x10f2, 0x73f4, 0x31f2, 0x73f2, 0xfb8a, 0x717c,
        0x713e, 0xf30a, 0xf71a, 0xe20a, 0xe61a, 0xee3a, 0xc41a, 0xcc3a, 0xdc7a, 0x883a, 0x987a, 0xb8fa,
        0x107a, 0x30fa, 0x71fa, 0x70be, 0xe150, 0xf0ac, 0xe148, 0xf0a6, 0xe144, 0xe142, 0xc2d0, 0xe16c,
        0xc2c8, 0xe166, 0xc2c4, 0xc2c2, 0x85d0, 0xc2ec, 0x85c8, 0xc2e6, 0x85c4, 0x85c2, 0x0bd0, 0x85ec,
        0x0bc8, 0x85e6, 0x0bc4, 0x0bc2, 0x0bec, 0x0be6, 0xf1a8, 0xf8d6, 0x1afc, 0xf1a4, 0x1a7e, 0xf1a2,
        0xe128, 0xf096, 0xe368, 0xe124, 0xe364, 0xe122, 0xe362, 0xc268, 0xe136, 0xc6e8, 0xc264, 0xc6e4,
        0xc262, 0xc6e2, 0x84e8, 0xc276, 0x8de8, 0x84e4, 0x8de4, 0x84e2, 0x8de2, 0x09e8, 0x84f6, 0x1be8,
        0x09e4, 0x1be4, 0x09e2, 0x1be2, 0x09f6, 0x1bf6, 0xf9d4, 0x3af8, 0x9d7e, 0xf9d2, 0x3a7c, 0x3a3e,
        0xf194, 0x197e, 0xf3b4, 0xf192, 0x3b7e, 0xf3b2, 0xe114, 0xe334, 0xe112, 0xe774, 0xe332, 0xe772,
        0xc234, 0xc674, 0xc232, 0xcef4, 0xc672, 0xcef2, 0x8474, 0x8cf4, 0x8472, 0x9df4, 0x8cf2, 0x9df2,
        0x08f4, 0x19f4, 0x08f2, 0x3bf4, 0x19f2, 0x3bf2, 0x7af0, 0xbd7c, 0x7a78, 0xbd3e, 0x7a3c, 0x7a1e,
        0xf9ca, 0x397c, 0xfbda, 0x7b7c, 0x393e, 0x7b3e, 0xf18a, 0xf39a, 0xf7ba, 0xe10a, 0xe31a, 0xe73a,
        0xef7a, 0xc21a, 0xc63a, 0xce7a, 0xdefa, 0x843a, 0x8c7a, 0x9cfa, 0xbdfa, 0x087a, 0x18fa, 0x39fa,
        0x7978, 0xbcbe, 0x793c, 0x791e, 0x38be, 0x79be, 0x78bc, 0x789e, 0x785e, 0xe0a8, 0xe0a4, 0xe0a2,
        0xc168, 0xe0b6, 0xc164, 0xc162, 0x82e8, 0xc176, 0x82e4, 0x82e2, 0x05e8, 0x82f6, 0x05e4, 0x05e2,
        0x05f6, 0xf0d4, 0x0d7e, 0xf0d2, 0xe094, 0xe1b4, 0xe092, 0xe1b2, 0xc134, 0xc374, 0xc132, 0xc372,
        0x8274, 0x86f4, 0x8272, 0x86f2, 0x04f4, 0x0df4, 0x04f2, 0x0df2, 0xf8ea, 0x1d7c, 0x1d3e, 0xf0ca,
        0xf1da, 0xe08a, 0xe19a, 0xe3ba, 0xc11a, 0xc33a, 0xc77a, 0x823a, 0x867a, 0x8efa, 0x047a, 0x0cfa,
        0x1dfa, 0x3d78, 0x9ebe, 0x3d3c, 0x3d1e, 0x1cbe, 0x3dbe, 0x7d70, 0xbebc, 0x7d38, 0xbe9e, 0x7d1c,
        0x7d0e, 0x3cbc, 0x7dbc, 0x3c9e, 0x7d9e, 0x7cb8, 0xbe5e, 0x7c9c, 0x7c8e, 0x3c5e, 0x7cde, 0x7c5c,
        0x7c4e, 0x7c2e, 0xc0b4, 0xc0b2, 0x8174, 0x8172, 0x02f4, 0x02f2, 0xe0da, 0xc09a, 0xc1ba, 0x813a,
        0x837a, 0x027a, 0x06fa, 0x0ebe, 0x1ebc, 0x1e9e, 0x3eb8, 0x9f5e, 0x3e9c, 0x3e8e, 0x1e5e, 0x3ede,
        0x7eb0, 0xbf5c, 0x7e98, 0xbf4e, 0x7e8c, 0x7e86, 0x3e5c, 0x7edc, 0x3e4e, 0x7ece, 0x7e58, 0xbf2e,
        0x7e4c, 0x7e46, 0x3e2e, 0x7e6e, 0x7e2c, 0x7e26, 0x0f5e, 0x1f5c, 0x1f4e, 0x3f58, 0x9fae, 0x3f4c,
        0x3f46, 0x1f2e, 0x3f6e, 0x3f2c, 0x3f26,
    ],
    [
        0xabe0, 0xd5f8, 0x53c0, 0xa9f0, 0xd4fc, 0x51e0, 0xa8f8, 0xd47e, 0x50f0, 0xa87c, 0x5078, 0xfad0,
        0x5be0, 0xadf8, 0xfac8, 0x59f0, 0xacfc, 0xfac4, 0x58f8, 0xac7e, 0xfac2, 0x587c, 0xf5d0, 0xfaec,
        0x5df8, 0xf5c8, 0xfae6, 0x5cfc, 0xf5c4, 0x5c7e, 0xf5c2, 0xebd0, 0xf5ec, 0xebc8, 0xf5e6, 0xebc4,
        0xebc2, 0xd7d0, 0xebec, 0xd7c8, 0xebe6, 0xd7c4, 0xd7c2, 0xafd0, 0xd7ec, 0xafc8, 0xd7e6, 0xafc4,
        0x4bc0, 0xa5f0, 0xd2fc, 0x49e0, 0xa4f8, 0xd27e, 0x48f0, 0xa47c, 0x4878, 0xa43e, 0x483c, 0xfa68,
        0x4df0, 0xa6fc, 0xfa64, 0x4cf8, 0xa67e, 0xfa62, 0x4c7c, 0x4c3e, 0xf4e8, 0xfa76, 0x4efc, 0xf4e4,
        0x4e7e, 0xf4e2, 0xe9e8, 0xf4f6, 0xe9e4, 0xe9e2, 0xd3e8, 0xe9f6, 0xd3e4, 0xd3e2, 0xa7e8, 0xd3f6,
        0xa7e4, 0xa7e2, 0x45e0, 0xa2f8, 0xd17e, 0x44f0, 0xa27c, 0x4478, 0xa23e, 0x443c, 0x441e, 0xfa34,
        0x46f8, 0xa37e, 0xfa32, 0x467c, 0x463e, 0xf474, 0x477e, 0xf472, 0xe8f4, 0xe8f2, 0xd1f4, 0xd1f2,
        0xa3f4, 0xa3f2, 0x42f0, 0xa17c, 0x4278, 0xa13e, 0x423c, 0x421e, 0xfa1a, 0x437c, 0x433e, 0xf43a,
        0xe87a, 0xd0fa, 0x4178, 0xa0be, 0x413c, 0x411e, 0x41be, 0x40bc, 0x409e, 0x2bc0, 0x95f0, 0xcafc,
        0x29e0, 0x94f8, 0xca7e, 0x28f0, 0x947c, 0x2878, 0x943e, 0x283c, 0xf968, 0x2df0, 0x96fc, 0xf964,
        0x2cf8, 0x967e, 0xf962, 0x2c7c, 0x2c3e, 0xf2e8, 0xf976, 0x2efc, 0xf2e4, 0x2e7e, 0xf2e2, 0xe5e8,
        0xf2f6, 0xe5e4, 0xe5e2, 0xcbe8, 0xe5f6, 0xcbe4, 0xcbe2, 0x97e8, 0xcbf6, 0x97e4, 0x97e2, 0xb5e0,
        0xdaf8, 0xed7e, 0x69c0, 0xb4f0, 0xda7c, 0x68e0, 0xb478, 0xda3e, 0x6870, 0xb43c, 0x6838, 0xb41e,
        0x681c, 0x25e0, 0x92f8, 0xc97e, 0x6de0, 0x24f0, 0x927c, 0x6cf0, 0xb67c, 0x923e, 0x6c78, 0x243c,
        0x6c3c, 0x241e, 0x6c1e, 0xf934, 0x26f8, 0x937e, 0xfb74, 0xf932, 0x6ef8, 0x267c, 0xfb72, 0x6e7c,
        0x263e, 0x6e3e, 0xf274, 0x277e, 0xf6f4, 0xf272, 0x6f7e, 0xf6f2, 0xe4f4, 0xedf4, 0xe4f2, 0xedf2,
        0xc9f4, 0xdbf4, 0xc9f2, 0xdbf2, 0x93f4, 0x93f2, 0x65c0, 0xb2f0, 0xd97c, 0x64e0, 0xb278, 0xd93e,
        0x6470, 0xb23c, 0x6438, 0xb21e, 0x641c, 0x640e, 0x22f0, 0x917c, 0x66f0, 0x2278, 0x913e, 0x6678,
        0xb33e, 0x663c, 0x221e, 0x661e, 0xf91a, 0x237c, 0xfb3a, 0x677c, 0x233e, 0x673e, 0xf23a, 0xf67a,
        0xe47a, 0xecfa, 0xc8fa, 0xd9fa, 0x91fa, 0x62e0, 0xb178, 0xd8be, 0x6270, 0xb13c, 0x6238, 0xb11e,
        0x621c, 0x620e, 0x2178, 0x90be, 0x6378, 0x213c, 0x633c, 0x211e, 0x631e, 0x21be, 0x63be, 0x6170,
        0xb0bc, 0x6138, 0xb09e, 0x611c, 0x610e, 0x20bc, 0x61bc, 0x209e, 0x619e, 0x60b8, 0xb05e, 0x609c,
        0x608e, 0x205e, 0x60de, 0x605c, 0x604e, 0x15e0, 0x8af8, 0xc57e, 0x14f0, 0x8a7c, 0x1478, 0x8a3e,
        0x143c, 0x141e, 0xf8b4, 0x16f8, 0x8b7e, 0xf8b2, 0x167c, 0x163e, 0xf174, 0x177e, 0xf172, 0xe2f4,
        0xe2f2, 0xc5f4, 0xc5f2, 0x8bf4, 0x8bf2, 0x35c0, 0x9af0, 0xcd7c, 0x34e0, 0x9a78, 0xcd3e, 0x3470,
        0x9a3c, 0x3438, 0x9a1e, 0x341c, 0x340e, 0x12f0, 0x897c, 0x36f0, 0x1278, 0x893e, 0x3678, 0x9b3e,
        0x363c, 0x121e, 0x361e, 0xf89a, 0x137c, 0xf9ba, 0x377c, 0x133e, 0x373e, 0xf13a, 0xf37a, 0xe27a,
        0xe6fa, 0xc4fa, 0xcdfa, 0x89fa, 0xbae0, 0xdd78, 0xeebe, 0x74c0, 0xba70, 0xdd3c, 0x7460, 0xba38,
        0xdd1e, 0x7430, 0xba1c, 0x7418, 0xba0e, 0x740c, 0x32e0, 0x9978, 0xccbe, 0x76e0, 0x3270, 0x993c,
        0x7670, 0xbb3c, 0x991e, 0x7638, 0x321c, 0x761c, 0x320e, 0x760e, 0x1178, 0x88be, 0x3378, 0x113c,
        0x7778, 0x333c, 0x111e, 0x773c, 0x331e, 0x771e, 0x11be, 0x33be, 0x77be, 0x72c0, 0xb970, 0xdcbc,
        0x7260, 0xb938, 0xdc9e, 0x7230, 0xb91c, 0x7218, 0xb90e, 0x720c, 0x7206, 0x3170, 0x98bc, 0x7370,
        0x3138, 0x989e, 0x7338, 0xb99e, 0x731c, 0x310e, 0x730e, 0x10bc, 0x31bc, 0x109e, 0x73bc, 0x319e,
        0x739e, 0x7160, 0xb8b8, 0xdc5e, 0x7130, 0xb89c, 0x7118, 0xb88e, 0x710c, 0x7106, 0x30b8, 0x985e,
        0x71b8, 0x309c, 0x719c, 0x308e, 0x718e, 0x105e, 0x30de, 0x71de, 0x70b0, 0xb85c, 0x7098, 0xb84e,
        0x708c, 0x7086, 0x305c, 0x70dc, 0x304e, 0x70ce, 0x7058, 0xb82e, 0x704c, 0x7046, 0x302e, 0x706e,
        0x702c, 0x7026, 0x0af0, 0x857c, 0x0a78, 0x853e, 0x0a3c, 0x0a1e, 0x0b7c, 0x0b3e, 0xf0ba, 0xe17a,
        0xc2fa, 0x85fa, 0x1ae0, 0x8d78, 0xc6be, 0x1a70, 0x8d3c, 0x1a38, 0x8d1e, 0x1a1c, 0x1a0e, 0x0978,
        0x84be, 0x1b78, 0x093c, 0x1b3c, 0x091e, 0x1b1e, 0x09be, 0x1bbe, 0x3ac0, 0x9d70, 0xcebc, 0x3a60,
        0x9d38, 0xce9e, 0x3a30, 0x9d1c, 0x3a18, 0x9d0e, 0x3a0c, 0x3a06, 0x1970, 0x8cbc, 0x3b70, 0x1938,
        0x8c9e, 0x3b38, 0x191c, 0x3b1c, 0x190e, 0x3b0e, 0x08bc, 0x19bc, 0x089e, 0x3bbc, 0x199e, 0x3b9e,
        0xbd60, 0xdeb8, 0xef5e, 0x7a40, 0xbd30, 0xde9c, 0x7a20, 0xbd18, 0xde8e, 0x7a10, 0xbd0c, 0x7a08,
        0xbd06, 0x7a04, 0x3960, 0x9cb8, 0xce5e, 0x7b60, 0x3930, 0x9c9c, 0x7b30, 0xbd9c, 0x9c8e, 0x7b18,
        0x390c, 0x7b0c, 0x3906, 0x7b06, 0x18b8, 0x8c5e, 0x39b8, 0x189c, 0x7bb8, 0x399c, 0x188e, 0x7b9c,
        0x398e, 0x7b8e, 0x085e, 0x18de, 0x39de, 0x7bde, 0x7940, 0xbcb0, 0xde5c, 0x7920, 0xbc98, 0xde4e,
        0x7910, 0xbc8c, 0x7908, 0xbc86, 0x7904, 0x7902, 0x38b0, 0x9c5c, 0x79b0, 0x3898, 0x9c4e, 0x7998,
        0xbcce, 0x798c, 0x3886, 0x7986, 0x185c, 0x38dc, 0x184e, 0x79dc, 0x38ce, 0x79ce, 0x78a0, 0xbc58,
        0xde2e, 0x7890, 0xbc4c, 0x7888, 0xbc46, 0x7884, 0x7882, 0x3858, 0x9c2e, 0x78d8, 0x384c, 0x78cc,
        0x3846, 0x78c6, 0x182e, 0x386e, 0x78ee, 0x7850, 0xbc2c, 0x7848, 0xbc26, 0x7844, 0x7842, 0x382c,
        0x786c, 0x3826, 0x7866, 0x7828, 0xbc16, 0x7824, 0x7822, 0x3816, 0x7836, 0x0578, 0x82be, 0x053c,
        0x051e, 0x05be, 0x0d70, 0x86bc, 0x0d38, 0x869e, 0x0d1c, 0x0d0e, 0x04bc, 0x0dbc, 0x049e, 0x0d9e,
        0x1d60, 0x8eb8, 0xc75e, 0x1d30, 0x8e9c, 0x1d18, 0x8e8e, 0x1d0c, 0x1d06, 0x0cb8, 0x865e, 0x1db8,
        0x0c9c, 0x1d9c, 0x0c8e, 0x1d8e, 0x045e, 0x0cde, 0x1dde, 0x3d40, 0x9eb0, 0xcf5c, 0x3d20, 0x9e98,
        0xcf4e, 0x3d10, 0x9e8c, 0x3d08, 0x9e86, 0x3d04, 0x3d02, 0x1cb0, 0x8e5c, 0x3db0, 0x1c98, 0x8e4e,
        0x3d98, 0x9ece, 0x3d8c, 0x1c86, 0x3d86, 0x0c5c, 0x1cdc, 0x0c4e, 0x3ddc, 0x1cce, 0x3dce, 0xbea0,
        0xdf58, 0xefae, 0xbe90, 0xdf4c, 0xbe88, 0xdf46, 0xbe84, 0xbe82, 0x3ca0, 0x9e58, 0xcf2e, 0x7da0,
        0x3c90, 0x9e4c, 0x7d90, 0xbecc, 0x9e46, 0x7d88, 0x3c84, 0x7d84, 0x3c82, 0x7d82, 0x1c58, 0x8e2e,
        0x3cd8, 0x1c4c, 0x7dd8, 0x3ccc, 0x1c46, 0x7dcc, 0x3cc6, 0x7dc6, 0x0c2e, 0x1c6e, 0x3cee, 0x7dee,
        0xbe50, 0xdf2c, 0xbe48, 0xdf26, 0xbe44, 0xbe42, 0x3c50, 0x9e2c, 0x7cd0, 0x3c48, 0x9e26, 0x7cc8,
        0xbe66, 0x7cc4, 0x3c42, 0x7cc2, 0x1c2c, 0x3c6c, 0x1c26, 0x7cec, 0x3c66, 0x7ce6, 0xbe28, 0xdf16,
        0xbe24, 0xbe22, 0x3c28, 0x9e16, 0x7c68, 0x3c24, 0x7c64, 0x3c22, 0x7c62, 0x1c16, 0x3c36, 0x7c76,
        0xbe14, 0xbe12, 0x3c14, 0x7c34, 0x3c12, 0x7c32, 0x02bc, 0x029e, 0x06b8, 0x835e, 0x069c, 0x068e,
        0x025e, 0x06de, 0x0eb0, 0x875c, 0x0e98, 0x874e, 0x0e8c, 0x0e86, 0x065c, 0x0edc, 0x064e, 0x0ece,
        0x1ea0, 0x8f58, 0xc7ae, 0x1e90, 0x8f4c, 0x1e88, 0x8f46, 0x1e84, 0x1e82, 0x0e58, 0x872e, 0x1ed8,
        0x8f6e, 0x1ecc, 0x0e46, 0x1ec6, 0x062e, 0x0e6e, 0x1eee, 0x9f50, 0xcfac, 0x9f48, 0xcfa6, 0x9f44,
        0x9f42, 0x1e50, 0x8f2c, 0x3ed0, 0x9f6c, 0x8f26, 0x3ec8, 0x1e44, 0x3ec4, 0x1e42, 0x3ec2, 0x0e2c,
        0x1e6c, 0x0e26, 0x3eec, 0x1e66, 0x3ee6, 0xdfa8, 0xefd6, 0xdfa4, 0xdfa2, 0x9f28, 0xcf96, 0xbf68,
        0x9f24, 0xbf64, 0x9f22, 0xbf62, 0x1e28, 0x8f16, 0x3e68, 0x1e24, 0x7ee8, 0x3e64, 0x1e22, 0x7ee4,
        0x3e62, 0x7ee2, 0x0e16, 0x1e36, 0x3e76, 0x7ef6, 0xdf94, 0xdf92, 0x9f14, 0xbf34, 0x9f12, 0xbf32,
        0x1e14, 0x3e34, 0x1e12, 0x7e74, 0x3e32, 0x7e72, 0xdf8a, 0x9f0a, 0xbf1a, 0x1e0a, 0x3e1a, 0x7e3a,
        0x035c, 0x034e, 0x0758, 0x83ae, 0x074c, 0x0746, 0x032e, 0x076e, 0x0f50, 0x87ac, 0x0f48, 0x87a6,
        0x0f44, 0x0f42, 0x072c, 0x0f6c, 0x0726, 0x0f66, 0x8fa8, 0xc7d6, 0x8fa4, 0x8fa2, 0x0f28, 0x8796,
        0x1f68, 0x8fb6, 0x1f64, 0x0f22, 0x1f62, 0x0716, 0x0f36, 0x1f76, 0xcfd4, 0xcfd2, 0x8f94, 0x9fb4,
        0x8f92, 0x9fb2, 0x0f14, 0x1f34, 0x0f12, 0x3f74, 0x1f32, 0x3f72, 0xcfca, 0x8f8a, 0x9f9a, 0x0f0a,
        0x1f1a, 0x3f3a, 0x03ac, 0x03a6, 0x07a8, 0x83d6, 0x07a4, 0x07a2, 0x0396, 0x07b6, 0x87d4, 0x87d2,
        0x0794, 0x0fb4, 0x0792, 0x0fb2, 0xc7ea,
    ],
];

#[cfg(test)]
mod tests {
    use super::*;
    use rxing::BarcodeFormat;

    /// Render to an 8-bit luma image (quiet zone, `sx` x `sy` pixels per module) and decode.
    fn decode(m: &Matrix, format: BarcodeFormat, sx: u32, sy: u32) -> Option<String> {
        let q = 4;
        let w = (m.width + 2 * q) * sx;
        let h = (m.height + 2 * q) * sy;
        let mut luma = vec![255u8; (w * h) as usize];
        for y in 0..m.height {
            for x in 0..m.width {
                if m.get(x, y) {
                    for py in 0..sy {
                        for px in 0..sx {
                            let (ix, iy) = ((x + q) * sx + px, (y + q) * sy + py);
                            luma[(iy * w + ix) as usize] = 0;
                        }
                    }
                }
            }
        }
        rxing::helpers::detect_in_luma(luma, w, h, Some(format))
            .ok()
            .map(|r| r.getText().to_string())
    }

    fn row_bits(m: &Matrix, y: u32, from: u32, len: u32) -> String {
        (from..from + len)
            .map(|x| if m.get(x, y) { '1' } else { '0' })
            .collect()
    }

    // ----- DataMatrix -----

    #[test]
    fn datamatrix_sizes() {
        let m = datamatrix(b"Hello", 0, 0, false, None).unwrap();
        assert_eq!((m.width, m.height), (12, 12));
        let m = datamatrix(b"123456", 0, 0, false, None).unwrap();
        assert_eq!((m.width, m.height), (10, 10));
        let m = datamatrix(b"Hello", 0, 0, true, None).unwrap();
        assert_eq!((m.width, m.height), (18, 8));
        // An explicit size is exact (as on Labelary): too small, or not a symbol of the
        // aspect ratio, prints nothing.
        let m = datamatrix(b"Hello", 24, 24, false, None).unwrap();
        assert_eq!((m.width, m.height), (24, 24));
        assert_eq!(datamatrix(&[b'A'; 100], 10, 10, false, None), Err(EncodeError::TooLong));
        assert_eq!(datamatrix(b"AB", 8, 8, false, None), Err(EncodeError::TooLong));
        assert_eq!(datamatrix(b"Hi", 12, 26, false, None), Err(EncodeError::TooLong));
        assert_eq!(datamatrix(b"Hi", 36, 16, true, None), Err(EncodeError::TooLong));
        let m = datamatrix(b"Hi", 12, 26, true, None).unwrap();
        assert_eq!((m.width, m.height), (26, 12));
        let m = datamatrix(b"RECT8", 8, 18, true, None).unwrap();
        assert_eq!((m.width, m.height), (18, 8));
        // Rows or columns alone.
        let m = datamatrix(b"ROWS ONLY", 16, 0, false, None).unwrap();
        assert_eq!((m.width, m.height), (16, 16));
        let m = datamatrix(b"COLS ONLY", 0, 16, false, None).unwrap();
        assert_eq!((m.width, m.height), (16, 16));
        // Finder pattern: solid left column and bottom row.
        let m = datamatrix(b"Hello", 0, 0, false, None).unwrap();
        assert!((0..12).all(|y| m.get(0, y)));
        assert!((0..12).all(|x| m.get(x, 11)));
        assert_eq!(
            datamatrix(&[b'x'; 5000], 0, 0, false, None),
            Err(EncodeError::TooLong)
        );
    }

    #[test]
    fn datamatrix_roundtrip() {
        for data in [
            "Hello",
            "ABC-123 abc 0123456789",
            "https://example.com/path?q=1&r=2",
        ] {
            let m = datamatrix(data.as_bytes(), 0, 0, false, None).unwrap();
            assert_eq!(
                decode(&m, BarcodeFormat::DATA_MATRIX, 4, 4).as_deref(),
                Some(data)
            );
            let m = datamatrix(data.as_bytes(), 0, 0, true, None).unwrap();
            assert_eq!(
                decode(&m, BarcodeFormat::DATA_MATRIX, 4, 4).as_deref(),
                Some(data)
            );
        }
    }

    #[test]
    fn datamatrix_escapes() {
        let u = dm_unescape(b"_101012345_110ABC__x_d065_J", b'_');
        let mut expect = vec![DM_FNC1];
        expect.extend(b"01012345".iter().map(|&b| u16::from(b)));
        expect.push(DM_FNC1);
        expect.extend(b"10ABC_xA\x0a".iter().map(|&b| u16::from(b)));
        assert_eq!(u, expect);
        // GS1: leading FNC1 is codeword 232, embedded FNC1 decodes as GS.
        let m = datamatrix(b"_10101234567890128_110AB12", 0, 0, false, Some('_')).unwrap();
        assert_eq!(strict_dm(&m), b"0101234567890128\x1d10AB12");
    }

    /// Decode a DataMatrix module grid exactly (no image processing) with the `datamatrix` crate.
    fn strict_dm(m: &Matrix) -> Vec<u8> {
        datamatrix_ref::DataMatrix::decode(&m.modules, m.width as usize)
            .expect("strict DataMatrix decode")
    }

    #[test]
    fn datamatrix_all_sizes_strict() {
        for s in &DM_SIZES {
            let cap = usize::from(s.data);
            let mut inputs: Vec<Vec<u8>> = vec![
                b"A".to_vec(),
                (0..cap).map(|i| b'a' + (i % 26) as u8).collect(), // ASCII, exact fit
                (0..(cap - 1) / 2 * 3)
                    .map(|i| b'A' + (i % 26) as u8)
                    .collect(), // C40 near fit
                (0..cap * 2).map(|i| b'0' + (i % 10) as u8).collect(), // digit pairs, exact
                (0..cap.saturating_sub(3))
                    .map(|i| (i * 37 % 256) as u8)
                    .collect(), // Base 256
            ];
            if cap >= 6 {
                inputs.push((0..(cap - 4) / 2).map(|i| 200 + (i % 50) as u8).collect()); // upper shift
            }
            for data in inputs {
                let m = datamatrix(&data, s.rows, s.cols, s.rows != s.cols, None).unwrap();
                assert_eq!(
                    (m.height, m.width),
                    (s.rows, s.cols),
                    "{}x{} len {}",
                    s.rows,
                    s.cols,
                    data.len()
                );
                assert_eq!(strict_dm(&m), data, "{}x{}", s.rows, s.cols);
            }
        }
    }

    #[test]
    fn datamatrix_roundtrips_through_reference_decoder() {
        let samples: [&[u8]; 7] = [
            b"Hello",
            b"0123456789ABCDEFGHIJ-shipment 4711/2024 batch:XY12",
            b"(01)09501101530003(17)140704(10)AB-123",
            b"lowercase text only here please",
            b"MIXED Case 123 with some punctuation!?",
            b"\xe9\xe8\xff\x00binary\x01\x02\x03\x04\x05\x06\x07\x08",
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789ABCDEFGHIJKLMN",
        ];
        for data in samples {
            let ours = datamatrix(data, 0, 0, false, None).unwrap();
            assert_eq!(strict_dm(&ours), data);
        }
    }

    #[test]
    fn datamatrix_codewords_match_labelary() {
        // Data codewords read back from Labelary renders: C40/Text/X12/EDIFACT/Base 256
        // switches and the way each mode is closed.
        let cases: [(&[u8], bool, &[u8]); 7] = [
            (b"RECTANGLE AUTO", true, &[230, 196, 161, 208, 140, 128, 251, 21, 19, 210, 161, 254, 129]),
            (b"DATAMATRIX 200", false, &[230, 108, 146, 91, 159, 211, 47, 231, 191, 254, 130, 129]),
            (b"FT DATAMATRIX", false, &[230, 123, 236, 108, 146, 91, 159, 211, 47, 254, 89, 129]),
            (
                b"hello world lowercase text",
                false,
                &[239, 134, 42, 160, 164, 229, 128, 158, 236, 160, 197, 117, 105, 92, 147, 23, 251, 236, 105, 254, 129],
            ),
            (
                b"ABC*DEF>GHI*123>456",
                false,
                &[238, 89, 233, 8, 251, 119, 37, 134, 178, 32, 56, 13, 202, 254, 55, 129],
            ),
            (
                b"A.B.C/D:E;F.G/H:I;J",
                false,
                &[240, 6, 224, 174, 14, 241, 58, 23, 177, 174, 30, 242, 58, 39, 178, 159],
            ),
            (b"caf\xe9 cr\xe8me br\xfbl\xe9e", false, &[231, 61, 36, 184, 82, 107, 56, 16, 181, 192, 219, 105]),
        ];
        for (data, rectangular, expect) in cases {
            let input: Vec<u16> = data.iter().map(|&b| u16::from(b)).collect();
            let (_, cws) = dm_codewords(&input, 0, 0, rectangular).unwrap();
            assert_eq!(&cws[..expect.len()], expect, "{}", String::from_utf8_lossy(data));
        }
    }

    // ----- PDF417 -----

    #[test]
    fn pdf417_structure() {
        let m = pdf417(b"Hello, world!", Some(2), 3, 0, false).unwrap();
        assert_eq!(m.width, 17 + 17 + 3 * 17 + 17 + 18);
        assert!(m.height >= 3);
        for y in 0..m.height {
            assert_eq!(row_bits(&m, y, 0, 17), "11111111010101000");
            assert_eq!(row_bits(&m, y, m.width - 18, 18), "111111101000101001");
        }
        let t = pdf417(b"Hello, world!", Some(2), 3, 0, true).unwrap();
        assert_eq!(t.width, 17 + 17 + 3 * 17 + 1);
        assert_eq!(t.height, m.height);
        for y in 0..t.height {
            assert_eq!(row_bits(&t, y, 0, 17), "11111111010101000");
            assert_eq!(row_bits(&t, y, t.width - 2, 2), "01");
        }
    }

    #[test]
    fn pdf417_dimensions() {
        // The requested grid is honoured when large enough and grown otherwise.
        let m = pdf417(b"ABC", Some(0), 5, 10, false).unwrap();
        assert_eq!((m.width, m.height), (69 + 5 * 17, 10));
        let m = pdf417(&[b'7'; 600], Some(5), 2, 3, false).unwrap();
        assert!(m.height <= 90 && m.height >= 3);
        let m = pdf417(b"Hello", None, 0, 0, false).unwrap();
        assert!(m.height >= 3);
        assert_eq!(
            pdf417(&[0xAAu8; 2000], None, 0, 0, false),
            Err(EncodeError::TooLong)
        );
    }

    #[test]
    fn pdf417_auto_grid_matches_labelary() {
        // (data, security) -> (data columns, rows), from Labelary's renders.
        let a = |n: usize| "A".repeat(n);
        let cases = [
            ("AB".to_string(), 3, (1, 18)),
            ("AB".to_string(), 4, (2, 17)),
            ("AB".to_string(), 5, (3, 22)),
            ("AB".to_string(), 6, (5, 26)),
            ("AB".to_string(), 7, (8, 33)),
            (a(30), 3, (2, 16)),
            (a(32), 3, (2, 17)),
            (a(62), 4, (3, 22)),
            ("SECURITY 1".to_string(), 1, (1, 11)),
            ("SECURITY 5".to_string(), 5, (3, 24)),
        ];
        for (data, security, (cols, rows)) in cases {
            let m = pdf417(data.as_bytes(), Some(security), 0, 0, false).unwrap();
            assert_eq!((m.width, m.height), (69 + 17 * cols, rows), "{data} s{security}");
        }
        // No security level is level 0: two error correction codewords.
        let m = pdf417(b"AB", None, 0, 0, false).unwrap();
        assert_eq!((m.width, m.height), (86, 4));
    }

    #[test]
    fn pdf417_compaction_matches_labelary() {
        // Data codewords (after the length descriptor) read off Labelary's renders.
        let cases: [(&str, &[u16]); 8] = [
            ("AB1234", &[1, 841, 63, 149]),
            ("AB1234567", &[1, 902, 13, 782, 767]),
            ("AB1234567CD", &[1, 841, 63, 125, 187, 842, 119]),
            ("AB12345678CD", &[1, 902, 138, 628, 478, 900, 63]),
            ("1234567AB", &[841, 63, 125, 187, 840, 59]),
            ("123456", &[902, 1, 348, 256]),
            ("ab12345678", &[810, 59, 902, 138, 628, 478]),
            ("PDF417 TEST DATA 1234567890", &[453, 178, 121, 236, 859, 138, 596, 90, 570, 809, 902, 15, 369, 753, 190]),
        ];
        for (data, want) in cases {
            let mut cws = Vec::new();
            pdf_high_level(data.as_bytes(), &mut cws);
            assert_eq!(cws, want, "{data}");
        }
    }

    #[test]
    fn pdf417_ecc_generator() {
        assert_eq!(pdf_generator(0), &[27, 917]);
        assert_eq!(pdf_generator(1), &[522, 568, 723, 809]);
    }

    #[test]
    fn pdf417_roundtrip() {
        let long = "The quick brown fox jumps over the lazy dog 1234567890123456 \
                    MIXED case; punct!? #$% end.";
        let cases: [(&str, Option<u8>, u32, u32); 6] = [
            ("Hello, world!", None, 0, 0),
            ("12345678901234567890123", Some(3), 4, 0),
            ("abc\x01\x02xyz bytes!", Some(1), 0, 6),
            (long, None, 0, 0),
            (long, Some(5), 8, 0),
            ("A", Some(0), 1, 3),
        ];
        for (data, sec, cols, rows) in cases {
            let m = pdf417(data.as_bytes(), sec, cols, rows, false).unwrap();
            assert_eq!(
                decode(&m, BarcodeFormat::PDF_417, 3, 9).as_deref(),
                Some(data),
                "{data:?} {sec:?} {cols} {rows}"
            );
        }
    }

    // ----- Aztec -----

    #[test]
    fn aztec_sizes() {
        let m = aztec(b"Hello", 0).unwrap();
        assert_eq!((m.width, m.height), (15, 15));
        let m = aztec(b"Hello", 104).unwrap();
        assert_eq!(m.width, 27);
        let m = aztec(b"Hello", 201).unwrap();
        assert_eq!(m.width, 19);
        let m = aztec(b"Hello", 215).unwrap();
        assert_eq!(m.width, 14 + 15 * 4 + 1 + 2 * ((37 - 1) / 15));
        assert_eq!(aztec(b"Hello", 300).unwrap().width, 11);
        assert_eq!(aztec(b"", 0), Err(EncodeError::InvalidData));
        assert_eq!(aztec(&[0u8; 5000], 0), Err(EncodeError::TooLong));
    }

    /// `bits` as 0/1 text, `n` bits per group, for readable expectations.
    fn groups(bits: &[bool], n: &[usize]) -> Vec<String> {
        let mut out = Vec::new();
        let mut at = 0;
        for &len in n {
            out.push(bits[at..at + len].iter().map(|&b| if b { '1' } else { '0' }).collect());
            at += len;
        }
        out
    }

    #[test]
    fn aztec_modes_follow_zint() {
        // Labelary keeps the space in upper and latches to digit after it (ZXing's shortest
        // path would take the space in digit mode).
        let bits = aztec_high_level(b"C 1");
        assert_eq!(groups(&bits, &[5, 5, 5, 4]), ["00100", "00001", "11110", "0011"]);
        // Two capitals in lower latch through mixed; one is shifted.
        let bits = aztec_high_level(b"aBCa");
        assert_eq!(groups(&bits, &[5, 5, 5, 5, 5, 5, 5, 5]), ["11100", "00010", "11101", "11101", "00011", "00100", "11100", "00010"]);
        let bits = aztec_high_level(b"aBa");
        assert_eq!(groups(&bits, &[5, 5, 5, 5, 5]), ["11100", "00010", "11100", "00011", "00010"]);
    }

    #[test]
    fn aztec_default_size_follows_zint() {
        // Labelary: 14 characters (70 bits) need 2 compact layers (Zint: < 66 bits for one).
        assert_eq!(aztec(b"AZTEC ROTATION", 0).unwrap().width, 19);
        assert_eq!(aztec(b"AZTEC ROTAT", 0).unwrap().width, 15);
    }

    #[test]
    fn aztec_rune_core() {
        let rune = aztec(b"RUNE", 300).unwrap();
        assert_eq!((rune.width, rune.height), (11, 11));
        // Bull's eye: dark centre and ring at 2, light ring at 1, orientation mark top-left.
        assert!(rune.get(5, 5) && !rune.get(5, 4) && rune.get(5, 3) && rune.get(0, 0));
        assert_ne!(aztec(b"7", 300).unwrap(), rune);
    }

    #[test]
    fn aztec_roundtrip() {
        let long: String = (0..400)
            .map(|i| (b'!' + (i * 7 % 90) as u8) as char)
            .collect();
        let cases: Vec<(String, u32)> = vec![
            ("Hello".into(), 0),
            ("Hello, World! 0123456789".into(), 0),
            ("MiXeD cAsE with @ ~ | and \r\n lines".into(), 0),
            ("1234567890 99.5, 7 ABC def".into(), 50),
            ("Code 2D!".into(), 103),
            ("Full range symbol".into(), 205),
            ("bin\x00\x10\x7f\x1b data".into(), 0),
            (long, 0),
            ("A".repeat(1500), 0),
        ];
        for (data, ecc) in cases {
            let m = aztec(data.as_bytes(), ecc).unwrap();
            assert_eq!(
                decode(&m, BarcodeFormat::AZTEC, 4, 4).as_deref(),
                Some(data.as_str()),
                "{data:?} ecc {ecc} size {}",
                m.width
            );
        }
    }

    /// Pseudo-random round trips over printable/control ASCII for all three symbologies.
    #[test]
    fn fuzz_roundtrip() {
        let mut seed = 0x2545_f491_4f6c_dd1du64;
        let mut rnd = move |n: u64| {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed % n
        };
        const ALPHABETS: [&[u8]; 4] = [
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz",
            b"0123456789 .,ABCabc-/:",
            b"!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ aZ09\r\n\t",
            b"\x01\x02\x1b\x7fAa1 .",
        ];
        for case in 0..150 {
            let len = 1 + rnd(if case % 10 == 0 { 300 } else { 60 }) as usize;
            let data: Vec<u8> = (0..len)
                .map(|_| {
                    let a = ALPHABETS[rnd(4) as usize];
                    a[rnd(a.len() as u64) as usize]
                })
                .collect();
            let text = String::from_utf8(data.clone()).unwrap();
            let m = aztec(&data, 0).unwrap();
            assert_eq!(
                decode(&m, BarcodeFormat::AZTEC, 3, 3).as_deref(),
                Some(text.as_str()),
                "aztec {data:?}"
            );
            let m = pdf417(&data, None, 0, 0, false).unwrap();
            assert_eq!(
                decode(&m, BarcodeFormat::PDF_417, 2, 6).as_deref(),
                Some(text.as_str()),
                "pdf417 {data:?}"
            );
            let m = datamatrix(&data, 0, 0, case % 2 == 0, None).unwrap();
            assert_eq!(
                decode(&m, BarcodeFormat::DATA_MATRIX, 3, 3).as_deref(),
                Some(text.as_str()),
                "dm {data:?}"
            );
            assert_eq!(strict_dm(&m), data);
        }
    }

    #[test]
    fn binary_roundtrip() {
        // Bytes >= 0x80 are decoded as ISO-8859-1 by the reference decoder.
        let data: Vec<u8> = (0u8..=255).rev().chain(*b"tail ABC 123").collect();
        let text: String = data.iter().map(|&b| b as char).collect();
        let m = aztec(&data, 0).unwrap();
        assert_eq!(decode(&m, BarcodeFormat::AZTEC, 3, 3), Some(text.clone()));
        let m = pdf417(&data, None, 0, 0, false).unwrap();
        assert_eq!(decode(&m, BarcodeFormat::PDF_417, 2, 6), Some(text.clone()));
        let m = pdf417(&data[..240], Some(4), 10, 0, true).unwrap();
        let t: String = data[..240].iter().map(|&b| b as char).collect();
        assert_eq!(decode(&m, BarcodeFormat::PDF_417, 2, 6), Some(t));
    }

    #[test]
    fn aztec_all_fixed_layers() {
        let data = b"ZPL Aztec 0123 test";
        for p in (101..=104).chain(201..=232) {
            let m = aztec(data, p).unwrap();
            assert_eq!(
                decode(&m, BarcodeFormat::AZTEC, 2, 2).as_deref(),
                Some("ZPL Aztec 0123 test"),
                "{p}"
            );
        }
    }

    #[test]
    #[ignore = "timing sanity check; run with --ignored --nocapture in release"]
    fn timing() {
        let data = b"0123456789ABCDEFGHIJ-shipment 4711/2024 batch:XY12";
        for _ in 0..500 {
            let _ = std::hint::black_box((
                datamatrix(data, 0, 0, false, None),
                pdf417(data, None, 0, 0, false),
                aztec(data, 0),
            ));
        }
        let n = 5000;
        let t = std::time::Instant::now();
        for _ in 0..n {
            std::hint::black_box(
                datamatrix(std::hint::black_box(data), 0, 0, false, None).unwrap(),
            );
        }
        println!("datamatrix: {:?}/symbol", t.elapsed() / n);
        let t = std::time::Instant::now();
        for _ in 0..n {
            std::hint::black_box(pdf417(std::hint::black_box(data), None, 0, 0, false).unwrap());
        }
        println!("pdf417:     {:?}/symbol", t.elapsed() / n);
        let t = std::time::Instant::now();
        for _ in 0..n {
            std::hint::black_box(aztec(std::hint::black_box(data), 0).unwrap());
        }
        println!("aztec:      {:?}/symbol", t.elapsed() / n);
    }
}
