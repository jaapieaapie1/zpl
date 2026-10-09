//! Field modifiers, stored objects, page settings and the less common barcodes.

use zpl_tokenizer::Span;

use super::Parser;
use crate::{
    command::{Command, FieldDirection, LinearKind, LineColor, MsiCheck, Orientation},
    error::ParseError,
};

/// Parameter `i`, or "" when absent.
fn param<'p>(params: &[&'p str], i: usize) -> &'p str {
    params.get(i).copied().unwrap_or("").trim()
}

fn first_char(params: &[&str], i: usize) -> Option<char> {
    param(params, i).chars().next()
}

impl Parser<'_> {
    fn opt_orientation(&self, params: &[&str], i: usize, span: Span) -> Result<Option<Orientation>, ParseError> {
        match param(params, i) {
            "" => Ok(None),
            s => self.parse_orientation(s, span).map(Some),
        }
    }

    fn yes_no_or(&self, params: &[&str], i: usize, default: bool, span: Span) -> Result<bool, ParseError> {
        match param(params, i) {
            "" => Ok(default),
            s => self.parse_yes_no(s, span),
        }
    }

    fn u32_or(&self, params: &[&str], i: usize, default: u32, span: Span) -> Result<u32, ParseError> {
        Ok(self.parse_optional_u32(params, i, span)?.unwrap_or(default))
    }

    pub(super) fn parse_extra(&self, tilde: bool, name: &str, params: &[&str], span: Span) -> Option<Result<Command, ParseError>> {
        Some(match (tilde, name) {
            (false, "FH") => Ok(Command::FieldHexIndicator { indicator: first_char(params, 0).unwrap_or('_') }),
            (false, "FW") => self.parse_field_orientation(params, span),
            (false, "FP") => self.parse_field_parameter(params, span),
            (false, "TB") => self.parse_text_block(params, span),
            (false, "FN") => Ok(Command::FieldNumber {
                number: param(params, 0).chars().take_while(char::is_ascii_digit).collect::<String>().parse().unwrap_or(0),
            }),
            (false, "SN") => Ok(Command::SerializationData { start: param(params, 0).to_string() }),
            (false, "DF") => Ok(Command::DownloadFormat { name: param(params, 0).to_string() }),
            (false, "XF") => Ok(Command::RecallFormat { name: param(params, 0).to_string() }),
            (true, "DG") => self.parse_download_graphic(params, span),
            (true, "DY") => return self.parse_download_object(params, span),
            (false, "XG") => Ok(Command::RecallGraphic {
                name: param(params, 0).to_string(),
                magnification_x: self.u32_or(params, 1, 1, span).ok()?,
                magnification_y: self.u32_or(params, 2, 1, span).ok()?,
            }),
            (false, "IM") => Ok(Command::RecallGraphic { name: param(params, 0).to_string(), magnification_x: 1, magnification_y: 1 }),
            (false, "LR") => self.yes_no_or(params, 0, false, span).map(|enabled| Command::LabelReverse { enabled }),
            (false, "PM") => self.yes_no_or(params, 0, false, span).map(|enabled| Command::PrintMirror { enabled }),
            (false, "PO") => Ok(Command::PrintOrientation { inverted: param(params, 0).eq_ignore_ascii_case("I") }),
            (false, "LS") => Ok(Command::LabelShift {
                shift: match param(params, 0) {
                    "" => 0,
                    s => self.parse_i32(s, span).ok()?,
                },
            }),
            (false, "GE") => self.parse_graphic_ellipse(params, span),
            (false, "GS") => self.parse_graphic_symbol(params, span),
            (false, "BX") => self.parse_data_matrix(params, span),
            (false, "B7") => self.parse_pdf417(params, span),
            (false, "BO" | "B0") => self.parse_aztec(params, span),
            // ^BLo,h,g: LOGMARS is Code 39 with a mandatory check digit.
            (false, "BL") => Ok(Command::LinearBarcode {
                kind: LinearKind::Code39 { check_digit: true },
                orientation: self.opt_orientation(params, 0, span).ok()?,
                height: self.parse_optional_u32(params, 1, span).ok()?,
                print_interpretation_line: true,
                print_above: self.yes_no_or(params, 2, false, span).ok()?,
            }),
            (false, name) => return self.parse_linear(name, params, span),
            _ => return None,
        })
    }

    fn parse_field_orientation(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^FWr,z
        Ok(Command::FieldOrientation {
            orientation: self.opt_orientation(params, 0, span)?.unwrap_or(Orientation::Normal),
            justification: self.parse_justification(param(params, 1), span)?,
        })
    }

    fn parse_field_parameter(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^FPd,g
        let direction = match param(params, 0).to_ascii_uppercase().as_str() {
            "V" => FieldDirection::Vertical,
            "R" => FieldDirection::Reverse,
            _ => FieldDirection::Horizontal,
        };
        Ok(Command::FieldParameter { direction, char_gap: self.u32_or(params, 1, 0, span)? })
    }

    fn parse_text_block(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^TBa,b,c
        Ok(Command::TextBlock {
            orientation: self.opt_orientation(params, 0, span)?,
            width: self.u32_or(params, 1, 0, span)?,
            height: self.u32_or(params, 2, 0, span)?,
        })
    }

    fn parse_graphic_ellipse(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^GEw,h,t,c
        let color = match param(params, 3) {
            "" => LineColor::Black,
            s => self.parse_line_color(s, span)?,
        };
        Ok(Command::GraphicEllipse {
            width: self.u32_or(params, 0, 3, span)?,
            height: self.u32_or(params, 1, 3, span)?,
            thickness: self.u32_or(params, 2, 1, span)?,
            color,
        })
    }

    fn parse_graphic_symbol(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^GSo,h,w
        Ok(Command::GraphicSymbol {
            orientation: self.opt_orientation(params, 0, span)?,
            height: self.parse_optional_u32(params, 1, span)?,
            width: self.parse_optional_u32(params, 2, span)?,
        })
    }

    /// Decodes ~DG / ~DY bitmap data: Z64/B64, or (compressed) ASCII hex.
    fn decode_graphic(&self, raw: &str, bytes_per_row: u32) -> Result<Vec<u8>, ParseError> {
        let raw = raw.trim();
        if raw.starts_with(":Z64:") || raw.starts_with(":B64:") {
            let compact: String = raw.chars().filter(|c| !c.is_whitespace()).collect();
            self.decode_z64_data(&compact)
        } else {
            Ok(self.decode_hex_data(raw, bytes_per_row))
        }
    }

    fn parse_download_graphic(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ~DGd:o.x,t,w,data
        let bytes_per_row = self.u32_or(params, 2, 0, span)?;
        let raw = params.get(3..).map(|rest| rest.join(",")).unwrap_or_default();
        Ok(Command::DownloadGraphic {
            name: param(params, 0).to_string(),
            bytes_per_row,
            data: self.decode_graphic(&raw, bytes_per_row)?,
        })
    }

    /// ~DYd:f,b,x,t,w,data. Only GRF bitmaps (`x` = G) can be drawn; other objects (fonts,
    /// PNG, ...) are ignored.
    fn parse_download_object(&self, params: &[&str], span: Span) -> Option<Result<Command, ParseError>> {
        if !param(params, 2).eq_ignore_ascii_case("G") {
            return Some(Ok(Command::Comment { text: String::new() }));
        }
        let bytes_per_row = match self.u32_or(params, 4, 0, span) {
            Ok(v) => v,
            Err(e) => return Some(Err(e)),
        };
        let raw = params.get(5..).map(|rest| rest.join(",")).unwrap_or_default();
        let data = if param(params, 1).eq_ignore_ascii_case("A") || raw.trim_start().starts_with(':') {
            self.decode_graphic(&raw, bytes_per_row)
        } else {
            Ok(raw.into_bytes())
        };
        Some(data.map(|data| Command::DownloadGraphic { name: param(params, 0).to_string(), bytes_per_row, data }))
    }

    fn parse_data_matrix(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^BXo,h,s,c,r,f,g,a
        Ok(Command::DataMatrix {
            orientation: self.opt_orientation(params, 0, span)?,
            module_height: self.parse_optional_u32(params, 1, span)?.filter(|&h| h > 0),
            columns: self.u32_or(params, 3, 0, span)?,
            rows: self.u32_or(params, 4, 0, span)?,
            // ECC 200 always honours the escape character (default `~`).
            escape: Some(first_char(params, 6).unwrap_or('~')),
            rectangular: param(params, 7) == "2",
        })
    }

    fn parse_pdf417(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^B7o,h,s,c,r,t
        Ok(Command::Pdf417 {
            orientation: self.opt_orientation(params, 0, span)?,
            row_height: self.parse_optional_u32(params, 1, span)?.filter(|&h| h > 0),
            security: self.parse_optional_u32(params, 2, span)?.map(|s| s.min(8) as u8),
            columns: self.u32_or(params, 3, 0, span)?,
            rows: self.u32_or(params, 4, 0, span)?,
            truncated: self.yes_no_or(params, 5, false, span)?,
        })
    }

    fn parse_aztec(&self, params: &[&str], span: Span) -> Result<Command, ParseError> {
        // ^BOa,b,c,d,e,f,g
        Ok(Command::Aztec {
            orientation: self.opt_orientation(params, 0, span)?,
            magnification: self.parse_optional_u32(params, 1, span)?,
            ecc: self.u32_or(params, 3, 0, span)?,
        })
    }

    /// The 1D barcodes. Most share `o,h,f,g` (orientation, height, interpretation line,
    /// line above); a few put a check-digit flag first (`o,e,h,f,g`) or last.
    fn parse_linear(&self, name: &str, params: &[&str], span: Span) -> Option<Result<Command, ParseError>> {
        // Parameter index of the height; f and g follow it.
        let check_first = matches!(name, "B1" | "BK" | "BM" | "BP");
        let h = if check_first { 2 } else { 1 };
        let check_last = param(params, h + 3);
        let check_first_param = param(params, 1);
        let yes = |s: &str, default: bool| match s {
            "" => Ok(default),
            s => self.parse_yes_no(s, span),
        };

        let kind = (|| -> Result<Option<LinearKind>, ParseError> {
            Ok(Some(match name {
                "B1" => LinearKind::Code11 { check_digits: if yes(check_first_param, false)? { 1 } else { 2 } },
                "B2" => LinearKind::Interleaved2of5 { check_digit: yes(check_last, false)? },
                "B8" => LinearKind::Ean8,
                "B9" => LinearKind::UpcE { print_check_digit: yes(check_last, true)? },
                "BA" => LinearKind::Code93 { print_check_digit: yes(check_last, false)? },
                "BE" => LinearKind::Ean13,
                "BI" => LinearKind::Industrial2of5,
                "BJ" => LinearKind::Standard2of5,
                "BK" => LinearKind::Codabar {
                    check_digit: yes(check_first_param, false)?,
                    start: first_char(params, 5).unwrap_or('A').to_ascii_uppercase(),
                    stop: first_char(params, 6).unwrap_or('A').to_ascii_uppercase(),
                },
                "BM" => LinearKind::Msi {
                    check: match check_first_param.to_ascii_uppercase().as_str() {
                        "A" => MsiCheck::None,
                        "C" => MsiCheck::Mod10Mod10,
                        "D" => MsiCheck::Mod11Mod10,
                        _ => MsiCheck::Mod10,
                    },
                    print_check_digit: yes(param(params, 5), false)?,
                },
                "BP" => LinearKind::Plessey { print_check_digit: yes(check_first_param, false)? },
                "BS" => LinearKind::UpcEanExtension,
                "BU" => LinearKind::UpcA { print_check_digit: yes(check_last, true)? },
                _ => return Ok(None),
            }))
        })();
        let kind = match kind {
            Ok(Some(kind)) => kind,
            Ok(None) => return None,
            Err(e) => return Some(Err(e)),
        };

        // ^BS prints its digits above the bars by default.
        let above_default = name == "BS";
        Some((|| {
            Ok(Command::LinearBarcode {
                kind,
                orientation: self.opt_orientation(params, 0, span)?,
                height: self.parse_optional_u32(params, h, span)?,
                print_interpretation_line: yes(param(params, h + 1), true)?,
                print_above: yes(param(params, h + 2), above_default)?,
            })
        })())
    }
}
