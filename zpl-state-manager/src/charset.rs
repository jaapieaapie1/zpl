//! ^CI character sets for bytes written with ^FH escapes.
//!
//! Field data arrives as Unicode text, so only escaped bytes need decoding; the literal text
//! between escapes is kept as is.

use zpl_parser::command::Encoding;

/// Decodes the bytes of one run of consecutive ^FH escapes.
pub fn decode(bytes: &[u8], encoding: Encoding) -> String {
    use encoding_rs as e;
    let codec = match encoding {
        Encoding::Utf8 => return String::from_utf8_lossy(bytes).into_owned(),
        Encoding::Utf16BigEndian | Encoding::UCS2BigEndian => e::UTF_16BE,
        Encoding::Utf16LittleEndian => e::UTF_16LE,
        Encoding::Cp1252 => e::WINDOWS_1252,
        Encoding::Cp1250 => e::WINDOWS_1250,
        Encoding::Cp1251 => e::WINDOWS_1251,
        Encoding::Cp1253 => e::WINDOWS_1253,
        Encoding::Cp1254 => e::WINDOWS_1254,
        Encoding::Cp1255 => e::WINDOWS_1255,
        Encoding::ShiftJIS => e::SHIFT_JIS,
        Encoding::EUCJPCN => e::EUC_JP,
        Encoding::DoubleByteAsian | Encoding::MultibyteAsian => e::GBK,
        // ^CI0-13 and single-byte Asian: Zebra Code Page 850.
        _ => return oem_cp::decode_string_complete_table(bytes, &oem_cp::code_table::DECODING_TABLE_CP850),
    };
    codec.decode_without_bom_handling(bytes).0.into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_pages() {
        assert_eq!(decode(&[0x82, 0x41], Encoding::Usa1), "éA");
        assert_eq!(decode(&[0xE9], Encoding::Cp1252), "é");
        assert_eq!(decode(&[0xC3, 0xA9], Encoding::Utf8), "é");
        assert_eq!(decode(&[0xC0], Encoding::Cp1251), "А");
        assert_eq!(decode(&[0x00, 0x41], Encoding::Utf16BigEndian), "A");
    }
}
