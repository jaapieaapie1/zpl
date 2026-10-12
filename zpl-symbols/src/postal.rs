//! Height-modulated postal barcodes: POSTNET, PLANET and the USPS Intelligent Mail barcode
//! (IMb, a.k.a. OneCode / 4-State Customer Barcode).
//!
//! ZPL entry points:
//! - `^BZo,h,f,g,t`: `t = 0` POSTNET, `t = 1` PLANET, `t = 2` reserved, `t = 3` USPS
//!   Intelligent Mail barcode (see [`PostalKind::from_bz_type`]).
//! - `^B5o,h,f,g`: PLANET Code.
//!
//! In both commands `h` is the full (tallest) bar height in dots (default `^BY` height),
//! `f = Y` prints the interpretation line ([`PostalSymbol::hri`]) and `g = Y` puts it above
//! the symbol instead of below.
//!
//! Geometry: every bar is a [`PostalBar`] whose `top`/`bottom` are fractions of `h`. Bars are
//! all equally wide and laid out at a fixed pitch; see [`PostalSymbol::bar_width`] and
//! [`PostalSymbol::pitch`].

use crate::{PostalBar, SymbolError};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PostalKind {
    /// POSTNET: 5 (ZIP), 9 (ZIP+4) or 11 (ZIP+4+delivery point) digits; a mod-10 check digit
    /// is appended.
    Postnet,
    /// PLANET Code: 11 or 13 digits; a mod-10 check digit is appended.
    Planet,
    /// USPS Intelligent Mail barcode (USPS-B-3200): 20-digit tracking code followed by a
    /// 0/5/9/11-digit routing (ZIP) code, i.e. 20, 25, 29 or 31 digits. A `-` (or space)
    /// may separate tracking and routing code.
    UspsImb,
}

impl PostalKind {
    /// Maps the `t` parameter of `^BZ` (0 POSTNET, 1 PLANET, 2 reserved, 3 IMb).
    pub fn from_bz_type(t: u32) -> Option<PostalKind> {
        match t {
            0 => Some(PostalKind::Postnet),
            1 => Some(PostalKind::Planet),
            3 => Some(PostalKind::UspsImb),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PostalSymbol {
    /// Bars from left to right.
    pub bars: Vec<PostalBar>,
    /// Width of one bar, in bar-width units: always `1.0`. The renderer draws each bar
    /// `^BY` module width dots wide.
    pub bar_width: f32,
    /// Distance from the left edge of one bar to the left edge of the next, in bar widths.
    /// USPS fixes the physical layout (bar width 0.020", 22 bars per inch, i.e. pitch
    /// 0.04545") for all three symbologies, so this is `(1/22) / 0.020 = 2.2727`; a renderer
    /// with module width `w` dots places bar `i` at `x = round(i * pitch * w)` and draws it
    /// `w` dots wide (at 203 dpi, `w = 4` gives 0.020" bars and 0.045" pitch).
    pub pitch: f32,
    /// Interpretation line text (the digits as encoded, without spaces or separators;
    /// POSTNET/PLANET include the computed check digit).
    pub hri: Option<String>,
}

/// USPS nominal bar pitch (22 bars/inch) divided by the nominal bar width (0.020").
const PITCH: f32 = (1.0 / 22.0) / 0.020;

/// POSTNET/PLANET (DMM 708.4): full bar 0.125", half bar 0.050" — the half bar is 0.4 of the
/// full bar, bottom aligned.
const POSTNET_FULL: PostalBar = PostalBar {
    top: 0.0,
    bottom: 1.0,
};
const POSTNET_HALF: PostalBar = PostalBar {
    top: 0.6,
    bottom: 1.0,
};

/// IMb (USPS-B-3200 §2.3.1, mid-tolerance values): overall height 0.145" (0.125–0.165"),
/// tracker 0.048" (0.039–0.057") centred, ascender/descender extending 0.0485" beyond it.
const IMB_EXT: f32 = 0.0485 / 0.145;
const IMB_FULL: PostalBar = PostalBar {
    top: 0.0,
    bottom: 1.0,
};
const IMB_ASCENDER: PostalBar = PostalBar {
    top: 0.0,
    bottom: 1.0 - IMB_EXT,
};
const IMB_DESCENDER: PostalBar = PostalBar {
    top: IMB_EXT,
    bottom: 1.0,
};
const IMB_TRACKER: PostalBar = PostalBar {
    top: IMB_EXT,
    bottom: 1.0 - IMB_EXT,
};

pub fn encode(kind: PostalKind, data: &str) -> Result<PostalSymbol, SymbolError> {
    match kind {
        PostalKind::Postnet => encode_postnet_planet(data, &[5, 9, 11], false),
        PostalKind::Planet => encode_postnet_planet(data, &[11, 13], true),
        PostalKind::UspsImb => encode_imb(data),
    }
}

fn digits(data: &str) -> Result<Vec<u8>, SymbolError> {
    data.chars()
        .map(|c| {
            c.to_digit(10)
                .map(|d| d as u8)
                .ok_or(SymbolError::InvalidCharacter(c))
        })
        .collect()
}

/// POSTNET digit patterns (weights 7,4,2,1,0; `true` = full bar). PLANET is the inverse.
const POSTNET_TABLE: [[bool; 5]; 10] = {
    const T: bool = true;
    const F: bool = false;
    [
        [T, T, F, F, F],
        [F, F, F, T, T],
        [F, F, T, F, T],
        [F, F, T, T, F],
        [F, T, F, F, T],
        [F, T, F, T, F],
        [F, T, T, F, F],
        [T, F, F, F, T],
        [T, F, F, T, F],
        [T, F, T, F, F],
    ]
};

fn encode_postnet_planet(
    data: &str,
    lengths: &[usize],
    planet: bool,
) -> Result<PostalSymbol, SymbolError> {
    let mut d = digits(data)?;
    if !lengths.contains(&d.len()) {
        return Err(SymbolError::InvalidLength(d.len()));
    }
    let sum: u32 = d.iter().map(|&x| x as u32).sum();
    d.push(((10 - sum % 10) % 10) as u8);

    let mut bars = Vec::with_capacity(d.len() * 5 + 2);
    bars.push(POSTNET_FULL);
    for &digit in &d {
        for &tall in &POSTNET_TABLE[digit as usize] {
            bars.push(if tall != planet {
                POSTNET_FULL
            } else {
                POSTNET_HALF
            });
        }
    }
    bars.push(POSTNET_FULL);
    Ok(PostalSymbol {
        bars,
        bar_width: 1.0,
        pitch: PITCH,
        hri: Some(d.iter().map(|&x| char::from(b'0' + x)).collect()),
    })
}

// ---------------------------------------------------------------------------------------
// USPS Intelligent Mail barcode (USPS-B-3200 Rev H)
// ---------------------------------------------------------------------------------------

/// Appendix E, "N of 13" table generation: 13-bit values with `n` bits set; values come in
/// pairs with their bit reversal from the front, palindromes are filled from the back.
const fn n_of_13_table<const LEN: usize>(n: u32) -> [u16; LEN] {
    let mut table = [0u16; LEN];
    let mut lower = 0;
    let mut upper = LEN - 1;
    let mut count: u16 = 0;
    while count < 8192 {
        if count.count_ones() == n {
            let reverse = count.reverse_bits() >> 3;
            if reverse >= count {
                if reverse == count {
                    table[upper] = count;
                    upper = upper.wrapping_sub(1);
                } else {
                    table[lower] = count;
                    table[lower + 1] = reverse;
                    lower += 2;
                }
            }
        }
        count += 1;
    }
    table
}

static TABLE_5_OF_13: [u16; 1287] = n_of_13_table::<1287>(5);
static TABLE_2_OF_13: [u16; 78] = n_of_13_table::<78>(2);

/// Appendix E Table 22 (Bar to Character Mapping) as a reverse lookup: entry `13 * c + b`
/// is the extender driven by bit `b` of character `c` (A = 0 .. J = 9). Values 1..=65 are
/// the descender of bar 1..=65, 66..=130 the ascender of bar 1..=65.
const BAR_MAP: [u8; 130] = [
    67, 6, 78, 16, 86, 95, 34, 40, 45, 113, 117, 121, 62, //
    87, 18, 104, 41, 76, 57, 119, 115, 72, 97, 2, 127, 26, //
    105, 35, 122, 52, 114, 7, 24, 82, 68, 63, 94, 44, 77, //
    112, 70, 100, 39, 30, 107, 15, 125, 85, 10, 65, 54, 88, //
    20, 106, 46, 66, 8, 116, 29, 61, 99, 80, 90, 37, 123, //
    51, 25, 84, 129, 56, 4, 109, 96, 28, 36, 47, 11, 71, //
    33, 102, 21, 9, 17, 49, 124, 79, 64, 91, 42, 69, 53, //
    60, 14, 1, 27, 103, 126, 75, 89, 50, 120, 19, 32, 110, //
    92, 111, 130, 59, 31, 12, 81, 43, 55, 5, 74, 22, 101, //
    128, 58, 118, 48, 108, 38, 98, 93, 23, 83, 13, 73, 3, //
];

/// Appendix D CRC-11 frame check sequence over the 102-bit binary value.
fn imb_crc11(value: u128) -> u16 {
    const POLY: u16 = 0x0F35;
    let bytes = value.to_be_bytes(); // 16 bytes; the 13 low ones hold the 102 bits
    let bytes = &bytes[3..];
    let mut fcs: u16 = 0x07FF;
    for (i, &byte) in bytes.iter().enumerate() {
        // The two most significant bits of the first byte are not part of the data.
        let (mut data, skip) = (u16::from(byte) << 3, if i == 0 { 2 } else { 0 });
        data <<= skip;
        for _ in skip..8 {
            fcs = if (fcs ^ data) & 0x400 != 0 {
                (fcs << 1) ^ POLY
            } else {
                fcs << 1
            };
            fcs &= 0x7FF;
            data <<= 1;
        }
    }
    fcs
}

/// The 65 bar states of an IMb: bit 0 = descender present, bit 1 = ascender present.
fn imb_states(tracking: &[u8], routing: &[u8]) -> Result<[u8; 65], SymbolError> {
    if tracking.len() != 20 {
        return Err(SymbolError::InvalidLength(tracking.len() + routing.len()));
    }
    if tracking[1] > 4 {
        return Err(SymbolError::InvalidData(
            "IMb barcode identifier second digit must be 0-4".into(),
        ));
    }
    // Step 1: binary data.
    let routing_value = routing.iter().fold(0u128, |v, &d| v * 10 + u128::from(d));
    let mut value: u128 = match routing.len() {
        0 => 0,
        5 => routing_value + 1,
        9 => routing_value + 100_000 + 1,
        11 => routing_value + 1_000_000_000 + 100_000 + 1,
        n => return Err(SymbolError::InvalidLength(20 + n)),
    };
    value = value * 10 + u128::from(tracking[0]);
    value = value * 5 + u128::from(tracking[1]);
    for &d in &tracking[2..] {
        value = value * 10 + u128::from(d);
    }

    // Step 2: frame check sequence.
    let fcs = imb_crc11(value);

    // Step 3: codewords (A most significant, J base 636, B..I base 1365).
    let mut cw = [0u32; 10];
    cw[9] = (value % 636) as u32;
    value /= 636;
    for c in cw[1..9].iter_mut().rev() {
        *c = (value % 1365) as u32;
        value /= 1365;
    }
    cw[0] = value as u32; // < 659 for every valid input

    // Step 4: orientation in J, FCS bit 10 in A.
    cw[9] *= 2;
    if fcs & 0x400 != 0 {
        cw[0] += 659;
    }

    // Step 5: characters, FCS bits 0..9 invert characters A..J.
    let mut chars = [0u16; 10];
    for (i, ch) in chars.iter_mut().enumerate() {
        let c = cw[i] as usize;
        *ch = if c < 1287 {
            TABLE_5_OF_13[c]
        } else {
            TABLE_2_OF_13[c - 1287]
        };
        if fcs & (1 << i) != 0 {
            *ch = !*ch & 0x1FFF;
        }
    }

    // Step 6: characters to bars.
    let mut states = [0u8; 65];
    for (i, &ch) in chars.iter().enumerate() {
        for bit in 0..13 {
            if ch & (1 << bit) != 0 {
                let pos = BAR_MAP[13 * i + bit] as usize - 1;
                if pos < 65 {
                    states[pos] |= 1;
                } else {
                    states[pos - 65] |= 2;
                }
            }
        }
    }
    Ok(states)
}

fn encode_imb(data: &str) -> Result<PostalSymbol, SymbolError> {
    let (tracking, routing) = match data.find(['-', ' ']) {
        Some(p) => (digits(&data[..p])?, digits(&data[p + 1..])?),
        None => {
            let d = digits(data)?;
            if d.len() < 20 {
                return Err(SymbolError::InvalidLength(d.len()));
            }
            let (t, r) = d.split_at(20);
            (t.to_vec(), r.to_vec())
        }
    };
    let states = imb_states(&tracking, &routing)?;
    let bars = states
        .iter()
        .map(|&s| match s {
            0 => IMB_TRACKER,
            1 => IMB_DESCENDER,
            2 => IMB_ASCENDER,
            _ => IMB_FULL,
        })
        .collect();
    let hri = tracking
        .iter()
        .chain(&routing)
        .map(|&x| char::from(b'0' + x))
        .collect();
    Ok(PostalSymbol {
        bars,
        bar_width: 1.0,
        pitch: PITCH,
        hri: Some(hri),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn postnet_pattern(sym: &PostalSymbol) -> String {
        sym.bars
            .iter()
            .map(|b| if *b == POSTNET_FULL { '1' } else { '0' })
            .collect()
    }

    fn imb_letters(data: &str) -> String {
        encode(PostalKind::UspsImb, data)
            .unwrap()
            .bars
            .iter()
            .map(|b| match *b {
                IMB_FULL => 'F',
                IMB_ASCENDER => 'A',
                IMB_DESCENDER => 'D',
                _ => 'T',
            })
            .collect()
    }

    #[test]
    fn postal_tables() {
        assert_eq!(&TABLE_5_OF_13[..4], &[0x001F, 0x1F00, 0x002F, 0x1E80]);
        assert_eq!(TABLE_5_OF_13[1286], 0x01F0); // palindrome 0000111110000 (B-3200 Fig. 3)
        assert_eq!(&TABLE_2_OF_13[..4], &[0x0003, 0x1800, 0x0005, 0x1400]);
        assert_eq!(TABLE_2_OF_13[77], 0x00A0);
        assert_eq!(TABLE_2_OF_13[7], 0x1200); // codeword 1294 -> character H of example 4
        let mut seen = [false; 130];
        for &b in &BAR_MAP {
            assert!(!seen[b as usize - 1]);
            seen[b as usize - 1] = true;
        }
    }

    #[test]
    fn postal_imb_crc() {
        // USPS-B-3200 Appendix C step 1/2 values.
        let v = u128::from_str_radix("016907B2A24ABC16A2E5C004B1", 16).unwrap();
        assert_eq!(imb_crc11(v), 0x751);
        let v = u128::from_str_radix("00000000001122103B5C2004B1", 16).unwrap();
        assert_eq!(imb_crc11(v), 0x051);
    }

    #[test]
    fn postal_imb_spec_examples() {
        // USPS-B-3200 Rev H Appendix C, Tables 13-16.
        let t = "01234567094987654321";
        assert_eq!(
            imb_letters(t),
            "ATTFATTDTTADTAATTDTDTATTDAFDDFADFDFTFFFFFTATFAAAATDFFTDAADFTFDTDT"
        );
        assert_eq!(
            imb_letters(&format!("{t}01234")),
            "DTTAFADDTTFTDTFTFDTDDADADAFADFATDDFTAAAFDTTADFAAATDFDTDFADDDTDFFT"
        );
        assert_eq!(
            imb_letters(&format!("{t}012345678")),
            "ADFTTAFDTTTTFATTADTAAATFTFTATDAAAFDDADATATDTDTTDFDTDATADADTDFFTFA"
        );
        assert_eq!(
            imb_letters(&format!("{t}-01234567891")),
            "AADTFFDFTDADTAADAATFDTDDAAADDTDTTDAFADADDDTFFFDDTTTADFAAADFTDAADA"
        );
    }

    #[test]
    fn postal_imb_errors_and_hri() {
        let t = "01234567094987654321";
        assert!(encode(PostalKind::UspsImb, &t[..19]).is_err());
        assert!(encode(PostalKind::UspsImb, &format!("{t}0123")).is_err());
        assert!(encode(PostalKind::UspsImb, "05234567094987654321").is_err());
        assert!(encode(PostalKind::UspsImb, "0123456709498765432X").is_err());
        let s = encode(PostalKind::UspsImb, &format!("{t}-01234")).unwrap();
        assert_eq!(s.bars.len(), 65);
        assert_eq!(s.hri.as_deref(), Some("0123456709498765432101234"));
        assert!((IMB_TRACKER.bottom - IMB_TRACKER.top - 0.048 / 0.145).abs() < 1e-4);
    }

    #[test]
    fn postal_postnet() {
        // 12345 -> sum 15 -> check 5. Frame, 1 2 3 4 5 5, frame.
        let s = encode(PostalKind::Postnet, "12345").unwrap();
        assert_eq!(s.hri.as_deref(), Some("123455"));
        assert_eq!(
            postnet_pattern(&s),
            format!(
                "1{}{}{}{}{}{}1",
                "00011", "00101", "00110", "01001", "01010", "01010"
            )
        );
        assert_eq!(s.bars.len(), 32);
        // Digit 0 = 11000; 9 digits -> 52 bars, 11 digits -> 62 bars.
        let s = encode(PostalKind::Postnet, "000000000").unwrap();
        assert_eq!(s.bars.len(), 52);
        assert!(postnet_pattern(&s).starts_with("111000"));
        // DMM example: ZIP+4+DPC 55555-1237-12 -> check digit 9 (sum 41).
        let s = encode(PostalKind::Postnet, "55555123712").unwrap();
        assert_eq!(s.bars.len(), 62);
        assert_eq!(s.hri.as_deref(), Some("555551237129"));
        assert!((POSTNET_HALF.bottom - POSTNET_HALF.top - 0.4).abs() < 1e-6);
        assert!(encode(PostalKind::Postnet, "1234").is_err());
        assert!(encode(PostalKind::Postnet, "1234A").is_err());
    }

    #[test]
    fn postal_planet() {
        let s = encode(PostalKind::Planet, "40123456789").unwrap();
        // sum 49 -> check 1
        assert_eq!(s.hri.as_deref(), Some("401234567891"));
        assert_eq!(s.bars.len(), 62);
        let p = postnet_pattern(&s);
        // Frame, 4 = inverse of 01001 = 10110, 0 = 00111.
        assert!(p.starts_with("11011000111"));
        assert!(p.ends_with("111001")); // check digit 1 = inverse of 00011, then frame
        assert_eq!(
            encode(PostalKind::Planet, "0123456789012")
                .unwrap()
                .bars
                .len(),
            72
        );
        assert!(encode(PostalKind::Planet, "12345").is_err());
        assert_eq!(PostalKind::from_bz_type(3), Some(PostalKind::UspsImb));
        assert_eq!(PostalKind::from_bz_type(2), None);
    }
}
