//! ASCII hex graphic data (^GF A, ~DG, ~DY), including ZPL's ASCII compression:
//! - `G`-`Y` repeat the next hex digit 1-19 times, `g`-`z` add 20-400 (combinable)
//! - `,` fills the rest of the row with `0`
//! - `!` fills the rest of the row with `F`
//! - `:` repeats the previous row
//!
//! Anything else (whitespace, line breaks) is skipped. Labels carry tens of kilobytes of
//! this, so it is decoded byte by byte through one lookup table straight into the output.

use std::borrow::Cow;

/// Parameters `from..` joined back with ',' (graphic data may contain commas). The tokenizer
/// hands raw data over as one parameter, which is borrowed rather than copied.
pub(super) fn data_param<'p>(params: &[&'p str], from: usize) -> Cow<'p, str> {
    match params.get(from..).unwrap_or_default() {
        [] => Cow::Borrowed(""),
        [one] => Cow::Borrowed(one),
        rest => Cow::Owned(rest.join(",")),
    }
}

/// `data` without whitespace, for base64 wrapped over several lines.
pub(super) fn without_whitespace(data: &str) -> Cow<'_, str> {
    if data.contains(char::is_whitespace) {
        Cow::Owned(data.chars().filter(|c| !c.is_whitespace()).collect())
    } else {
        Cow::Borrowed(data)
    }
}

/// What an input byte does: a hex digit (0-15), a repeat count, a row command or nothing.
const SKIP: u16 = 0xFFFF;
const ROW_ZEROS: u16 = 0xFFF0;
const ROW_ONES: u16 = 0xFFF1;
const ROW_REPEAT: u16 = 0xFFF2;
/// Repeat counts are `REPEAT | n`, n <= 400.
const REPEAT: u16 = 0x1000;

const CLASS: [u16; 256] = {
    let mut table = [SKIP; 256];
    let mut i = 0;
    while i < 10 {
        table[b'0' as usize + i] = i as u16;
        i += 1;
    }
    let mut i = 0;
    while i < 6 {
        table[b'A' as usize + i] = 10 + i as u16;
        table[b'a' as usize + i] = 10 + i as u16;
        i += 1;
    }
    // G-Y: 1-19, g-z: 20-400.
    let mut i = 0;
    while i < 20 {
        if i < 19 {
            table[b'G' as usize + i] = REPEAT | (i as u16 + 1);
        }
        table[b'g' as usize + i] = REPEAT | ((i as u16 + 1) * 20);
        i += 1;
    }
    table[b',' as usize] = ROW_ZEROS;
    table[b'!' as usize] = ROW_ONES;
    table[b':' as usize] = ROW_REPEAT;
    table
};

/// Output being built a nibble at a time. A half-written byte is the last one in `bytes`,
/// with its low nibble still zero.
struct Nibbles {
    bytes: Vec<u8>,
    half: bool,
}

impl Nibbles {
    #[inline]
    fn len(&self) -> usize {
        self.bytes.len() * 2 - self.half as usize
    }

    /// Appends `count` (>= 1) copies of nibble `value`.
    #[inline]
    fn push_run(&mut self, value: u8, mut count: usize) {
        if self.half {
            *self.bytes.last_mut().unwrap() |= value;
            self.half = false;
            count -= 1;
        }
        self.bytes.resize(self.bytes.len() + count / 2, value * 0x11);
        if count % 2 == 1 {
            self.bytes.push(value << 4);
            self.half = true;
        }
    }

    /// Fills up to nibble `end` (> len) with `value`.
    #[inline]
    fn fill_to(&mut self, end: usize, value: u8) {
        let count = end - self.len();
        self.push_run(value, count);
    }
}

/// Decodes ASCII hex graphic data with `bytes_per_row` bytes per row (0: no row commands).
/// `size_hint` is the declared byte count; a trailing odd nibble is dropped.
pub(super) fn decode_ascii_hex(data: &[u8], bytes_per_row: usize, size_hint: usize) -> Vec<u8> {
    let row_len = bytes_per_row * 2;
    // At most one row (`,` `!` `:`) or 400 nibbles per input byte: never reserve more than
    // the data could expand to, whatever the label declares.
    let bound = data.len().saturating_mul(bytes_per_row.max(200));
    let mut out = Nibbles { bytes: Vec::with_capacity(size_hint.min(bound)), half: false };
    let mut repeat: usize = 0;
    let mut i = 0;

    while i < data.len() {
        let class = CLASS[data[i] as usize];
        i += 1;
        if class < 16 {
            // Plain digit pairs on a byte boundary are the common case.
            if repeat == 0
                && !out.half
                && let Some(&next) = data.get(i)
                && let low = CLASS[next as usize]
                && low < 16
            {
                out.bytes.push((class as u8) << 4 | low as u8);
                i += 1;
                continue;
            }
            out.push_run(class as u8, repeat.max(1));
            repeat = 0;
            continue;
        }
        match class {
            SKIP => {}
            ROW_ZEROS | ROW_ONES if row_len > 0 => {
                let start = out.len() / row_len * row_len;
                out.fill_to(start + row_len, if class == ROW_ZEROS { 0x0 } else { 0xF });
                repeat = 0;
            }
            ROW_REPEAT if row_len > 0 => {
                let len = out.len();
                let mut start = len / row_len * row_len;
                if len > start {
                    out.fill_to(start + row_len, 0);
                    start += row_len;
                }
                // Rows are whole bytes, so the previous row is `bytes_per_row` whole bytes.
                let start = start / 2;
                if start >= bytes_per_row {
                    out.bytes.extend_from_within(start - bytes_per_row..start);
                } else {
                    out.bytes.resize(bytes_per_row, 0);
                }
                repeat = 0;
            }
            ROW_ZEROS | ROW_ONES | ROW_REPEAT => {}
            _ => repeat += (class & !REPEAT) as usize,
        }
    }

    let mut bytes = out.bytes;
    if out.half {
        bytes.pop();
    }
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The previous decoder, kept as the reference: one nibble per element.
    fn reference(hex_str: &str, bytes_per_row: u32) -> Vec<u8> {
        let row_len = (bytes_per_row as usize) * 2;
        let mut nibbles: Vec<u8> = Vec::with_capacity(hex_str.len() * 2);
        let row_start = |len: usize| len / row_len * row_len;
        let mut repeat: usize = 0;

        for &c in hex_str.as_bytes() {
            match c {
                b'G'..=b'Y' => repeat += (c - b'F') as usize,
                b'g'..=b'z' => repeat += (c - b'f') as usize * 20,
                b',' | b'!' if row_len > 0 => {
                    let fill = if c == b',' { 0x0 } else { 0xF };
                    let start = row_start(nibbles.len());
                    nibbles.resize(start + row_len, fill);
                    repeat = 0;
                }
                b':' if row_len > 0 => {
                    let mut start = row_start(nibbles.len());
                    if nibbles.len() > start {
                        nibbles.resize(start + row_len, 0);
                        start += row_len;
                    }
                    if start >= row_len {
                        nibbles.extend_from_within(start - row_len..start);
                    } else {
                        nibbles.resize(row_len, 0);
                    }
                    repeat = 0;
                }
                _ => {
                    let value = match c {
                        b'0'..=b'9' => c - b'0',
                        b'A'..=b'F' => c - b'A' + 10,
                        b'a'..=b'f' => c - b'a' + 10,
                        _ => continue,
                    };
                    nibbles.resize(nibbles.len() + repeat.max(1), value);
                    repeat = 0;
                }
            }
        }

        nibbles.chunks_exact(2).map(|pair| (pair[0] << 4) | pair[1]).collect()
    }

    fn check(data: &str, bytes_per_row: u32) {
        let expected = reference(data, bytes_per_row);
        for hint in [0, expected.len(), usize::MAX] {
            assert_eq!(decode_ascii_hex(data.as_bytes(), bytes_per_row as usize, hint), expected, "{data:?} / {bytes_per_row}");
        }
    }

    /// xorshift64*: deterministic and dependency free.
    struct Rng(u64);

    impl Rng {
        fn next(&mut self) -> u64 {
            self.0 ^= self.0 >> 12;
            self.0 ^= self.0 << 25;
            self.0 ^= self.0 >> 27;
            self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
        }

        fn below(&mut self, n: u64) -> u64 {
            self.next() % n
        }
    }

    /// One random token of compressed data: digits in either case, repeat prefixes (also
    /// summing past 400), row commands, whitespace and stray bytes.
    fn token(rng: &mut Rng, out: &mut String) {
        const HEX: &[u8] = b"0123456789ABCDEFabcdef";
        match rng.below(20) {
            0..=7 => out.push(HEX[rng.below(HEX.len() as u64) as usize] as char),
            8..=10 => {
                for _ in 0..1 + rng.below(3) {
                    let c = if rng.below(2) == 0 { b'G' + rng.below(19) as u8 } else { b'g' + rng.below(20) as u8 };
                    out.push(c as char);
                }
                out.push(HEX[rng.below(HEX.len() as u64) as usize] as char);
            }
            11 | 12 => out.push(','),
            13 => out.push('!'),
            14 | 15 => out.push(':'),
            16 => out.push(['\n', '\r', ' ', '\t'][rng.below(4) as usize]),
            17 => out.push(['Z', 'z', 'x', '.', '~', 'é'][rng.below(6) as usize]),
            // A repeat prefix with nothing to repeat before a row command.
            18 => out.push_str(["G,", "h!", "Y:", "zz:"][rng.below(4) as usize]),
            _ => {
                for _ in 0..rng.below(40) {
                    out.push(HEX[rng.below(16) as usize] as char);
                }
            }
        }
    }

    #[test]
    fn matches_reference_on_random_data() {
        let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
        for _ in 0..20_000 {
            let mut data = String::new();
            for _ in 0..rng.below(80) {
                token(&mut rng, &mut data);
            }
            check(&data, rng.below(12) as u32);
        }
    }

    #[test]
    fn edge_cases() {
        for bytes_per_row in [0, 1, 2, 3, 5] {
            for data in [
                "",
                "F",
                "ABC",
                "abc,",
                ",",
                ",,",
                ":",
                "::",
                "F:",
                "FF::",
                "F,:",
                "!F",
                "A,B!C:D",
                "zzzzA",
                "zzzzzzzzzzzzzzzzzzzzzzzzA,",
                "GA",
                "GAB",
                "HA:",
                "IF!:",
                "0 1\n2\r\n3\t4",
                "ffFFaa00",
                "G",
                "zz",
                "Yz",
            ] {
                check(data, bytes_per_row);
            }
        }
    }
}
