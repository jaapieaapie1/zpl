//! Splitting the output into printed labels: one page per ^XA...^XZ format, repeated for
//! ^PQ copies, with ^SN / ^SF serialized fields counting up from copy to copy.

use crate::{DrawInstruction, PageSetup};

/// Most pages [`crate::StateManager::process_pages`] returns by default. A printer would
/// happily print `^PQ99999999`; a renderer holding every page in memory cannot.
pub const MAX_PAGES: usize = 1000;

/// One printed label.
#[derive(Debug, Clone, PartialEq)]
pub struct Page {
    pub instructions: Vec<DrawInstruction>,
    pub setup: PageSetup,
}

/// How a serialized field's data changes from one serial number to the next.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Serial {
    /// ^SN: the last run of digits (at most 12) changes by `increment`.
    Number { increment: i64, leading_zeros: bool },
    /// ^SF: `increment` is added to the data under `mask`, both right-aligned to the data.
    Mask { mask: String, increment: String },
}

impl Serial {
    /// The data `step` serial numbers after `base`; step 0 is `base` itself.
    pub(crate) fn value(&self, base: &str, step: u64) -> String {
        if step == 0 {
            return base.to_string();
        }
        match self {
            Serial::Number { increment, leading_zeros } => number(base, *increment, *leading_zeros, step),
            Serial::Mask { mask, increment } => masked(base, mask, increment, step),
        }
    }
}

/// ^SN. With leading zeros the number keeps its width and wraps around like a counter;
/// without, it grows freely (up to 12 digits) and is printed unpadded. Counting below zero
/// wraps around either way.
fn number(base: &str, increment: i64, leading_zeros: bool, step: u64) -> String {
    let bytes = base.as_bytes();
    let Some(last) = bytes.iter().rposition(u8::is_ascii_digit) else { return base.to_string() };
    let end = last + 1;
    let mut start = last;
    while start > 0 && bytes[start - 1].is_ascii_digit() && end - start < 12 {
        start -= 1;
    }
    let width = end - start;
    let value: i128 = base[start..end].parse().unwrap_or(0);
    let next = value + increment as i128 * step as i128;
    let digits = if leading_zeros {
        format!("{:0width$}", next.rem_euclid(10i128.pow(width as u32)))
    } else if next < 0 {
        next.rem_euclid(10i128.pow(width as u32)).to_string()
    } else {
        (next % 10i128.pow(12)).to_string()
    };
    format!("{}{digits}{}", &base[..start], &base[end..])
}

/// Value of `c` at a ^SF mask position, if it is a valid character there.
fn mask_digit(mask: char, c: char) -> Option<u32> {
    match mask {
        'D' => c.to_digit(10),
        'O' => c.to_digit(8),
        'H' => c.to_digit(16),
        'N' => c.to_digit(36),
        // Letters count from A = 0.
        'A' if c.is_ascii_alphabetic() => Some(c.to_ascii_uppercase() as u32 - 'A' as u32),
        _ => None,
    }
}

fn mask_radix(mask: char) -> u32 {
    match mask {
        'D' => 10,
        'O' => 8,
        'H' => 16,
        'A' => 26,
        _ => 36,
    }
}

/// ^SF. Each masked position gains its increment digit times `step`, plus the carry from
/// the right; % positions are skipped and the carry passes over them, so `BL09-9` under
/// `AADD%D` with increment `1%1` becomes `BL11-0`. A carry out of the leftmost masked
/// position is dropped. Characters invalid for their position count as zero.
fn masked(base: &str, mask: &str, increment: &str, step: u64) -> String {
    let mut data: Vec<char> = base.chars().collect();
    let mut increment = increment.chars().rev();
    let mut carry: u128 = 0;
    for (pos, m) in (0..data.len()).rev().zip(mask.chars().rev()) {
        let inc = increment.next().and_then(|c| mask_digit(m, c)).unwrap_or(0);
        if m == '%' {
            continue;
        }
        let c = data[pos];
        let radix = mask_radix(m) as u128;
        let total = mask_digit(m, c).unwrap_or(0) as u128 + inc as u128 * step as u128 + carry;
        let digit = (total % radix) as u32;
        let out = if m == 'A' {
            char::from(b'A' + digit as u8)
        } else {
            char::from_digit(digit, 36).unwrap_or('0').to_ascii_uppercase()
        };
        data[pos] = if c.is_ascii_lowercase() { out.to_ascii_lowercase() } else { out };
        carry = total / radix;
    }
    data.into_iter().collect()
}

/// A serialized field: its instruction's index in the flat output and its first value.
#[derive(Debug)]
struct Slot {
    index: usize,
    base: String,
    serial: Serial,
}

/// The end of a format and how it prints.
#[derive(Debug)]
struct Break {
    end: usize,
    setup: PageSetup,
    quantity: u32,
    replicates: u32,
    slots: Vec<Slot>,
}

/// Recorded while processing: where each format ended, and its serialized fields.
#[derive(Debug, Default)]
pub(crate) struct PageLog {
    breaks: Vec<Break>,
    /// Serialized fields of the format in progress.
    slots: Vec<Slot>,
}

impl PageLog {
    pub(crate) fn serial(&mut self, index: usize, base: String, serial: Serial) {
        self.slots.push(Slot { index, base, serial });
    }

    /// Ends a format whose output runs up to `end`.
    pub(crate) fn page_break(&mut self, end: usize, setup: PageSetup, quantity: u32, replicates: u32) {
        let mut slots = std::mem::take(&mut self.slots);
        // ^IS without printing takes the format's drawing back out.
        slots.retain(|slot| slot.index < end);
        self.breaks.push(Break { end, setup, quantity, replicates, slots });
    }

    /// Cuts `out` and the parallel `extra` (one entry per instruction) into pages, skipping
    /// formats that drew nothing. Returns a blank `fallback` page if nothing was drawn.
    pub(crate) fn paginate<T: Clone>(
        self,
        out: Vec<DrawInstruction>,
        extra: Vec<T>,
        fallback: PageSetup,
        max_pages: usize,
    ) -> Vec<(Page, Vec<T>)> {
        let mut pages: Vec<(Page, Vec<T>)> = Vec::new();
        let mut out = out.into_iter();
        let mut extra = extra.into_iter();
        let mut start = 0;
        for b in self.breaks {
            let len = b.end.saturating_sub(start);
            start = start.max(b.end);
            if len == 0 {
                continue;
            }
            if pages.len() >= max_pages {
                break;
            }
            let instructions: Vec<_> = out.by_ref().take(len).collect();
            let sources: Vec<_> = extra.by_ref().take(len).collect();
            let first = pages.len();
            let copies = (b.quantity as usize).min(max_pages - first);
            pages.push((Page { instructions, setup: b.setup }, sources));

            for copy in 1..copies {
                let (page, sources) = &pages[first];
                let mut page = page.clone();
                let step = (copy / b.replicates as usize) as u64;
                for slot in &b.slots {
                    if let Some(data) = slot
                        .index
                        .checked_sub(b.end - len)
                        .and_then(|i| page.instructions.get_mut(i))
                        .and_then(DrawInstruction::field_data_mut) {
                        *data = slot.serial.value(&slot.base, step);
                    }
                }
                let sources = sources.clone();
                pages.push((page, sources));
            }
        }
        if pages.is_empty() {
            pages.push((Page { instructions: Vec::new(), setup: fallback }, Vec::new()));
        }
        pages
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serial_numbers() {
        let n = |base: &str, inc: i64, zeros: bool, step: u64| Serial::Number { increment: inc, leading_zeros: zeros }.value(base, step);
        assert_eq!(n("A007", 1, true, 0), "A007");
        assert_eq!(n("A007", 1, true, 3), "A010");
        assert_eq!(n("A007", 1, false, 0), "A007");
        assert_eq!(n("A007", 1, false, 3), "A10");
        assert_eq!(n("Box 99 of", 1, false, 1), "Box 100 of");
        assert_eq!(n("999", 1, true, 1), "000");
        assert_eq!(n("010", -5, true, 3), "995");
        assert_eq!(n("10", -3, false, 2), "4");
        assert_eq!(n("1", -3, false, 1), "8");
        assert_eq!(n("12X", 10, false, 2), "32X");
        assert_eq!(n("none", 1, false, 5), "none");
        // Only the last 12 digits count.
        assert_eq!(n("1999999999999", 1, true, 1), "1000000000000");
    }

    #[test]
    fn serial_masks() {
        let m = |base: &str, mask: &str, inc: &str, step: u64| Serial::Mask { mask: mask.into(), increment: inc.into() }.value(base, step);
        assert_eq!(m("BL0000", "AADDDD", "1", 1), "BL0001");
        assert_eq!(m("BL9999", "AADDDD", "1", 1), "BM0000");
        assert_eq!(m("BL00-0", "AADD%D", "1%1", 1), "BL01-1");
        assert_eq!(m("BL00-0", "AADD%D", "1%1", 10), "BL11-0");
        assert_eq!(m("A99", "%DD", "1", 1), "A00");
        assert_eq!(m("0f", "HH", "1", 1), "10");
        assert_eq!(m("0e", "HH", "1", 1), "0f");
        assert_eq!(m("7", "O", "1", 1), "0");
        assert_eq!(m("az", "NN", "1", 1), "b0");
        // B is one for letters; the mask only covers the end of the data.
        assert_eq!(m("xxZ", "A", "B", 1), "xxA");
        assert_eq!(m("12", "DD", "5", 4), "32");
    }
}
